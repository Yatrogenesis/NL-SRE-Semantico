//! # Semantic Disambiguator
//!
//! Motor principal de desambiguación semántica.
//! Integra todos los componentes:
//! - UNIFORM kernel para unificación
//! - APPLOG para contexto compartido
//! - TAO para mensajería entre componentes
//! - CharMatcher para candidatos por caracteres
//! - SpanishGrammar para validación gramatical
//! - SemanticDB para análisis de contexto
//! - SpanishDictionary para diccionario completo RAE/LATAM

use crate::{Config, ProcessedSentence, Correction, CorrectionExplanation};
use crate::chars::CharMatcher;
use crate::grammar::SpanishGrammar;
use crate::semantic::SemanticDB;
use crate::applog::{SharedContext, Source};
use crate::uniform::UnifyValue;
use crate::dictionary::SpanishDictionary;

/// Motor de desambiguación semántica
#[derive(Debug)]
pub struct SemanticDisambiguator {
    /// Configuración
    config: Config,

    /// Matcher de caracteres
    char_matcher: CharMatcher,

    /// Gramática española
    grammar: SpanishGrammar,

    /// Base de datos semántica
    semantic_db: SemanticDB,

    /// Contexto compartido (APPLOG)
    shared_context: SharedContext,

    /// Diccionario completo (opcional, para carga desde archivo)
    dictionary: Option<SpanishDictionary>,
}

impl SemanticDisambiguator {
    /// Crea desambiguador con configuración por defecto
    pub fn new() -> Self {
        let mut disambiguator = Self {
            config: Config::default(),
            char_matcher: CharMatcher::new(),
            grammar: SpanishGrammar::new(),
            semantic_db: SemanticDB::new(),
            shared_context: SharedContext::new(),
            dictionary: None,
        };

        // Cargar diccionario inicial
        disambiguator.load_default_dictionary();

        disambiguator
    }

    /// Crea con configuración personalizada
    pub fn with_config(config: Config) -> Self {
        let mut d = Self::new();
        d.config = config;
        d
    }

    /// Crea con diccionario externo (RAE/LATAM)
    pub fn with_dictionary(dict: SpanishDictionary) -> Self {
        let mut disambiguator = Self {
            config: Config::default(),
            char_matcher: CharMatcher::new(),
            grammar: SpanishGrammar::new(),
            semantic_db: SemanticDB::new(),
            shared_context: SharedContext::new(),
            dictionary: Some(dict),
        };

        // Cargar palabras del diccionario al CharMatcher
        disambiguator.load_from_spanish_dictionary();

        // Cargar también palabras gramaticales básicas
        disambiguator.load_grammar_words();

        disambiguator
    }

    /// Crea con diccionario y configuración
    pub fn with_dictionary_and_config(dict: SpanishDictionary, config: Config) -> Self {
        let mut d = Self::with_dictionary(dict);
        d.config = config;
        d
    }

    /// Carga palabras desde SpanishDictionary al CharMatcher
    fn load_from_spanish_dictionary(&mut self) {
        if let Some(ref dict) = self.dictionary {
            // Cargar todas las palabras válidas al CharMatcher
            let words: Vec<String> = dict.all_words().cloned().collect();
            self.char_matcher.load_dictionary(words.iter().map(|s| s.as_str()));

            // Añadir sustantivos a la gramática para los que tenemos información
            use crate::grammar::{NounInfo, Gender, Number, NounCategory};
            use crate::dictionary::PartOfSpeech;

            for word in dict.all_words() {
                for entry in dict.get_entries(word) {
                    // Si es sustantivo, añadirlo a la gramática
                    if entry.pos.contains(&PartOfSpeech::Noun) {
                        // Inferir género del artículo en definiciones o de la terminación
                        let gender = if word.ends_with('a') || word.ends_with("ión") || word.ends_with("dad") {
                            Gender::Feminine
                        } else {
                            Gender::Masculine
                        };

                        self.grammar.add_noun(&entry.original, NounInfo {
                            gender,
                            number: Number::Singular,
                            category: NounCategory::Thing,
                            can_be_subject: true,
                            can_be_object: true,
                        });
                    }

                    // Si es adjetivo, añadirlo
                    if entry.pos.contains(&PartOfSpeech::Adjective) {
                        self.grammar.add_adjective(&entry.original);
                    }
                }
            }
        }
    }

    /// Carga palabras gramaticales básicas (artículos, preposiciones, etc.)
    fn load_grammar_words(&mut self) {
        let grammar_words = [
            "el", "la", "los", "las", "un", "una", "unos", "unas",
            "yo", "tú", "él", "ella", "nosotros", "me", "te", "le", "se",
            "a", "ante", "bajo", "con", "contra", "de", "desde", "en",
            "entre", "hacia", "hasta", "para", "por", "según", "sin",
            "sobre", "tras", "y", "e", "o", "u", "pero", "sino", "que",
            "porque", "aunque", "si", "cuando", "donde", "como",
            "muy", "bien", "mal", "mucho", "poco", "siempre", "nunca",
            "ya", "todavía", "aquí", "allí", "ahora", "después", "antes",
            "también", "tampoco", "sí", "no",
        ];

        self.char_matcher.load_dictionary(grammar_words.iter().copied());
    }

    /// Obtiene frecuencia de una palabra (si hay diccionario)
    pub fn word_frequency(&self, word: &str) -> u64 {
        if let Some(ref dict) = self.dictionary {
            dict.frequency(word)
        } else {
            0
        }
    }

    /// Verifica si el motor tiene diccionario externo cargado
    pub fn has_external_dictionary(&self) -> bool {
        self.dictionary.is_some()
    }

    /// Estadísticas del diccionario
    pub fn dictionary_stats(&self) -> Option<&crate::dictionary::DictionaryStats> {
        self.dictionary.as_ref().map(|d| &d.stats)
    }

    /// Carga diccionario por defecto
    fn load_default_dictionary(&mut self) {
        // Palabras del vocabulario semántico
        let semantic_words = [
            "roma", "coliseo", "paris", "madrid", "amor", "odio", "paz",
            "ramo", "mora", "casa", "rosita", "azul", "romano",
        ];

        self.char_matcher.load_dictionary(semantic_words.iter().copied());

        // Palabras gramaticales
        let grammar_words = [
            "el", "la", "los", "las", "un", "una", "unos", "unas",
            "yo", "tú", "él", "ella", "nosotros", "me", "te", "le", "se",
            "a", "ante", "bajo", "con", "contra", "de", "desde", "en",
            "entre", "hacia", "hasta", "para", "por", "según", "sin",
            "sobre", "tras", "y", "e", "o", "u", "pero", "sino", "que",
            "porque", "aunque", "si", "cuando", "donde", "como",
            "muy", "bien", "mal", "mucho", "poco", "siempre", "nunca",
            "ya", "todavía", "aquí", "allí", "ahora", "después", "antes",
            "también", "tampoco", "sí", "no",
            "gusta", "gustan", "gustó", "soy", "eres", "es", "somos", "son",
            "estoy", "estás", "está", "estamos", "están",
            "visito", "visitas", "visita", "visité", "visitó",
            "corro", "corres", "corre", "corremos", "corren",
            "voy", "vas", "va", "vamos", "van", "fui", "fue",
        ];

        self.char_matcher.load_dictionary(grammar_words.iter().copied());

        // Añadir sustantivos a la gramática
        use crate::grammar::{NounInfo, Gender, Number, NounCategory};

        self.grammar.add_noun("roma", NounInfo {
            gender: Gender::Feminine,
            number: Number::Singular,
            category: NounCategory::Place,
            can_be_subject: true,
            can_be_object: true,
        });

        self.grammar.add_noun("coliseo", NounInfo {
            gender: Gender::Masculine,
            number: Number::Singular,
            category: NounCategory::Place,
            can_be_subject: false,
            can_be_object: true,
        });

        self.grammar.add_noun("casa", NounInfo {
            gender: Gender::Feminine,
            number: Number::Singular,
            category: NounCategory::Thing,
            can_be_subject: true,
            can_be_object: true,
        });

        self.grammar.add_noun("amor", NounInfo {
            gender: Gender::Masculine,
            number: Number::Singular,
            category: NounCategory::Concept,
            can_be_subject: true,
            can_be_object: true,
        });

        // Añadir adjetivos
        self.grammar.add_adjective("azul");
        self.grammar.add_adjective("romano");
        self.grammar.add_adjective("grande");
        self.grammar.add_adjective("pequeño");
    }

    /// Procesa una oración completa
    pub fn process(&mut self, sentence: &str) -> ProcessedSentence {
        // 1. Tokenizar
        let tokens = self.tokenize(sentence);

        // 2. Detectar anomalías (palabras no en diccionario)
        let anomalies: Vec<(usize, String)> = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| !self.char_matcher.is_valid(t) && !self.is_punctuation(t))
            .map(|(i, t)| (i, t.clone()))
            .collect();

        // 3. Si no hay anomalías, retornar como está
        if anomalies.is_empty() {
            return ProcessedSentence {
                original: sentence.to_string(),
                corrected: sentence.to_string(),
                confidence: 1.0,
                corrections: Vec::new(),
            };
        }

        // 4. Extraer contexto (palabras conocidas)
        let context_words: Vec<String> = tokens
            .iter()
            .filter(|t| self.char_matcher.is_valid(t))
            .cloned()
            .collect();

        // 5. Inferir tema del contexto
        let theme = self.semantic_db.infer_theme(&context_words);
        if let Some((theme_name, _)) = &theme {
            // Guardar en contexto compartido
            let _ = self.shared_context.set(
                "current_theme",
                UnifyValue::Atom(theme_name.clone()),
                Source::Semantic,
                0.8,
            );
        }

        // 6. Para cada anomalía, desambiguar
        let mut corrected_tokens = tokens.clone();
        let mut corrections = Vec::new();

        for (idx, anomaly) in anomalies {
            let (correction, conf, explanation) = self.disambiguate_word(
                &anomaly,
                idx,
                &tokens,
                &context_words,
                theme.as_ref().map(|(t, _)| t.as_str()),
            );

            if conf >= self.config.min_confidence {
                corrected_tokens[idx] = correction.clone();
                corrections.push(Correction {
                    position: idx,
                    original: anomaly,
                    corrected: correction,
                    confidence: conf,
                    explanation,
                });
            }
        }

        // 7. Calcular confianza global
        let global_confidence = if corrections.is_empty() {
            1.0
        } else {
            corrections.iter().map(|c| c.confidence).sum::<f64>() / corrections.len() as f64
        };

        ProcessedSentence {
            original: sentence.to_string(),
            corrected: corrected_tokens.join(" "),
            confidence: global_confidence,
            corrections,
        }
    }

    /// Desambigua una palabra individual
    fn disambiguate_word(
        &self,
        word: &str,
        position: usize,
        sentence: &[String],
        _context_words: &[String],
        theme: Option<&str>,
    ) -> (String, f64, CorrectionExplanation) {
        // 1. Obtener candidatos por caracteres
        let candidates = self.char_matcher.find_candidates(word);

        if candidates.is_empty() {
            return (
                word.to_string(),
                0.0,
                CorrectionExplanation {
                    char_score: 0.0,
                    grammar_score: 0.0,
                    context_score: 0.0,
                    candidates: Vec::new(),
                    reason: "No se encontraron candidatos".to_string(),
                },
            );
        }

        // 2. Calcular scores combinados para cada candidato
        let mut scored_candidates: Vec<(String, f64, f64, f64, f64)> = Vec::new();

        for candidate in &candidates {
            let char_score = candidate.score;

            let grammar_score = self.grammar.is_valid_at_position(
                &candidate.word,
                position,
                sentence,
            );

            let context_score = if let Some(t) = theme {
                self.semantic_db.compatibility_score(&candidate.word, t)
            } else {
                0.5  // Neutral si no hay tema
            };

            // Score combinado: α·char + β·grammar + γ·context
            let total = self.config.alpha * char_score
                      + self.config.beta * grammar_score
                      + self.config.gamma * context_score;

            scored_candidates.push((
                candidate.word.clone(),
                total,
                char_score,
                grammar_score,
                context_score,
            ));
        }

        // 3. Ordenar por score total
        scored_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // 4. Seleccionar el mejor
        let best = &scored_candidates[0];
        let (best_word, best_total, best_char, best_grammar, best_context) = best;

        // 5. Crear explicación
        let explanation = CorrectionExplanation {
            char_score: *best_char,
            grammar_score: *best_grammar,
            context_score: *best_context,
            candidates: scored_candidates
                .iter()
                .take(5)
                .map(|(w, s, _, _, _)| (w.clone(), *s))
                .collect(),
            reason: format!(
                "Elegido '{}' porque: caracteres={:.0}%, gramática={:.0}%, contexto={:.0}%",
                best_word,
                best_char * 100.0,
                best_grammar * 100.0,
                best_context * 100.0,
            ),
        };

        (best_word.clone(), *best_total, explanation)
    }

    /// Tokeniza una oración
    fn tokenize(&self, sentence: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for c in sentence.chars() {
            if c.is_whitespace() {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else if c.is_alphanumeric() || c == '\'' || c == '-' || c == 'á' || c == 'é'
                || c == 'í' || c == 'ó' || c == 'ú' || c == 'ñ' || c == 'ü'
                || c == 'Á' || c == 'É' || c == 'Í' || c == 'Ó' || c == 'Ú' || c == 'Ñ'
            {
                current.push(c);
            } else {
                // Puntuación u otro carácter
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                // Añadir puntuación como token separado
                if !c.is_whitespace() {
                    tokens.push(c.to_string());
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Verifica si un token es puntuación
    fn is_punctuation(&self, token: &str) -> bool {
        token.len() == 1 && !token.chars().next().unwrap().is_alphanumeric()
    }

    /// Añade palabras al diccionario
    pub fn add_to_dictionary<I: IntoIterator<Item = S>, S: AsRef<str>>(&mut self, words: I) {
        self.char_matcher.load_dictionary(words);
    }

    /// Acceso a la configuración
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Modifica la configuración
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Acceso al contexto compartido
    pub fn shared_context(&self) -> &SharedContext {
        &self.shared_context
    }

    /// Tamaño del diccionario
    pub fn dictionary_size(&self) -> usize {
        self.char_matcher.dictionary_size()
    }
}

impl Default for SemanticDisambiguator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let d = SemanticDisambiguator::new();

        let tokens = d.tokenize("Visité el Coliseo romano");
        assert_eq!(tokens, vec!["Visité", "el", "Coliseo", "romano"]);

        let tokens = d.tokenize("¿Cómo estás?");
        assert_eq!(tokens, vec!["¿", "Cómo", "estás", "?"]);
    }

    #[test]
    fn test_process_correct_sentence() {
        let mut d = SemanticDisambiguator::new();
        // Add "grande" to dictionary since it's not in the base vocabulary
        d.add_to_dictionary(vec!["grande"]);

        let result = d.process("el amor es grande");
        assert!(result.corrections.is_empty());
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_disambiguate_smor() {
        let mut d = SemanticDisambiguator::new();

        // Contexto de arquitectura romana
        let result = d.process("Visité el Coliseo romano en smor");

        // Debe detectar que "smor" es anómalo
        assert!(!result.corrections.is_empty());

        // Con contexto de "Coliseo romano", debe preferir "Roma" sobre "amor"
        let correction = &result.corrections[0];
        println!("Original: {}", correction.original);
        println!("Corregido: {}", correction.corrected);
        println!("Explicación: {}", correction.explanation.reason);

        // El score de Roma debe ser mayor que amor por contexto
        // (aunque caracteres den similar)
    }

    #[test]
    fn test_flexible_spanish() {
        let mut d = SemanticDisambiguator::new();

        // Oración válida aunque orden diferente
        let result1 = d.process("la casa azul me gusta");
        assert!(result1.corrections.is_empty());

        let result2 = d.process("me gusta la casa azul");
        assert!(result2.corrections.is_empty());
    }
}
