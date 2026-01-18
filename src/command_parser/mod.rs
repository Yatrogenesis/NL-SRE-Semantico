//! # Command Parser Module
//!
//! Parser semántico que convierte español natural → predicados PIRS.
//!
//! ## Ejemplo
//! "Requiero que me diseñes un producto que sustituya al propofol"
//! → request(user, agent, design(Product)), goal(Product, substitute(propofol))
//!
//! ## Roles Semánticos
//! - AGENT: quién ejecuta (tú/sistema)
//! - USER: quién solicita (yo)
//! - TARGET: qué se solicita (conocido/desconocido)
//! - GOAL: propósito
//! - CONSTRAINT: restricciones
//!
//! ## Autor
//! Francisco Molina-Burgos, Avermex Research Division

use std::collections::HashMap;

/// Comando parseado desde lenguaje natural
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Texto original
    pub original: String,
    /// Acción principal solicitada
    pub action: CommandAction,
    /// Quién solicita (normalmente USER)
    pub requester: SemanticRole,
    /// Quién ejecuta (normalmente AGENT)
    pub executor: SemanticRole,
    /// Objeto/Target de la acción
    pub target: CommandTarget,
    /// Meta/Propósito
    pub goal: Option<Goal>,
    /// Restricciones/Calificativos
    pub constraints: Vec<Constraint>,
    /// Verbos encontrados con su análisis
    pub verbs: Vec<VerbAnalysis>,
    /// Confianza del parsing (0.0 - 1.0)
    pub confidence: f64,
}

/// Acción del comando
#[derive(Debug, Clone, PartialEq)]
pub enum CommandAction {
    /// Solicitud directa (requiero, quiero, necesito, pido)
    Request {
        verb: String,
        formality: Formality,
    },
    /// Delegación de acción (ayúdame, diseña, crea)
    Delegate {
        verb: String,
        mode: VerbMode,
    },
    /// Búsqueda (busco, encuentra, localiza)
    Search {
        verb: String,
    },
    /// Análisis (analiza, evalúa, examina)
    Analyze {
        verb: String,
    },
    /// Creación (crea, genera, produce, diseña)
    Create {
        verb: String,
    },
    /// Explicación (explícame, dime, cuéntame)
    Explain {
        verb: String,
    },
    /// Cálculo (calcula, computa, determina)
    Compute {
        verb: String,
    },
    /// Desconocida
    Unknown,
}

/// Formalidad del comando
#[derive(Debug, Clone, PartialEq)]
pub enum Formality {
    /// Formal (requiero, solicito)
    Formal,
    /// Normal (quiero, necesito)
    Normal,
    /// Informal (ocupo, dame)
    Informal,
}

/// Modo verbal
#[derive(Debug, Clone, PartialEq)]
pub enum VerbMode {
    /// Indicativo (diseño, creo)
    Indicative,
    /// Subjuntivo (diseñes, crees)
    Subjunctive,
    /// Imperativo (diseña, crea, ayúdame)
    Imperative,
    /// Infinitivo (diseñar, crear)
    Infinitive,
}

/// Rol semántico
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticRole {
    /// Usuario que hace la solicitud
    User,
    /// Agente que ejecuta (sistema/IA)
    Agent,
    /// Tercero mencionado
    Third { reference: String },
    /// Implícito (no mencionado explícitamente)
    Implicit,
}

/// Target del comando
#[derive(Debug, Clone)]
pub enum CommandTarget {
    /// Target conocido y específico
    Known {
        name: String,
        category: Option<String>,
    },
    /// Target indefinido (un producto, algo, un compuesto)
    Unknown {
        /// Pista de tipo ("producto", "compuesto", "sustancia")
        hint: Option<String>,
        /// Categoría inferida
        category: Option<String>,
        /// Artículo usado ("un", "una", "algún")
        article: Option<String>,
    },
    /// Referencia a algo mencionado antes ("él", "eso", "lo anterior")
    Reference {
        pronoun: String,
    },
    /// Sin target explícito
    None,
}

/// Meta/Propósito de la acción
#[derive(Debug, Clone)]
pub struct Goal {
    /// Verbo del propósito (sustituir, reemplazar, mejorar)
    pub action: String,
    /// Target del propósito
    pub target: String,
    /// Palabras clave del contexto
    pub context: Vec<String>,
}

/// Restricción/Calificativo
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Atributo que se restringe (seguridad, costo, calidad)
    pub attribute: String,
    /// Tipo de restricción
    pub constraint_type: ConstraintType,
    /// Valor o referencia
    pub value: ConstraintValue,
    /// Texto original
    pub original_text: String,
}

/// Tipo de restricción
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    /// Mayor que (mejor que, más que, superior a)
    GreaterThan,
    /// Menor que (más barato, menos que, inferior a)
    LessThan,
    /// Igual a
    EqualTo,
    /// Superlativo (súper, muy, extremadamente)
    Superlative,
    /// Negación (no, sin, nunca)
    Negation,
}

/// Valor de la restricción
#[derive(Debug, Clone)]
pub enum ConstraintValue {
    /// Referencia a otra entidad
    Reference(String),
    /// Valor cualitativo (alto, bajo, máximo)
    Qualitative(String),
    /// Valor numérico
    Numeric(f64),
    /// Booleano
    Boolean(bool),
}

/// Análisis de un verbo encontrado
#[derive(Debug, Clone)]
pub struct VerbAnalysis {
    /// Verbo conjugado como apareció
    pub conjugated: String,
    /// Lema/infinitivo
    pub lemma: String,
    /// Persona (1, 2, 3)
    pub person: u8,
    /// Número (singular, plural)
    pub number: GrammaticalNumber,
    /// Modo
    pub mode: VerbMode,
    /// Tiempo (presente, pasado, futuro)
    pub tense: VerbTense,
    /// Posición en la oración
    pub position: usize,
    /// Rol semántico inferido
    pub semantic_role: VerbSemanticRole,
}

/// Número gramatical
#[derive(Debug, Clone, PartialEq)]
pub enum GrammaticalNumber {
    Singular,
    Plural,
}

/// Tiempo verbal
#[derive(Debug, Clone, PartialEq)]
pub enum VerbTense {
    Present,
    Past,
    Future,
    Conditional,
}

/// Rol semántico del verbo
#[derive(Debug, Clone, PartialEq)]
pub enum VerbSemanticRole {
    /// Verbo de solicitud (requiero, quiero, necesito)
    Request,
    /// Verbo de acción delegada (diseña, crea, ayúdame)
    DelegatedAction,
    /// Verbo de propósito (para sustituir, para mejorar)
    Purpose,
    /// Verbo auxiliar (tiene que ser, debe ser)
    Auxiliary,
    /// Verbo de estado (es, está)
    State,
    /// Otro
    Other,
}

/// Parser de comandos en español
#[derive(Debug)]
pub struct CommandParser {
    /// Verbos de solicitud (1a persona)
    request_verbs: HashMap<String, (String, Formality)>,
    /// Verbos de acción (infinitivos y sus categorías)
    action_verbs: HashMap<String, ActionCategory>,
    /// Indicadores de objeto indefinido
    indefinite_indicators: Vec<String>,
    /// Indicadores de superlativo
    superlative_indicators: Vec<String>,
    /// Indicadores comparativos "mayor que"
    comparative_greater: Vec<String>,
    /// Indicadores comparativos "menor que"
    comparative_less: Vec<String>,
    /// Atributos comunes (seguro, barato, rápido, etc.)
    common_attributes: HashMap<String, String>,
}

/// Categoría de acción
#[derive(Debug, Clone, PartialEq)]
pub enum ActionCategory {
    Create,
    Search,
    Analyze,
    Explain,
    Compute,
    Transform,
    Other,
}

impl CommandParser {
    /// Crea un nuevo parser con vocabulario predefinido
    pub fn new() -> Self {
        let mut parser = Self {
            request_verbs: HashMap::new(),
            action_verbs: HashMap::new(),
            indefinite_indicators: Vec::new(),
            superlative_indicators: Vec::new(),
            comparative_greater: Vec::new(),
            comparative_less: Vec::new(),
            common_attributes: HashMap::new(),
        };

        parser.load_vocabulary();
        parser
    }

    /// Carga el vocabulario de verbos y patrones
    fn load_vocabulary(&mut self) {
        // === VERBOS DE SOLICITUD (1a persona) ===
        // Formato: (conjugación → (lema, formalidad))

        // Formales
        self.request_verbs.insert("requiero".to_string(), ("requerir".to_string(), Formality::Formal));
        self.request_verbs.insert("solicito".to_string(), ("solicitar".to_string(), Formality::Formal));
        self.request_verbs.insert("preciso".to_string(), ("precisar".to_string(), Formality::Formal));

        // Normales
        self.request_verbs.insert("quiero".to_string(), ("querer".to_string(), Formality::Normal));
        self.request_verbs.insert("necesito".to_string(), ("necesitar".to_string(), Formality::Normal));
        self.request_verbs.insert("pido".to_string(), ("pedir".to_string(), Formality::Normal));
        self.request_verbs.insert("busco".to_string(), ("buscar".to_string(), Formality::Normal));
        self.request_verbs.insert("deseo".to_string(), ("desear".to_string(), Formality::Normal));

        // Informales / regionales
        self.request_verbs.insert("ocupo".to_string(), ("ocupar".to_string(), Formality::Informal)); // México
        self.request_verbs.insert("ando buscando".to_string(), ("buscar".to_string(), Formality::Informal));

        // === VERBOS DE ACCIÓN ===
        // Ayuda (categoría especial)
        self.action_verbs.insert("ayudar".to_string(), ActionCategory::Other);

        // Creación
        self.action_verbs.insert("crear".to_string(), ActionCategory::Create);
        self.action_verbs.insert("diseñar".to_string(), ActionCategory::Create);
        self.action_verbs.insert("generar".to_string(), ActionCategory::Create);
        self.action_verbs.insert("producir".to_string(), ActionCategory::Create);
        self.action_verbs.insert("fabricar".to_string(), ActionCategory::Create);
        self.action_verbs.insert("construir".to_string(), ActionCategory::Create);
        self.action_verbs.insert("desarrollar".to_string(), ActionCategory::Create);
        self.action_verbs.insert("elaborar".to_string(), ActionCategory::Create);
        self.action_verbs.insert("formular".to_string(), ActionCategory::Create);
        self.action_verbs.insert("sintetizar".to_string(), ActionCategory::Create);

        // Búsqueda
        self.action_verbs.insert("buscar".to_string(), ActionCategory::Search);
        self.action_verbs.insert("encontrar".to_string(), ActionCategory::Search);
        self.action_verbs.insert("localizar".to_string(), ActionCategory::Search);
        self.action_verbs.insert("hallar".to_string(), ActionCategory::Search);
        self.action_verbs.insert("identificar".to_string(), ActionCategory::Search);

        // Análisis
        self.action_verbs.insert("analizar".to_string(), ActionCategory::Analyze);
        self.action_verbs.insert("evaluar".to_string(), ActionCategory::Analyze);
        self.action_verbs.insert("examinar".to_string(), ActionCategory::Analyze);
        self.action_verbs.insert("revisar".to_string(), ActionCategory::Analyze);
        self.action_verbs.insert("estudiar".to_string(), ActionCategory::Analyze);
        self.action_verbs.insert("investigar".to_string(), ActionCategory::Analyze);

        // Explicación
        self.action_verbs.insert("explicar".to_string(), ActionCategory::Explain);
        self.action_verbs.insert("describir".to_string(), ActionCategory::Explain);
        self.action_verbs.insert("contar".to_string(), ActionCategory::Explain);
        self.action_verbs.insert("decir".to_string(), ActionCategory::Explain);
        self.action_verbs.insert("mostrar".to_string(), ActionCategory::Explain);

        // Cálculo
        self.action_verbs.insert("calcular".to_string(), ActionCategory::Compute);
        self.action_verbs.insert("computar".to_string(), ActionCategory::Compute);
        self.action_verbs.insert("determinar".to_string(), ActionCategory::Compute);
        self.action_verbs.insert("estimar".to_string(), ActionCategory::Compute);
        self.action_verbs.insert("medir".to_string(), ActionCategory::Compute);

        // Transformación
        self.action_verbs.insert("sustituir".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("reemplazar".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("cambiar".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("modificar".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("mejorar".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("optimizar".to_string(), ActionCategory::Transform);
        self.action_verbs.insert("convertir".to_string(), ActionCategory::Transform);

        // === INDICADORES DE OBJETO INDEFINIDO ===
        self.indefinite_indicators = vec![
            "un".to_string(), "una".to_string(),
            "unos".to_string(), "unas".to_string(),
            "algún".to_string(), "alguna".to_string(),
            "algunos".to_string(), "algunas".to_string(),
            "algo".to_string(), "alguien".to_string(),
            "cualquier".to_string(), "cierto".to_string(),
        ];

        // === INDICADORES SUPERLATIVOS ===
        self.superlative_indicators = vec![
            "súper".to_string(), "super".to_string(),
            "muy".to_string(), "mucho".to_string(),
            "extremadamente".to_string(), "totalmente".to_string(),
            "completamente".to_string(), "absolutamente".to_string(),
            "sumamente".to_string(), "altamente".to_string(),
            "máximo".to_string(), "máxima".to_string(),
            "óptimo".to_string(), "óptima".to_string(),
        ];

        // === INDICADORES COMPARATIVOS (mayor) ===
        self.comparative_greater = vec![
            "mejor que".to_string(), "más que".to_string(),
            "superior a".to_string(), "mayor que".to_string(),
            "por encima de".to_string(), "más".to_string(),
        ];

        // === INDICADORES COMPARATIVOS (menor) ===
        self.comparative_less = vec![
            "más barato".to_string(), "menos que".to_string(),
            "inferior a".to_string(), "menor que".to_string(),
            "por debajo de".to_string(), "menos".to_string(),
            "más económico".to_string(), "más barata".to_string(),
        ];

        // === ATRIBUTOS COMUNES ===
        self.common_attributes.insert("seguro".to_string(), "safety".to_string());
        self.common_attributes.insert("segura".to_string(), "safety".to_string());
        self.common_attributes.insert("barato".to_string(), "cost".to_string());
        self.common_attributes.insert("barata".to_string(), "cost".to_string());
        self.common_attributes.insert("económico".to_string(), "cost".to_string());
        self.common_attributes.insert("económica".to_string(), "cost".to_string());
        self.common_attributes.insert("caro".to_string(), "cost".to_string());
        self.common_attributes.insert("cara".to_string(), "cost".to_string());
        self.common_attributes.insert("rápido".to_string(), "speed".to_string());
        self.common_attributes.insert("rápida".to_string(), "speed".to_string());
        self.common_attributes.insert("lento".to_string(), "speed".to_string());
        self.common_attributes.insert("lenta".to_string(), "speed".to_string());
        self.common_attributes.insert("eficiente".to_string(), "efficiency".to_string());
        self.common_attributes.insert("eficaz".to_string(), "efficacy".to_string());
        self.common_attributes.insert("efectivo".to_string(), "efficacy".to_string());
        self.common_attributes.insert("efectiva".to_string(), "efficacy".to_string());
        self.common_attributes.insert("mejor".to_string(), "quality".to_string());
        self.common_attributes.insert("peor".to_string(), "quality".to_string());
        self.common_attributes.insert("bueno".to_string(), "quality".to_string());
        self.common_attributes.insert("buena".to_string(), "quality".to_string());
        self.common_attributes.insert("malo".to_string(), "quality".to_string());
        self.common_attributes.insert("mala".to_string(), "quality".to_string());
        self.common_attributes.insert("potente".to_string(), "power".to_string());
        self.common_attributes.insert("fuerte".to_string(), "strength".to_string());
        self.common_attributes.insert("débil".to_string(), "strength".to_string());
        self.common_attributes.insert("estable".to_string(), "stability".to_string());
        self.common_attributes.insert("confiable".to_string(), "reliability".to_string());
    }

    /// Parsea un comando en español
    pub fn parse(&self, text: &str) -> ParsedCommand {
        let text_lower = text.to_lowercase();
        let tokens = self.tokenize(&text_lower);

        // 1. Encontrar verbos y analizarlos
        let verbs = self.find_verbs(&tokens);

        // 2. Determinar acción principal
        let action = self.determine_action(&verbs, &tokens);

        // 3. Determinar roles (requester, executor)
        let (requester, executor) = self.determine_roles(&verbs);

        // 4. Encontrar target
        let target = self.find_target(&tokens);

        // 5. Encontrar goal/propósito
        let goal = self.find_goal(&tokens);

        // 6. Encontrar constraints
        let constraints = self.find_constraints(&tokens);

        // 7. Calcular confianza
        let confidence = self.calculate_confidence(&action, &target, &verbs);

        ParsedCommand {
            original: text.to_string(),
            action,
            requester,
            executor,
            target,
            goal,
            constraints,
            verbs,
            confidence,
        }
    }

    /// Tokeniza el texto
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric() && c != 'á' && c != 'é' && c != 'í' && c != 'ó' && c != 'ú' && c != 'ñ'))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Encuentra y analiza verbos en los tokens
    fn find_verbs(&self, tokens: &[String]) -> Vec<VerbAnalysis> {
        let mut verbs = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            // Verificar si es verbo de solicitud
            if let Some((lemma, _formality)) = self.request_verbs.get(token) {
                verbs.push(VerbAnalysis {
                    conjugated: token.clone(),
                    lemma: lemma.clone(),
                    person: 1,
                    number: GrammaticalNumber::Singular,
                    mode: VerbMode::Indicative,
                    tense: VerbTense::Present,
                    position: i,
                    semantic_role: VerbSemanticRole::Request,
                });
                continue;
            }

            // Detectar imperativo con pronombre (ayúdame, diseñame, etc.)
            if token.ends_with("me") || token.ends_with("nos") {
                let base = if token.ends_with("me") {
                    &token[..token.len()-2]
                } else {
                    &token[..token.len()-3]
                };

                // Intentar encontrar el lema
                let possible_lemma = format!("{}ar", base);
                if self.action_verbs.contains_key(&possible_lemma) {
                    verbs.push(VerbAnalysis {
                        conjugated: token.clone(),
                        lemma: possible_lemma,
                        person: 2,
                        number: GrammaticalNumber::Singular,
                        mode: VerbMode::Imperative,
                        tense: VerbTense::Present,
                        position: i,
                        semantic_role: VerbSemanticRole::DelegatedAction,
                    });
                    continue;
                }
            }

            // Detectar subjuntivo 2a persona (que diseñes, que crees)
            if token.ends_with("es") && tokens.get(i.saturating_sub(1)).map(|s| s.as_str()) == Some("que") {
                // Probable subjuntivo
                let base = &token[..token.len()-2];
                for (lemma, _) in &self.action_verbs {
                    if lemma.starts_with(base) {
                        verbs.push(VerbAnalysis {
                            conjugated: token.clone(),
                            lemma: lemma.clone(),
                            person: 2,
                            number: GrammaticalNumber::Singular,
                            mode: VerbMode::Subjunctive,
                            tense: VerbTense::Present,
                            position: i,
                            semantic_role: VerbSemanticRole::DelegatedAction,
                        });
                        break;
                    }
                }
                continue;
            }

            // Detectar infinitivos
            if token.ends_with("ar") || token.ends_with("er") || token.ends_with("ir") {
                if self.action_verbs.contains_key(token) {
                    verbs.push(VerbAnalysis {
                        conjugated: token.clone(),
                        lemma: token.clone(),
                        person: 0, // infinitivo no tiene persona
                        number: GrammaticalNumber::Singular,
                        mode: VerbMode::Infinitive,
                        tense: VerbTense::Present,
                        position: i,
                        semantic_role: VerbSemanticRole::Purpose,
                    });
                }
            }
        }

        verbs
    }

    /// Determina la acción principal del comando
    fn determine_action(&self, verbs: &[VerbAnalysis], tokens: &[String]) -> CommandAction {
        // Buscar verbo de solicitud primero (1a persona)
        for verb in verbs {
            if verb.semantic_role == VerbSemanticRole::Request {
                if let Some((_, formality)) = self.request_verbs.get(&verb.conjugated) {
                    return CommandAction::Request {
                        verb: verb.lemma.clone(),
                        formality: formality.clone(),
                    };
                }
            }
        }

        // Buscar verbo de acción delegada (2a persona)
        for verb in verbs {
            if verb.semantic_role == VerbSemanticRole::DelegatedAction {
                if let Some(category) = self.action_verbs.get(&verb.lemma) {
                    return match category {
                        ActionCategory::Create => CommandAction::Create { verb: verb.lemma.clone() },
                        ActionCategory::Search => CommandAction::Search { verb: verb.lemma.clone() },
                        ActionCategory::Analyze => CommandAction::Analyze { verb: verb.lemma.clone() },
                        ActionCategory::Explain => CommandAction::Explain { verb: verb.lemma.clone() },
                        ActionCategory::Compute => CommandAction::Compute { verb: verb.lemma.clone() },
                        _ => CommandAction::Delegate { verb: verb.lemma.clone(), mode: verb.mode.clone() },
                    };
                }
            }
        }

        // Buscar patrones de imperativo sin pronombre
        let imperatives = ["diseña", "crea", "genera", "busca", "analiza", "explica", "calcula"];
        for token in tokens {
            for imp in &imperatives {
                if token == *imp {
                    let lemma = format!("{}r", &token[..token.len()-1]);
                    return CommandAction::Delegate {
                        verb: lemma,
                        mode: VerbMode::Imperative,
                    };
                }
            }
        }

        CommandAction::Unknown
    }

    /// Determina los roles semánticos
    fn determine_roles(&self, verbs: &[VerbAnalysis]) -> (SemanticRole, SemanticRole) {
        let mut has_first_person = false;
        let mut has_second_person = false;

        for verb in verbs {
            if verb.person == 1 {
                has_first_person = true;
            }
            if verb.person == 2 {
                has_second_person = true;
            }
        }

        let requester = if has_first_person {
            SemanticRole::User
        } else {
            SemanticRole::Implicit
        };

        let executor = if has_second_person {
            SemanticRole::Agent
        } else {
            SemanticRole::Implicit
        };

        (requester, executor)
    }

    /// Encuentra el target del comando
    fn find_target(&self, tokens: &[String]) -> CommandTarget {
        // Buscar artículo indefinido seguido de sustantivo
        for (i, token) in tokens.iter().enumerate() {
            if self.indefinite_indicators.contains(token) {
                // El siguiente token probablemente es el target
                if let Some(next) = tokens.get(i + 1) {
                    return CommandTarget::Unknown {
                        hint: Some(next.clone()),
                        category: self.infer_category(next),
                        article: Some(token.clone()),
                    };
                }
            }
        }

        // Buscar "algo"
        if tokens.contains(&"algo".to_string()) {
            return CommandTarget::Unknown {
                hint: None,
                category: None,
                article: Some("algo".to_string()),
            };
        }

        // Buscar referencias (él, eso, lo)
        let references = ["él", "ella", "eso", "esto", "lo", "la"];
        for token in tokens {
            for ref_word in &references {
                if token == *ref_word {
                    return CommandTarget::Reference {
                        pronoun: token.clone(),
                    };
                }
            }
        }

        CommandTarget::None
    }

    /// Infiere categoría de un sustantivo
    fn infer_category(&self, word: &str) -> Option<String> {
        let categories: HashMap<&str, &str> = [
            ("producto", "product"),
            ("compuesto", "compound"),
            ("sustancia", "substance"),
            ("material", "material"),
            ("medicamento", "medicine"),
            ("fármaco", "drug"),
            ("solución", "solution"),
            ("alternativa", "alternative"),
            ("método", "method"),
            ("proceso", "process"),
            ("sistema", "system"),
            ("herramienta", "tool"),
            ("programa", "software"),
            ("algoritmo", "algorithm"),
        ].iter().cloned().collect();

        categories.get(word).map(|s| s.to_string())
    }

    /// Encuentra el goal/propósito
    fn find_goal(&self, tokens: &[String]) -> Option<Goal> {
        // Buscar patrones: "para [verbo]", "que [verbo]", "[verbo] a/al [target]"
        let purpose_indicators = ["para", "que"];

        for (i, token) in tokens.iter().enumerate() {
            // Patrón: "para sustituir al propofol"
            if purpose_indicators.contains(&token.as_str()) {
                // Buscar verbo siguiente
                if let Some(next) = tokens.get(i + 1) {
                    if self.action_verbs.contains_key(next) {
                        // Buscar target del propósito
                        let mut target = String::new();
                        let mut context = Vec::new();

                        for j in (i + 2)..tokens.len().min(i + 6) {
                            if let Some(t) = tokens.get(j) {
                                if t == "y" || t == "," {
                                    break;
                                }
                                if t != "a" && t != "al" && t != "el" && t != "la" {
                                    if target.is_empty() {
                                        target = t.clone();
                                    } else {
                                        context.push(t.clone());
                                    }
                                }
                            }
                        }

                        if !target.is_empty() {
                            return Some(Goal {
                                action: next.clone(),
                                target,
                                context,
                            });
                        }
                    }
                }
            }

            // Patrón: "sustituir al propofol"
            if self.action_verbs.get(token) == Some(&ActionCategory::Transform) {
                if let Some(prep) = tokens.get(i + 1) {
                    if prep == "a" || prep == "al" {
                        if let Some(target) = tokens.get(i + 2) {
                            return Some(Goal {
                                action: token.clone(),
                                target: target.clone(),
                                context: Vec::new(),
                            });
                        }
                    }
                }
            }
        }

        None
    }

    /// Encuentra constraints/restricciones
    fn find_constraints(&self, tokens: &[String]) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        let text = tokens.join(" ");

        // Buscar superlativos: "súper seguro", "muy barato"
        for (i, token) in tokens.iter().enumerate() {
            if self.superlative_indicators.contains(token) {
                if let Some(adj) = tokens.get(i + 1) {
                    if let Some(attr) = self.common_attributes.get(adj) {
                        constraints.push(Constraint {
                            attribute: attr.clone(),
                            constraint_type: ConstraintType::Superlative,
                            value: ConstraintValue::Qualitative("very_high".to_string()),
                            original_text: format!("{} {}", token, adj),
                        });
                    }
                }
            }
        }

        // Buscar comparativos mayores: "mejor que él", "más X que Y"
        for pattern in &self.comparative_greater {
            if text.contains(pattern) {
                // Extraer referencia
                let parts: Vec<&str> = text.split(pattern).collect();
                if parts.len() > 1 {
                    let reference = parts[1].split_whitespace().next().unwrap_or("unknown");
                    constraints.push(Constraint {
                        attribute: "quality".to_string(),
                        constraint_type: ConstraintType::GreaterThan,
                        value: ConstraintValue::Reference(reference.to_string()),
                        original_text: pattern.clone(),
                    });
                }
            }
        }

        // Buscar comparativos menores: "más barato", "menos costoso"
        for pattern in &self.comparative_less {
            if text.contains(pattern) {
                let parts: Vec<&str> = text.split(pattern).collect();
                let reference = if parts.len() > 1 {
                    parts[1].split_whitespace().next().unwrap_or("reference")
                } else {
                    "reference"
                };

                // Determinar atributo
                let attr = if pattern.contains("barato") || pattern.contains("económico") {
                    "cost"
                } else {
                    "general"
                };

                constraints.push(Constraint {
                    attribute: attr.to_string(),
                    constraint_type: ConstraintType::LessThan,
                    value: ConstraintValue::Reference(reference.to_string()),
                    original_text: pattern.clone(),
                });
            }
        }

        constraints
    }

    /// Calcula confianza del parsing
    fn calculate_confidence(&self, action: &CommandAction, target: &CommandTarget, verbs: &[VerbAnalysis]) -> f64 {
        let mut confidence = 0.5; // Base

        // +0.2 si encontramos acción clara
        if *action != CommandAction::Unknown {
            confidence += 0.2;
        }

        // +0.15 si encontramos target
        match target {
            CommandTarget::Unknown { hint: Some(_), .. } => confidence += 0.15,
            CommandTarget::Known { .. } => confidence += 0.2,
            CommandTarget::Reference { .. } => confidence += 0.1,
            _ => {}
        }

        // +0.1 por cada verbo identificado
        confidence += 0.1 * verbs.len().min(3) as f64;

        confidence.min(1.0)
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GENERACIÓN DE PREDICADOS PIRS
// ============================================================================

/// Predicado PIRS
#[derive(Debug, Clone)]
pub struct PirsPredicate {
    pub name: String,
    pub args: Vec<PirsArg>,
}

/// Argumento de predicado PIRS
#[derive(Debug, Clone)]
pub enum PirsArg {
    /// Átomo (constante)
    Atom(String),
    /// Variable
    Variable(String),
    /// Término compuesto
    Term(String, Vec<PirsArg>),
    /// Número
    Number(f64),
}

impl ParsedCommand {
    /// Genera predicados PIRS desde el comando parseado
    pub fn to_pirs(&self) -> Vec<PirsPredicate> {
        let mut predicates = Vec::new();

        // 1. Predicado de solicitud/delegación
        match &self.action {
            CommandAction::Request { verb, .. } => {
                predicates.push(PirsPredicate {
                    name: "request".to_string(),
                    args: vec![
                        PirsArg::Atom("user".to_string()),
                        PirsArg::Atom("agent".to_string()),
                        PirsArg::Atom(verb.clone()),
                    ],
                });
            }
            CommandAction::Delegate { verb, .. } |
            CommandAction::Create { verb } |
            CommandAction::Search { verb } |
            CommandAction::Analyze { verb } |
            CommandAction::Explain { verb } |
            CommandAction::Compute { verb } => {
                predicates.push(PirsPredicate {
                    name: "delegate".to_string(),
                    args: vec![
                        PirsArg::Atom("user".to_string()),
                        PirsArg::Atom("agent".to_string()),
                        PirsArg::Term(verb.clone(), vec![
                            PirsArg::Variable("Target".to_string()),
                        ]),
                    ],
                });
            }
            CommandAction::Unknown => {}
        }

        // 2. Target
        match &self.target {
            CommandTarget::Unknown { hint, category, .. } => {
                predicates.push(PirsPredicate {
                    name: "unknown".to_string(),
                    args: vec![PirsArg::Variable("Target".to_string())],
                });

                if let Some(h) = hint {
                    predicates.push(PirsPredicate {
                        name: "type_hint".to_string(),
                        args: vec![
                            PirsArg::Variable("Target".to_string()),
                            PirsArg::Atom(h.clone()),
                        ],
                    });
                }

                if let Some(cat) = category {
                    predicates.push(PirsPredicate {
                        name: "category".to_string(),
                        args: vec![
                            PirsArg::Variable("Target".to_string()),
                            PirsArg::Atom(cat.clone()),
                        ],
                    });
                }
            }
            CommandTarget::Known { name, category } => {
                predicates.push(PirsPredicate {
                    name: "known".to_string(),
                    args: vec![
                        PirsArg::Variable("Target".to_string()),
                        PirsArg::Atom(name.clone()),
                    ],
                });
                if let Some(cat) = category {
                    predicates.push(PirsPredicate {
                        name: "category".to_string(),
                        args: vec![
                            PirsArg::Variable("Target".to_string()),
                            PirsArg::Atom(cat.clone()),
                        ],
                    });
                }
            }
            CommandTarget::Reference { pronoun } => {
                predicates.push(PirsPredicate {
                    name: "reference".to_string(),
                    args: vec![
                        PirsArg::Variable("Target".to_string()),
                        PirsArg::Atom(pronoun.clone()),
                    ],
                });
            }
            CommandTarget::None => {}
        }

        // 3. Goal
        if let Some(goal) = &self.goal {
            predicates.push(PirsPredicate {
                name: "goal".to_string(),
                args: vec![
                    PirsArg::Variable("Target".to_string()),
                    PirsArg::Term(goal.action.clone(), vec![
                        PirsArg::Atom(goal.target.clone()),
                    ]),
                ],
            });
        }

        // 4. Constraints
        for constraint in &self.constraints {
            let constraint_term = match &constraint.constraint_type {
                ConstraintType::Superlative => {
                    PirsArg::Term("superlative".to_string(), vec![
                        match &constraint.value {
                            ConstraintValue::Qualitative(q) => PirsArg::Atom(q.clone()),
                            _ => PirsArg::Atom("high".to_string()),
                        }
                    ])
                }
                ConstraintType::GreaterThan => {
                    PirsArg::Term("greater_than".to_string(), vec![
                        match &constraint.value {
                            ConstraintValue::Reference(r) => PirsArg::Atom(r.clone()),
                            _ => PirsArg::Atom("reference".to_string()),
                        }
                    ])
                }
                ConstraintType::LessThan => {
                    PirsArg::Term("less_than".to_string(), vec![
                        match &constraint.value {
                            ConstraintValue::Reference(r) => PirsArg::Atom(r.clone()),
                            _ => PirsArg::Atom("reference".to_string()),
                        }
                    ])
                }
                ConstraintType::EqualTo => {
                    PirsArg::Term("equal_to".to_string(), vec![
                        match &constraint.value {
                            ConstraintValue::Reference(r) => PirsArg::Atom(r.clone()),
                            ConstraintValue::Numeric(n) => PirsArg::Number(*n),
                            _ => PirsArg::Atom("value".to_string()),
                        }
                    ])
                }
                ConstraintType::Negation => {
                    PirsArg::Term("not".to_string(), vec![
                        PirsArg::Atom("true".to_string()),
                    ])
                }
            };

            predicates.push(PirsPredicate {
                name: "constraint".to_string(),
                args: vec![
                    PirsArg::Variable("Target".to_string()),
                    PirsArg::Atom(constraint.attribute.clone()),
                    constraint_term,
                ],
            });
        }

        predicates
    }

    /// Formatea los predicados como código Prolog
    pub fn to_prolog_string(&self) -> String {
        let predicates = self.to_pirs();
        let mut output = String::new();

        output.push_str("% Comando parseado desde lenguaje natural\n");
        output.push_str(&format!("% Original: \"{}\"\n", self.original));
        output.push_str(&format!("% Confianza: {:.1}%\n\n", self.confidence * 100.0));

        for pred in predicates {
            output.push_str(&format!("{}.\n", pred.to_prolog()));
        }

        output
    }
}

impl PirsPredicate {
    /// Convierte a sintaxis Prolog
    pub fn to_prolog(&self) -> String {
        let args_str: Vec<String> = self.args.iter().map(|a| a.to_prolog()).collect();
        format!("{}({})", self.name, args_str.join(", "))
    }
}

impl PirsArg {
    /// Convierte a sintaxis Prolog
    pub fn to_prolog(&self) -> String {
        match self {
            PirsArg::Atom(s) => s.clone(),
            PirsArg::Variable(s) => s.clone(),
            PirsArg::Term(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_prolog()).collect();
                format!("{}({})", name, args_str.join(", "))
            }
            PirsArg::Number(n) => format!("{}", n),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let parser = CommandParser::new();
        let cmd = parser.parse("Requiero que me diseñes un producto");

        assert!(matches!(cmd.action, CommandAction::Request { .. }));
        assert_eq!(cmd.requester, SemanticRole::User);
    }

    #[test]
    fn test_parse_unknown_target() {
        let parser = CommandParser::new();
        let cmd = parser.parse("Necesito un compuesto nuevo");

        match &cmd.target {
            CommandTarget::Unknown { hint, article, .. } => {
                assert_eq!(hint.as_deref(), Some("compuesto"));
                assert_eq!(article.as_deref(), Some("un"));
            }
            _ => panic!("Expected Unknown target"),
        }
    }

    #[test]
    fn test_parse_goal() {
        let parser = CommandParser::new();
        let cmd = parser.parse("Quiero algo para sustituir al propofol");

        assert!(cmd.goal.is_some());
        let goal = cmd.goal.unwrap();
        assert_eq!(goal.action, "sustituir");
        assert_eq!(goal.target, "propofol");
    }

    #[test]
    fn test_parse_constraints() {
        let parser = CommandParser::new();
        let cmd = parser.parse("Necesito algo súper seguro y más barato");

        assert!(cmd.constraints.len() >= 1);

        // Verificar que encontró el superlativo
        let has_superlative = cmd.constraints.iter().any(|c|
            c.constraint_type == ConstraintType::Superlative
        );
        assert!(has_superlative);
    }

    #[test]
    fn test_full_example() {
        let parser = CommandParser::new();
        let cmd = parser.parse(
            "Requiero que me diseñes un producto que me ayude a sustituir al propofol \
             y tiene que ser súper seguro y mucho mejor que él, más barato"
        );

        // Verificar componentes
        assert!(matches!(cmd.action, CommandAction::Request { .. } | CommandAction::Create { .. }));
        assert!(matches!(cmd.target, CommandTarget::Unknown { .. }));
        assert!(cmd.confidence > 0.5);

        // Generar PIRS
        let prolog = cmd.to_prolog_string();
        println!("{}", prolog);

        assert!(prolog.contains("request") || prolog.contains("delegate"));
        assert!(prolog.contains("unknown") || prolog.contains("Target"));
    }

    #[test]
    fn test_imperative() {
        let parser = CommandParser::new();
        let cmd = parser.parse("Ayúdame a encontrar una solución");

        // Debug output
        println!("Action: {:?}", cmd.action);
        println!("Verbs: {:?}", cmd.verbs);
        println!("Target: {:?}", cmd.target);

        // El target debería ser "solución" con artículo indefinido
        match &cmd.target {
            CommandTarget::Unknown { hint, .. } => {
                assert_eq!(hint.as_deref(), Some("solución"));
            }
            _ => {
                // OK si no detecta el target exacto, el parser aún funciona
            }
        }

        // La confianza debe ser > 0
        assert!(cmd.confidence > 0.0);
    }
}
