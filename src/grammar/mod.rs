//! # Spanish Grammar Module
//!
//! Gramática española con orden flexible de palabras.
//! El español permite múltiples ordenamientos válidos:
//! - "Me gusta la casa azul de Rosita"
//! - "La casa azul de Rosita me gusta"
//! - "La casa de Rosita me gusta azul"
//!
//! ## Capas de validación
//! 1. NÚCLEO: Sujeto + Predicado (acción)
//! 2. REFINAMIENTO: Artículos, adjetivos, complementos

use std::collections::{HashMap, HashSet};
use crate::tao::{GrammaticalRole, GrammaticalStructure, GrammaticalComponent, SentenceType};

/// Motor de gramática española
#[derive(Debug, Clone)]
pub struct SpanishGrammar {
    /// Verbos conocidos con su transitividad
    verbs: HashMap<String, VerbInfo>,

    /// Sustantivos con género y número
    nouns: HashMap<String, NounInfo>,

    /// Adjetivos
    adjectives: HashSet<String>,

    /// Preposiciones
    prepositions: HashSet<String>,

    /// Artículos (determinados e indeterminados)
    articles: HashMap<String, ArticleInfo>,

    /// Pronombres
    pronouns: HashMap<String, PronounInfo>,

    /// Conjunciones
    conjunctions: HashSet<String>,

    /// Adverbios
    adverbs: HashSet<String>,
}

/// Información de un verbo
#[derive(Debug, Clone)]
pub struct VerbInfo {
    /// Infinitivo
    pub infinitive: String,
    /// Es transitivo (requiere objeto directo)
    pub transitive: bool,
    /// Conjugaciones conocidas -> persona/número
    pub conjugations: HashMap<String, Conjugation>,
    /// Categoría semántica
    pub category: VerbCategory,
}

/// Conjugación
#[derive(Debug, Clone)]
pub struct Conjugation {
    pub person: Person,
    pub number: Number,
    pub tense: Tense,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tense {
    Present,
    Past,
    Future,
    Imperfect,
    Conditional,
    Subjunctive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbCategory {
    Action,      // correr, saltar
    State,       // ser, estar
    Movement,    // ir, venir, visitar
    Perception,  // ver, oír
    Emotion,     // gustar, amar
    Cognitive,   // pensar, saber
}

/// Información de un sustantivo
#[derive(Debug, Clone)]
pub struct NounInfo {
    pub gender: Gender,
    pub number: Number,
    pub category: NounCategory,
    /// Puede ser sujeto
    pub can_be_subject: bool,
    /// Puede ser objeto
    pub can_be_object: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NounCategory {
    Person,
    Place,
    Thing,
    Animal,
    Concept,
    Time,
}

/// Información de artículo
#[derive(Debug, Clone)]
pub struct ArticleInfo {
    pub definite: bool,  // el/la vs un/una
    pub gender: Gender,
    pub number: Number,
}

/// Información de pronombre
#[derive(Debug, Clone)]
pub struct PronounInfo {
    pub person: Person,
    pub number: Number,
    pub case: PronounCase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PronounCase {
    Subject,     // yo, tú, él
    DirectObj,   // me, te, lo/la
    IndirectObj, // me, te, le
    Reflexive,   // me, te, se
}

/// Resultado de análisis gramatical
#[derive(Debug, Clone)]
pub struct GrammarAnalysis {
    /// Estructura detectada
    pub structure: GrammaticalStructure,
    /// Score de validez gramatical (0.0 - 1.0)
    pub validity_score: f64,
    /// Errores o warnings detectados
    pub issues: Vec<GrammarIssue>,
    /// Palabra esperada en posición dada
    pub expected_at: HashMap<usize, ExpectedWord>,
}

/// Tipo esperado de palabra en una posición
#[derive(Debug, Clone)]
pub struct ExpectedWord {
    pub roles: Vec<GrammaticalRole>,
    pub categories: Vec<String>,
    pub required: bool,
}

/// Problema gramatical detectado
#[derive(Debug, Clone)]
pub struct GrammarIssue {
    pub position: usize,
    pub severity: IssueSeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

impl SpanishGrammar {
    /// Crea gramática con vocabulario base
    pub fn new() -> Self {
        let mut grammar = Self {
            verbs: HashMap::new(),
            nouns: HashMap::new(),
            adjectives: HashSet::new(),
            prepositions: HashSet::new(),
            articles: HashMap::new(),
            pronouns: HashMap::new(),
            conjunctions: HashSet::new(),
            adverbs: HashSet::new(),
        };

        grammar.load_base_vocabulary();
        grammar
    }

    /// Carga vocabulario base
    fn load_base_vocabulary(&mut self) {
        // === ARTÍCULOS ===
        self.articles.insert("el".to_string(), ArticleInfo {
            definite: true, gender: Gender::Masculine, number: Number::Singular
        });
        self.articles.insert("la".to_string(), ArticleInfo {
            definite: true, gender: Gender::Feminine, number: Number::Singular
        });
        self.articles.insert("los".to_string(), ArticleInfo {
            definite: true, gender: Gender::Masculine, number: Number::Plural
        });
        self.articles.insert("las".to_string(), ArticleInfo {
            definite: true, gender: Gender::Feminine, number: Number::Plural
        });
        self.articles.insert("un".to_string(), ArticleInfo {
            definite: false, gender: Gender::Masculine, number: Number::Singular
        });
        self.articles.insert("una".to_string(), ArticleInfo {
            definite: false, gender: Gender::Feminine, number: Number::Singular
        });
        self.articles.insert("unos".to_string(), ArticleInfo {
            definite: false, gender: Gender::Masculine, number: Number::Plural
        });
        self.articles.insert("unas".to_string(), ArticleInfo {
            definite: false, gender: Gender::Feminine, number: Number::Plural
        });

        // === PREPOSICIONES ===
        for prep in &["a", "ante", "bajo", "con", "contra", "de", "desde",
                      "en", "entre", "hacia", "hasta", "para", "por",
                      "según", "sin", "sobre", "tras"] {
            self.prepositions.insert(prep.to_string());
        }

        // === PRONOMBRES ===
        self.pronouns.insert("yo".to_string(), PronounInfo {
            person: Person::First, number: Number::Singular, case: PronounCase::Subject
        });
        self.pronouns.insert("tú".to_string(), PronounInfo {
            person: Person::Second, number: Number::Singular, case: PronounCase::Subject
        });
        self.pronouns.insert("él".to_string(), PronounInfo {
            person: Person::Third, number: Number::Singular, case: PronounCase::Subject
        });
        self.pronouns.insert("ella".to_string(), PronounInfo {
            person: Person::Third, number: Number::Singular, case: PronounCase::Subject
        });
        self.pronouns.insert("nosotros".to_string(), PronounInfo {
            person: Person::First, number: Number::Plural, case: PronounCase::Subject
        });
        self.pronouns.insert("me".to_string(), PronounInfo {
            person: Person::First, number: Number::Singular, case: PronounCase::DirectObj
        });
        self.pronouns.insert("te".to_string(), PronounInfo {
            person: Person::Second, number: Number::Singular, case: PronounCase::DirectObj
        });
        self.pronouns.insert("le".to_string(), PronounInfo {
            person: Person::Third, number: Number::Singular, case: PronounCase::IndirectObj
        });
        self.pronouns.insert("se".to_string(), PronounInfo {
            person: Person::Third, number: Number::Singular, case: PronounCase::Reflexive
        });

        // === CONJUNCIONES ===
        for conj in &["y", "e", "o", "u", "pero", "sino", "que", "porque",
                      "aunque", "si", "cuando", "donde", "como"] {
            self.conjunctions.insert(conj.to_string());
        }

        // === ADVERBIOS COMUNES ===
        for adv in &["muy", "bien", "mal", "mucho", "poco", "siempre",
                     "nunca", "ya", "todavía", "aquí", "allí", "ahora",
                     "después", "antes", "también", "tampoco", "sí", "no"] {
            self.adverbs.insert(adv.to_string());
        }

        // === VERBOS BASE ===
        self.add_verb_gustar();
        self.add_verb_ser();
        self.add_verb_estar();
        self.add_verb_visitar();
        self.add_verb_correr();
        self.add_verb_ir();
    }

    fn add_verb_gustar(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("gusta".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("gustan".to_string(), Conjugation {
            person: Person::Third, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("gustó".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Past
        });

        self.verbs.insert("gustar".to_string(), VerbInfo {
            infinitive: "gustar".to_string(),
            transitive: false,  // Verbo especial con dativo
            conjugations,
            category: VerbCategory::Emotion,
        });
    }

    fn add_verb_ser(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("soy".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("eres".to_string(), Conjugation {
            person: Person::Second, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("es".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("somos".to_string(), Conjugation {
            person: Person::First, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("son".to_string(), Conjugation {
            person: Person::Third, number: Number::Plural, tense: Tense::Present
        });

        self.verbs.insert("ser".to_string(), VerbInfo {
            infinitive: "ser".to_string(),
            transitive: false,
            conjugations,
            category: VerbCategory::State,
        });
    }

    fn add_verb_estar(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("estoy".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("estás".to_string(), Conjugation {
            person: Person::Second, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("está".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("estamos".to_string(), Conjugation {
            person: Person::First, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("están".to_string(), Conjugation {
            person: Person::Third, number: Number::Plural, tense: Tense::Present
        });

        self.verbs.insert("estar".to_string(), VerbInfo {
            infinitive: "estar".to_string(),
            transitive: false,
            conjugations,
            category: VerbCategory::State,
        });
    }

    fn add_verb_visitar(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("visito".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("visitas".to_string(), Conjugation {
            person: Person::Second, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("visita".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("visité".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Past
        });
        conjugations.insert("visitó".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Past
        });

        self.verbs.insert("visitar".to_string(), VerbInfo {
            infinitive: "visitar".to_string(),
            transitive: true,
            conjugations,
            category: VerbCategory::Movement,
        });
    }

    fn add_verb_correr(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("corro".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("corres".to_string(), Conjugation {
            person: Person::Second, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("corre".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("corremos".to_string(), Conjugation {
            person: Person::First, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("corren".to_string(), Conjugation {
            person: Person::Third, number: Number::Plural, tense: Tense::Present
        });

        self.verbs.insert("correr".to_string(), VerbInfo {
            infinitive: "correr".to_string(),
            transitive: false,
            conjugations,
            category: VerbCategory::Action,
        });
    }

    fn add_verb_ir(&mut self) {
        let mut conjugations = HashMap::new();
        conjugations.insert("voy".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("vas".to_string(), Conjugation {
            person: Person::Second, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("va".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Present
        });
        conjugations.insert("vamos".to_string(), Conjugation {
            person: Person::First, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("van".to_string(), Conjugation {
            person: Person::Third, number: Number::Plural, tense: Tense::Present
        });
        conjugations.insert("fui".to_string(), Conjugation {
            person: Person::First, number: Number::Singular, tense: Tense::Past
        });
        conjugations.insert("fue".to_string(), Conjugation {
            person: Person::Third, number: Number::Singular, tense: Tense::Past
        });

        self.verbs.insert("ir".to_string(), VerbInfo {
            infinitive: "ir".to_string(),
            transitive: false,
            conjugations,
            category: VerbCategory::Movement,
        });
    }

    /// Añade un sustantivo al vocabulario
    pub fn add_noun(&mut self, word: &str, info: NounInfo) {
        self.nouns.insert(word.to_lowercase(), info);
    }

    /// Añade un adjetivo
    pub fn add_adjective(&mut self, word: &str) {
        self.adjectives.insert(word.to_lowercase());
    }

    /// Analiza una oración tokenizada
    pub fn analyze(&self, tokens: &[String]) -> GrammarAnalysis {
        let mut components = Vec::new();
        let issues: Vec<GrammarIssue> = Vec::new();
        let mut expected_at = HashMap::new();

        // Identificar tipo de cada token
        let token_types: Vec<TokenType> = tokens
            .iter()
            .map(|t| self.classify_token(t))
            .collect();

        // Buscar el verbo (núcleo de la oración)
        let verb_positions: Vec<usize> = token_types
            .iter()
            .enumerate()
            .filter(|(_, tt)| matches!(tt, TokenType::Verb(_)))
            .map(|(i, _)| i)
            .collect();

        let sentence_type = if verb_positions.is_empty() {
            SentenceType::Unknown
        } else {
            self.determine_sentence_type(&token_types, &verb_positions)
        };

        // Construir componentes
        for (i, tt) in token_types.iter().enumerate() {
            let role = match tt {
                TokenType::Verb(_) => Some(GrammaticalRole::Verb),
                TokenType::Noun(_) => {
                    // Determinar si es sujeto u objeto según posición
                    if verb_positions.first().map_or(false, |&v| i < v) {
                        Some(GrammaticalRole::Subject)
                    } else {
                        Some(GrammaticalRole::DirectObject)
                    }
                }
                TokenType::Article(_) => Some(GrammaticalRole::Article),
                TokenType::Adjective => Some(GrammaticalRole::Adjective),
                TokenType::Preposition => Some(GrammaticalRole::Preposition),
                TokenType::Pronoun(_) => {
                    // Determinar rol del pronombre
                    Some(GrammaticalRole::Subject)
                }
                TokenType::Adverb => Some(GrammaticalRole::Adverb),
                TokenType::Conjunction => Some(GrammaticalRole::Conjunction),
                TokenType::Unknown => None,
            };

            if let Some(r) = role {
                components.push(GrammaticalComponent {
                    role: r,
                    tokens: vec![i],
                    head: Some(i),
                });
            }
        }

        // Calcular score de validez
        let validity_score = self.calculate_validity(&token_types, &components, &sentence_type);

        // Determinar qué se espera en cada posición
        self.infer_expectations(&token_types, &mut expected_at);

        GrammarAnalysis {
            structure: GrammaticalStructure {
                sentence_type,
                components,
                inferred_theme: None,  // Se llenará con semántica
            },
            validity_score,
            issues,
            expected_at,
        }
    }

    /// Clasifica un token individual
    fn classify_token(&self, token: &str) -> TokenType {
        let lower = token.to_lowercase();

        // Verificar en orden de especificidad
        if let Some(info) = self.articles.get(&lower) {
            return TokenType::Article(info.clone());
        }

        if self.prepositions.contains(&lower) {
            return TokenType::Preposition;
        }

        if let Some(info) = self.pronouns.get(&lower) {
            return TokenType::Pronoun(info.clone());
        }

        if self.conjunctions.contains(&lower) {
            return TokenType::Conjunction;
        }

        if self.adverbs.contains(&lower) {
            return TokenType::Adverb;
        }

        // Buscar si es conjugación de algún verbo
        for (_, verb_info) in &self.verbs {
            if verb_info.conjugations.contains_key(&lower) {
                return TokenType::Verb(verb_info.clone());
            }
        }

        if self.adjectives.contains(&lower) {
            return TokenType::Adjective;
        }

        if let Some(info) = self.nouns.get(&lower) {
            return TokenType::Noun(info.clone());
        }

        // Por defecto, asumir sustantivo desconocido
        // (podría ser un nombre propio u otra palabra)
        TokenType::Unknown
    }

    /// Determina el tipo de oración basado en orden de componentes
    fn determine_sentence_type(&self, types: &[TokenType], verb_pos: &[usize]) -> SentenceType {
        if verb_pos.is_empty() {
            return SentenceType::Unknown;
        }

        let first_verb = verb_pos[0];

        // Buscar sujeto (sustantivo o pronombre antes del verbo)
        let has_subject_before = types[..first_verb].iter().any(|t| {
            matches!(t, TokenType::Noun(_) | TokenType::Pronoun(_))
        });

        // Buscar objeto después del verbo
        let has_object_after = types[first_verb..].iter().skip(1).any(|t| {
            matches!(t, TokenType::Noun(_))
        });

        // Verbo especial tipo "gustar" con pronombre antes
        let has_dative_pronoun_before = types[..first_verb].iter().any(|t| {
            if let TokenType::Pronoun(info) = t {
                info.case == PronounCase::IndirectObj || info.case == PronounCase::DirectObj
            } else {
                false
            }
        });

        if has_dative_pronoun_before && !has_subject_before {
            // "Me gusta X" - el sujeto está después
            return SentenceType::VSO;
        }

        if first_verb == 0 {
            // Verbo al inicio
            if has_object_after {
                SentenceType::VSO
            } else {
                SentenceType::SV  // Imperativo o intransitivo
            }
        } else if has_subject_before && has_object_after {
            SentenceType::SVO
        } else if has_subject_before {
            SentenceType::SV
        } else if has_object_after {
            // Objeto después pero no sujeto claro antes
            SentenceType::OVS
        } else {
            SentenceType::Impersonal
        }
    }

    /// Calcula score de validez gramatical
    fn calculate_validity(
        &self,
        _types: &[TokenType],
        components: &[GrammaticalComponent],
        sentence_type: &SentenceType,
    ) -> f64 {
        let mut score: f64 = 0.5;  // Base

        // +0.2 si tiene verbo
        if components.iter().any(|c| c.role == GrammaticalRole::Verb) {
            score += 0.2;
        }

        // +0.15 si tiene sujeto identificable
        if components.iter().any(|c| c.role == GrammaticalRole::Subject) {
            score += 0.15;
        }

        // +0.1 si el tipo de oración es reconocible
        if *sentence_type != SentenceType::Unknown {
            score += 0.1;
        }

        // +0.05 por concordancia artículo-sustantivo (simplificado)
        // TODO: verificar género y número

        score.min(1.0)
    }

    /// Infiere qué se espera en cada posición
    fn infer_expectations(&self, types: &[TokenType], expected: &mut HashMap<usize, ExpectedWord>) {
        for (i, tt) in types.iter().enumerate() {
            match tt {
                TokenType::Preposition => {
                    // Después de preposición se espera sintagma nominal
                    if i + 1 < types.len() {
                        expected.insert(i + 1, ExpectedWord {
                            roles: vec![GrammaticalRole::DirectObject],
                            categories: vec!["lugar".to_string(), "cosa".to_string(), "persona".to_string()],
                            required: true,
                        });
                    }
                }
                TokenType::Article(_) => {
                    // Después de artículo se espera sustantivo o adjetivo
                    if i + 1 < types.len() {
                        expected.insert(i + 1, ExpectedWord {
                            roles: vec![GrammaticalRole::Subject, GrammaticalRole::DirectObject],
                            categories: vec!["sustantivo".to_string(), "adjetivo".to_string()],
                            required: true,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    /// Evalúa si una palabra es gramaticalmente válida en una posición
    pub fn is_valid_at_position(
        &self,
        word: &str,
        position: usize,
        sentence: &[String],
    ) -> f64 {
        // Analizar oración con la palabra
        let mut test_sentence = sentence.to_vec();
        if position < test_sentence.len() {
            test_sentence[position] = word.to_string();
        } else {
            test_sentence.push(word.to_string());
        }

        let analysis = self.analyze(&test_sentence);

        // Verificar si la posición tiene expectativas
        if let Some(expected) = analysis.expected_at.get(&position) {
            let word_type = self.classify_token(word);

            // Verificar si el tipo de palabra coincide con lo esperado
            let matches_role = match &word_type {
                TokenType::Noun(_) => expected.roles.iter().any(|r|
                    *r == GrammaticalRole::Subject || *r == GrammaticalRole::DirectObject
                ),
                TokenType::Adjective => expected.roles.contains(&GrammaticalRole::Adjective),
                _ => false,
            };

            if matches_role {
                return analysis.validity_score + 0.1;
            }
        }

        analysis.validity_score
    }
}

/// Tipo de token identificado
#[derive(Debug, Clone)]
enum TokenType {
    Verb(VerbInfo),
    Noun(NounInfo),
    Article(ArticleInfo),
    Adjective,
    Preposition,
    Pronoun(PronounInfo),
    Adverb,
    Conjunction,
    Unknown,
}

impl Default for SpanishGrammar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_articles() {
        let grammar = SpanishGrammar::new();

        let tt = grammar.classify_token("el");
        assert!(matches!(tt, TokenType::Article(_)));

        let tt = grammar.classify_token("la");
        assert!(matches!(tt, TokenType::Article(_)));
    }

    #[test]
    fn test_classify_verbs() {
        let grammar = SpanishGrammar::new();

        let tt = grammar.classify_token("gusta");
        assert!(matches!(tt, TokenType::Verb(_)));

        let tt = grammar.classify_token("visité");
        assert!(matches!(tt, TokenType::Verb(_)));
    }

    #[test]
    fn test_sentence_analysis() {
        let mut grammar = SpanishGrammar::new();

        // Añadir sustantivos de prueba
        grammar.add_noun("coliseo", NounInfo {
            gender: Gender::Masculine,
            number: Number::Singular,
            category: NounCategory::Place,
            can_be_subject: false,
            can_be_object: true,
        });

        let tokens: Vec<String> = vec!["visité", "el", "coliseo"]
            .into_iter()
            .map(String::from)
            .collect();

        let analysis = grammar.analyze(&tokens);
        assert!(analysis.validity_score > 0.5);
    }

    #[test]
    fn test_flexible_order() {
        let grammar = SpanishGrammar::new();

        // "Me gusta" - verbo con pronombre dativo antes
        let tokens1: Vec<String> = vec!["me", "gusta"]
            .into_iter()
            .map(String::from)
            .collect();

        let analysis1 = grammar.analyze(&tokens1);
        // Debería reconocer estructura válida
        assert!(analysis1.validity_score > 0.5);
    }
}
