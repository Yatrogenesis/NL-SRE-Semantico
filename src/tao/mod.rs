//! # TAO Layer
//!
//! Capa de encapsulamiento y paso de mensajes inspirada en TAO (1983, Japón).
//! Permite nesting mutuo entre componentes: PIRS puede llamar a LIRS y viceversa.
//!
//! ## Concepto
//! Cada componente es un "actor" que recibe mensajes y responde.
//! El estado está encapsulado. Solo se comunica via mensajes.

use std::collections::HashMap;
use crate::uniform::UnifyValue;
use crate::applog::SharedContext;

/// Identificador de componente
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Grammar,
    Semantic,
    CharMatcher,
    Disambiguator,
    Custom(String),
}

/// Un mensaje entre componentes
#[derive(Debug, Clone)]
pub struct Message {
    /// Remitente
    pub from: ComponentId,
    /// Destinatario
    pub to: ComponentId,
    /// Tipo de mensaje
    pub msg_type: MessageType,
    /// Payload
    pub payload: MessagePayload,
    /// ID de correlación para request/response
    pub correlation_id: u64,
}

/// Tipos de mensaje
#[derive(Debug, Clone)]
pub enum MessageType {
    /// Solicitud de procesamiento
    Request,
    /// Respuesta a solicitud
    Response,
    /// Notificación (sin esperar respuesta)
    Notify,
    /// Error
    Error,
}

/// Payload del mensaje
#[derive(Debug, Clone)]
pub enum MessagePayload {
    /// Query de gramática
    GrammarQuery {
        sentence: Vec<String>,
        position: usize,
        candidates: Vec<String>,
    },

    /// Resultado de gramática
    GrammarResult {
        scores: Vec<(String, f64)>,
        best: Option<String>,
        structure: Option<GrammaticalStructure>,
    },

    /// Query semántica
    SemanticQuery {
        word: String,
        context_words: Vec<String>,
        theme: Option<String>,
    },

    /// Resultado semántico
    SemanticResult {
        category: Option<String>,
        compatibility: f64,
        related_words: Vec<String>,
    },

    /// Query de caracteres
    CharQuery {
        input: String,
        dictionary: Vec<String>,
    },

    /// Resultado de caracteres
    CharResult {
        candidates: Vec<(String, f64)>,
    },

    /// Solicitud de desambiguación completa
    DisambiguateRequest {
        sentence: String,
    },

    /// Resultado de desambiguación
    DisambiguateResult {
        corrected: String,
        confidence: f64,
        corrections: Vec<(usize, String, String, f64)>,
    },

    /// Valor genérico
    Value(UnifyValue),

    /// Error con mensaje
    ErrorMsg(String),

    /// Vacío (para notificaciones simples)
    Empty,
}

/// Estructura gramatical detectada
#[derive(Debug, Clone)]
pub struct GrammaticalStructure {
    /// Tipo de oración
    pub sentence_type: SentenceType,
    /// Componentes identificados
    pub components: Vec<GrammaticalComponent>,
    /// Tema inferido
    pub inferred_theme: Option<String>,
}

/// Tipo de oración
#[derive(Debug, Clone, PartialEq)]
pub enum SentenceType {
    /// Sujeto-Verbo-Objeto (orden canónico)
    SVO,
    /// Objeto-Verbo-Sujeto
    OVS,
    /// Verbo-Sujeto-Objeto
    VSO,
    /// Solo Sujeto-Verbo (intransitiva)
    SV,
    /// Impersonal
    Impersonal,
    /// No determinado
    Unknown,
}

/// Componente gramatical
#[derive(Debug, Clone)]
pub struct GrammaticalComponent {
    pub role: GrammaticalRole,
    pub tokens: Vec<usize>,  // índices en la oración
    pub head: Option<usize>, // índice del núcleo
}

/// Rol gramatical
#[derive(Debug, Clone, PartialEq)]
pub enum GrammaticalRole {
    Subject,
    Verb,
    DirectObject,
    IndirectObject,
    Complement,
    Adjective,
    Adverb,
    Preposition,
    Article,
    Conjunction,
    Punctuation,
}

/// Bus de mensajes central
#[derive(Debug)]
pub struct MessageBus {
    /// Contexto compartido (APPLOG)
    shared_context: SharedContext,

    /// Handlers registrados
    handlers: HashMap<ComponentId, Box<dyn MessageHandler>>,

    /// Contador de correlation IDs
    next_correlation_id: u64,

    /// Cola de mensajes pendientes (para procesamiento asíncrono futuro)
    #[allow(dead_code)]
    pending: Vec<Message>,
}

/// Trait para componentes que manejan mensajes
pub trait MessageHandler: std::fmt::Debug {
    /// Procesa un mensaje y retorna respuesta
    fn handle(&mut self, msg: &Message, ctx: &mut SharedContext) -> Option<Message>;

    /// ID del componente
    fn component_id(&self) -> ComponentId;
}

impl MessageBus {
    /// Crea nuevo bus
    pub fn new(shared_context: SharedContext) -> Self {
        Self {
            shared_context,
            handlers: HashMap::new(),
            next_correlation_id: 1,
            pending: Vec::new(),
        }
    }

    /// Registra un handler
    pub fn register<H: MessageHandler + 'static>(&mut self, handler: H) {
        let id = handler.component_id();
        self.handlers.insert(id, Box::new(handler));
    }

    /// Envía mensaje y espera respuesta (síncrono)
    pub fn send_sync(&mut self, msg: Message) -> Option<MessagePayload> {
        let to = msg.to.clone();
        let correlation = msg.correlation_id;

        // Buscar handler
        if let Some(handler) = self.handlers.get_mut(&to) {
            if let Some(response) = handler.handle(&msg, &mut self.shared_context) {
                if response.correlation_id == correlation {
                    return Some(response.payload);
                }
            }
        }

        None
    }

    /// Crea mensaje con correlation ID único
    pub fn create_message(
        &mut self,
        from: ComponentId,
        to: ComponentId,
        msg_type: MessageType,
        payload: MessagePayload,
    ) -> Message {
        let id = self.next_correlation_id;
        self.next_correlation_id += 1;

        Message {
            from,
            to,
            msg_type,
            payload,
            correlation_id: id,
        }
    }

    /// Acceso al contexto compartido
    pub fn context(&self) -> &SharedContext {
        &self.shared_context
    }

    /// Acceso mutable al contexto compartido
    pub fn context_mut(&mut self) -> &mut SharedContext {
        &mut self.shared_context
    }

    /// Notifica a todos los componentes
    pub fn broadcast(&mut self, from: ComponentId, payload: MessagePayload) {
        let ids: Vec<_> = self.handlers.keys().cloned().collect();

        for to in ids {
            if to != from {
                let msg = self.create_message(
                    from.clone(),
                    to.clone(),
                    MessageType::Notify,
                    payload.clone(),
                );

                if let Some(handler) = self.handlers.get_mut(&to) {
                    let _ = handler.handle(&msg, &mut self.shared_context);
                }
            }
        }
    }
}

/// Helper para crear queries de gramática
pub fn grammar_query(
    from: ComponentId,
    sentence: Vec<String>,
    position: usize,
    candidates: Vec<String>,
) -> Message {
    Message {
        from,
        to: ComponentId::Grammar,
        msg_type: MessageType::Request,
        payload: MessagePayload::GrammarQuery {
            sentence,
            position,
            candidates,
        },
        correlation_id: 0, // Bus asignará
    }
}

/// Helper para crear queries semánticas
pub fn semantic_query(
    from: ComponentId,
    word: String,
    context_words: Vec<String>,
    theme: Option<String>,
) -> Message {
    Message {
        from,
        to: ComponentId::Semantic,
        msg_type: MessageType::Request,
        payload: MessagePayload::SemanticQuery {
            word,
            context_words,
            theme,
        },
        correlation_id: 0,
    }
}

/// Helper para crear queries de caracteres
pub fn char_query(from: ComponentId, input: String, dictionary: Vec<String>) -> Message {
    Message {
        from,
        to: ComponentId::CharMatcher,
        msg_type: MessageType::Request,
        payload: MessagePayload::CharQuery { input, dictionary },
        correlation_id: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockHandler {
        id: ComponentId,
    }

    impl MessageHandler for MockHandler {
        fn handle(&mut self, msg: &Message, _ctx: &mut SharedContext) -> Option<Message> {
            Some(Message {
                from: self.id.clone(),
                to: msg.from.clone(),
                msg_type: MessageType::Response,
                payload: MessagePayload::Value(UnifyValue::Atom("ok".to_string())),
                correlation_id: msg.correlation_id,
            })
        }

        fn component_id(&self) -> ComponentId {
            self.id.clone()
        }
    }

    #[test]
    fn test_message_bus() {
        let ctx = SharedContext::new();
        let mut bus = MessageBus::new(ctx);

        bus.register(MockHandler {
            id: ComponentId::Grammar,
        });

        let msg = bus.create_message(
            ComponentId::Disambiguator,
            ComponentId::Grammar,
            MessageType::Request,
            MessagePayload::Empty,
        );

        let response = bus.send_sync(msg);
        assert!(response.is_some());
    }
}
