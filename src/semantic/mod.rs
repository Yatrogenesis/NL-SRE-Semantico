//! # Semantic Database Module
//!
//! Base de datos semántica para categorización de palabras y
//! análisis de compatibilidad contextual.
//!
//! ## Ejemplo
//! - "Roma" → Lugar(Italia) → compatible con tema "arquitectura_romana"
//! - "amor" → Emoción(positiva) → incompatible con tema "arquitectura"

use std::collections::HashMap;

/// Base de datos semántica
#[derive(Debug, Clone)]
pub struct SemanticDB {
    /// Palabras con sus categorías
    words: HashMap<String, SemanticEntry>,

    /// Temas conocidos
    themes: HashMap<String, ThemeInfo>,

    /// Relaciones semánticas (hiponimia, sinonimia, etc.)
    relations: Vec<SemanticRelation>,

    /// Reglas de compatibilidad tema-categoría
    compatibility_rules: Vec<CompatibilityRule>,
}

/// Entrada semántica para una palabra
#[derive(Debug, Clone)]
pub struct SemanticEntry {
    /// Palabra normalizada
    pub word: String,
    /// Categoría principal
    pub category: SemanticCategory,
    /// Subcategoría o especificación
    pub subcategory: Option<String>,
    /// Tags adicionales
    pub tags: Vec<String>,
    /// Palabras relacionadas
    pub related: Vec<String>,
}

/// Categoría semántica principal
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticCategory {
    /// Lugar geográfico
    Place {
        place_type: PlaceType,
        region: Option<String>,
        country: Option<String>,
    },
    /// Persona o nombre propio
    Person {
        role: Option<String>,
    },
    /// Objeto o cosa
    Object {
        object_type: ObjectType,
    },
    /// Emoción o sentimiento
    Emotion {
        valence: Valence,  // positivo, negativo, neutral
    },
    /// Concepto abstracto
    Concept {
        domain: Option<String>,
    },
    /// Acción o evento
    Action {
        action_type: ActionType,
    },
    /// Tiempo
    Time {
        time_type: TimeType,
    },
    /// Cantidad
    Quantity,
    /// Cualidad o adjetivo conceptualizado
    Quality,
    /// Desconocido
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlaceType {
    City,
    Country,
    Building,
    Monument,
    NaturalFeature,
    Region,
    Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Food,
    Plant,
    Animal,
    Artifact,
    Natural,
    Abstract,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Valence {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    Physical,
    Mental,
    Social,
    Movement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeType {
    Duration,
    Point,
    Frequency,
    Season,
}

/// Información de un tema
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    pub name: String,
    pub description: String,
    /// Categorías que son compatibles con este tema
    pub compatible_categories: Vec<CategoryMatcher>,
    /// Palabras clave que sugieren este tema
    pub keywords: Vec<String>,
}

/// Matcher para categorías compatibles
#[derive(Debug, Clone)]
pub enum CategoryMatcher {
    /// Lugar con región específica
    PlaceInRegion(String),
    /// Cualquier lugar
    AnyPlace,
    /// Objeto de tipo específico
    ObjectOfType(ObjectType),
    /// Emoción con valencia específica
    EmotionWithValence(Valence),
    /// Concepto en dominio
    ConceptInDomain(String),
    /// Cualquier categoría
    Any,
}

/// Relación semántica entre palabras
#[derive(Debug, Clone)]
pub struct SemanticRelation {
    pub word1: String,
    pub word2: String,
    pub relation_type: RelationType,
    /// Fuerza de la relación (0.0 - 1.0)
    pub strength: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationType {
    /// word1 es un tipo de word2 (perro es un animal)
    Hyponym,
    /// word1 incluye a word2 (animal incluye perro)
    Hypernym,
    /// Sinónimos
    Synonym,
    /// Antónimos
    Antonym,
    /// Parte de
    Meronym,
    /// Tiene como parte
    Holonym,
    /// Relacionado semánticamente
    Related,
}

/// Regla de compatibilidad tema-categoría
#[derive(Debug, Clone)]
pub struct CompatibilityRule {
    pub theme: String,
    pub matcher: CategoryMatcher,
    /// Score de compatibilidad si coincide (0.0 - 1.0)
    pub score: f64,
}

/// Resultado de análisis semántico
#[derive(Debug, Clone)]
pub struct SemanticAnalysis {
    /// Palabra analizada
    pub word: String,
    /// Categoría detectada
    pub category: Option<SemanticCategory>,
    /// Tema inferido del contexto
    pub inferred_theme: Option<String>,
    /// Score de compatibilidad con el contexto
    pub context_score: f64,
    /// Explicación
    pub explanation: String,
}

impl SemanticDB {
    /// Crea base de datos con vocabulario inicial
    pub fn new() -> Self {
        let mut db = Self {
            words: HashMap::new(),
            themes: HashMap::new(),
            relations: Vec::new(),
            compatibility_rules: Vec::new(),
        };

        db.load_base_vocabulary();
        db.load_themes();
        db.load_compatibility_rules();

        db
    }

    /// Carga vocabulario base
    fn load_base_vocabulary(&mut self) {
        // === LUGARES ===
        self.add_word(SemanticEntry {
            word: "roma".to_string(),
            category: SemanticCategory::Place {
                place_type: PlaceType::City,
                region: Some("italia".to_string()),
                country: Some("italia".to_string()),
            },
            subcategory: Some("capital_historica".to_string()),
            tags: vec!["arquitectura".to_string(), "historia".to_string(), "imperio_romano".to_string()],
            related: vec!["coliseo".to_string(), "vaticano".to_string(), "italia".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "coliseo".to_string(),
            category: SemanticCategory::Place {
                place_type: PlaceType::Monument,
                region: Some("italia".to_string()),
                country: Some("italia".to_string()),
            },
            subcategory: Some("anfiteatro".to_string()),
            tags: vec!["arquitectura".to_string(), "romano".to_string(), "monumento".to_string()],
            related: vec!["roma".to_string(), "gladiador".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "paris".to_string(),
            category: SemanticCategory::Place {
                place_type: PlaceType::City,
                region: Some("francia".to_string()),
                country: Some("francia".to_string()),
            },
            subcategory: Some("capital".to_string()),
            tags: vec!["romantico".to_string(), "arte".to_string()],
            related: vec!["torre_eiffel".to_string(), "louvre".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "madrid".to_string(),
            category: SemanticCategory::Place {
                place_type: PlaceType::City,
                region: Some("espana".to_string()),
                country: Some("espana".to_string()),
            },
            subcategory: Some("capital".to_string()),
            tags: vec!["espana".to_string()],
            related: vec!["prado".to_string()],
        });

        // === EMOCIONES ===
        self.add_word(SemanticEntry {
            word: "amor".to_string(),
            category: SemanticCategory::Emotion {
                valence: Valence::Positive,
            },
            subcategory: Some("afecto".to_string()),
            tags: vec!["sentimiento".to_string(), "romantico".to_string()],
            related: vec!["carino".to_string(), "querer".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "odio".to_string(),
            category: SemanticCategory::Emotion {
                valence: Valence::Negative,
            },
            subcategory: Some("aversion".to_string()),
            tags: vec!["sentimiento".to_string(), "negativo".to_string()],
            related: vec!["rencor".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "paz".to_string(),
            category: SemanticCategory::Concept {
                domain: Some("estado_social".to_string()),
            },
            subcategory: None,
            tags: vec!["positivo".to_string(), "armonia".to_string()],
            related: vec!["tranquilidad".to_string()],
        });

        // === OBJETOS ===
        self.add_word(SemanticEntry {
            word: "ramo".to_string(),
            category: SemanticCategory::Object {
                object_type: ObjectType::Plant,
            },
            subcategory: Some("flores".to_string()),
            tags: vec!["naturaleza".to_string(), "regalo".to_string()],
            related: vec!["flor".to_string(), "rosa".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "mora".to_string(),
            category: SemanticCategory::Object {
                object_type: ObjectType::Food,
            },
            subcategory: Some("fruta".to_string()),
            tags: vec!["comida".to_string(), "naturaleza".to_string()],
            related: vec!["fruta".to_string(), "zarzamora".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "casa".to_string(),
            category: SemanticCategory::Place {
                place_type: PlaceType::Building,
                region: None,
                country: None,
            },
            subcategory: Some("vivienda".to_string()),
            tags: vec!["edificio".to_string(), "hogar".to_string()],
            related: vec!["hogar".to_string(), "edificio".to_string()],
        });

        // === PERSONAS ===
        self.add_word(SemanticEntry {
            word: "rosita".to_string(),
            category: SemanticCategory::Person {
                role: None,
            },
            subcategory: Some("nombre_propio".to_string()),
            tags: vec!["femenino".to_string()],
            related: vec![],
        });

        // === CUALIDADES ===
        self.add_word(SemanticEntry {
            word: "azul".to_string(),
            category: SemanticCategory::Quality,
            subcategory: Some("color".to_string()),
            tags: vec!["color".to_string(), "frio".to_string()],
            related: vec!["celeste".to_string(), "marino".to_string()],
        });

        self.add_word(SemanticEntry {
            word: "romano".to_string(),
            category: SemanticCategory::Quality,
            subcategory: Some("gentilicio".to_string()),
            tags: vec!["roma".to_string(), "italia".to_string(), "antiguo".to_string()],
            related: vec!["roma".to_string(), "imperio".to_string()],
        });
    }

    /// Carga temas
    fn load_themes(&mut self) {
        self.themes.insert("arquitectura_romana".to_string(), ThemeInfo {
            name: "arquitectura_romana".to_string(),
            description: "Arquitectura y monumentos del Imperio Romano".to_string(),
            compatible_categories: vec![
                CategoryMatcher::PlaceInRegion("italia".to_string()),
                CategoryMatcher::ConceptInDomain("historia".to_string()),
            ],
            keywords: vec![
                "coliseo".to_string(),
                "romano".to_string(),
                "roma".to_string(),
                "imperio".to_string(),
                "gladiador".to_string(),
                "anfiteatro".to_string(),
            ],
        });

        self.themes.insert("romance".to_string(), ThemeInfo {
            name: "romance".to_string(),
            description: "Temas románticos y emocionales".to_string(),
            compatible_categories: vec![
                CategoryMatcher::EmotionWithValence(Valence::Positive),
                CategoryMatcher::ConceptInDomain("sentimiento".to_string()),
            ],
            keywords: vec![
                "amor".to_string(),
                "querer".to_string(),
                "corazon".to_string(),
                "romantico".to_string(),
            ],
        });

        self.themes.insert("naturaleza".to_string(), ThemeInfo {
            name: "naturaleza".to_string(),
            description: "Flora, fauna y elementos naturales".to_string(),
            compatible_categories: vec![
                CategoryMatcher::ObjectOfType(ObjectType::Plant),
                CategoryMatcher::ObjectOfType(ObjectType::Animal),
                CategoryMatcher::ObjectOfType(ObjectType::Natural),
            ],
            keywords: vec![
                "flor".to_string(),
                "arbol".to_string(),
                "rio".to_string(),
                "montana".to_string(),
            ],
        });

        self.themes.insert("hogar".to_string(), ThemeInfo {
            name: "hogar".to_string(),
            description: "Casa, familia, vida doméstica".to_string(),
            compatible_categories: vec![
                CategoryMatcher::AnyPlace,
                CategoryMatcher::Any,
            ],
            keywords: vec![
                "casa".to_string(),
                "familia".to_string(),
                "hogar".to_string(),
            ],
        });
    }

    /// Carga reglas de compatibilidad
    fn load_compatibility_rules(&mut self) {
        // === ARQUITECTURA ROMANA ===

        // Roma y lugares italianos son MUY compatibles con arquitectura romana
        self.compatibility_rules.push(CompatibilityRule {
            theme: "arquitectura_romana".to_string(),
            matcher: CategoryMatcher::PlaceInRegion("italia".to_string()),
            score: 0.98,  // Muy alto - Roma es perfecto para contexto romano
        });

        // Emociones son INCOMPATIBLES con arquitectura romana
        self.compatibility_rules.push(CompatibilityRule {
            theme: "arquitectura_romana".to_string(),
            matcher: CategoryMatcher::EmotionWithValence(Valence::Positive),
            score: 0.05,  // Muy bajo - amor no encaja con Coliseo
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "arquitectura_romana".to_string(),
            matcher: CategoryMatcher::EmotionWithValence(Valence::Negative),
            score: 0.05,
        });

        // Objetos naturales son poco compatibles con arquitectura
        self.compatibility_rules.push(CompatibilityRule {
            theme: "arquitectura_romana".to_string(),
            matcher: CategoryMatcher::ObjectOfType(ObjectType::Plant),
            score: 0.15,
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "arquitectura_romana".to_string(),
            matcher: CategoryMatcher::ObjectOfType(ObjectType::Food),
            score: 0.10,
        });

        // === ROMANCE ===

        // Emociones positivas son MUY compatibles con romance
        self.compatibility_rules.push(CompatibilityRule {
            theme: "romance".to_string(),
            matcher: CategoryMatcher::EmotionWithValence(Valence::Positive),
            score: 0.98,  // amor encaja perfecto en contexto romántico
        });

        // Lugares son menos compatibles con romance (a menos que sea París)
        self.compatibility_rules.push(CompatibilityRule {
            theme: "romance".to_string(),
            matcher: CategoryMatcher::PlaceInRegion("italia".to_string()),
            score: 0.30,  // Roma no encaja bien en "te quiero con todo mi ___"
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "romance".to_string(),
            matcher: CategoryMatcher::PlaceInRegion("francia".to_string()),
            score: 0.60,  // París es más romántico
        });

        // === NATURALEZA ===

        // Naturaleza es compatible con plantas
        self.compatibility_rules.push(CompatibilityRule {
            theme: "naturaleza".to_string(),
            matcher: CategoryMatcher::ObjectOfType(ObjectType::Plant),
            score: 0.90,
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "naturaleza".to_string(),
            matcher: CategoryMatcher::ObjectOfType(ObjectType::Food),
            score: 0.70,  // frutas también son naturaleza
        });

        // === GEOGRAFÍA/VIAJES ===

        self.themes.insert("viajes".to_string(), ThemeInfo {
            name: "viajes".to_string(),
            description: "Viajes y geografía".to_string(),
            compatible_categories: vec![
                CategoryMatcher::AnyPlace,
            ],
            keywords: vec![
                "viajé".to_string(),
                "visité".to_string(),
                "fui".to_string(),
                "desde".to_string(),
                "hacia".to_string(),
                "madrid".to_string(),
                "paris".to_string(),
                "roma".to_string(),
            ],
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "viajes".to_string(),
            matcher: CategoryMatcher::AnyPlace,
            score: 0.95,  // Lugares son perfectos para viajes
        });

        self.compatibility_rules.push(CompatibilityRule {
            theme: "viajes".to_string(),
            matcher: CategoryMatcher::EmotionWithValence(Valence::Positive),
            score: 0.20,  // Emociones no encajan bien en "viajé a ___"
        });
    }

    /// Añade una palabra
    pub fn add_word(&mut self, entry: SemanticEntry) {
        self.words.insert(entry.word.clone(), entry);
    }

    /// Busca información semántica de una palabra
    pub fn lookup(&self, word: &str) -> Option<&SemanticEntry> {
        self.words.get(&word.to_lowercase())
    }

    /// Infiere el tema del contexto basado en palabras
    pub fn infer_theme(&self, context_words: &[String]) -> Option<(String, f64)> {
        let mut theme_scores: HashMap<&str, f64> = HashMap::new();

        for word in context_words {
            let lower = word.to_lowercase();

            // Verificar keywords de cada tema
            for (theme_name, theme_info) in &self.themes {
                if theme_info.keywords.contains(&lower) {
                    *theme_scores.entry(theme_name.as_str()).or_insert(0.0) += 1.0;
                }
            }

            // Verificar tags de palabras conocidas
            if let Some(entry) = self.words.get(&lower) {
                for (theme_name, theme_info) in &self.themes {
                    for keyword in &theme_info.keywords {
                        if entry.tags.contains(keyword) {
                            *theme_scores.entry(theme_name.as_str()).or_insert(0.0) += 0.5;
                        }
                    }
                }
            }
        }

        // Encontrar tema con mayor score
        theme_scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(theme, score)| (theme.to_string(), score))
    }

    /// Calcula compatibilidad de una palabra con un tema
    pub fn compatibility_score(&self, word: &str, theme: &str) -> f64 {
        let entry = match self.lookup(word) {
            Some(e) => e,
            None => return 0.5,  // Palabra desconocida = neutral
        };

        let theme_info = match self.themes.get(theme) {
            Some(t) => t,
            None => return 0.5,  // Tema desconocido = neutral
        };

        // Buscar regla de compatibilidad
        for rule in &self.compatibility_rules {
            if rule.theme == theme && self.category_matches(&entry.category, &rule.matcher) {
                return rule.score;
            }
        }

        // Si no hay regla específica, verificar compatibilidad genérica
        for matcher in &theme_info.compatible_categories {
            if self.category_matches(&entry.category, matcher) {
                return 0.7;  // Compatible genéricamente
            }
        }

        // Sin match = baja compatibilidad
        0.2
    }

    /// Verifica si una categoría coincide con un matcher
    fn category_matches(&self, category: &SemanticCategory, matcher: &CategoryMatcher) -> bool {
        match (category, matcher) {
            (_, CategoryMatcher::Any) => true,

            (SemanticCategory::Place { .. }, CategoryMatcher::AnyPlace) => true,

            (SemanticCategory::Place { region: Some(r), .. }, CategoryMatcher::PlaceInRegion(expected)) => {
                r == expected
            }

            (SemanticCategory::Object { object_type }, CategoryMatcher::ObjectOfType(expected)) => {
                object_type == expected
            }

            (SemanticCategory::Emotion { valence }, CategoryMatcher::EmotionWithValence(expected)) => {
                valence == expected
            }

            (SemanticCategory::Concept { domain: Some(d) }, CategoryMatcher::ConceptInDomain(expected)) => {
                d == expected
            }

            _ => false,
        }
    }

    /// Análisis semántico completo de una palabra en contexto
    pub fn analyze(&self, word: &str, context_words: &[String]) -> SemanticAnalysis {
        let entry = self.lookup(word);
        let inferred_theme = self.infer_theme(context_words);

        let (theme_name, context_score, explanation) = match (&entry, &inferred_theme) {
            (Some(e), Some((theme, _))) => {
                let score = self.compatibility_score(word, theme);
                let exp = format!(
                    "'{}' es {:?}, tema inferido '{}', compatibilidad: {:.0}%",
                    word, e.category, theme, score * 100.0
                );
                (Some(theme.clone()), score, exp)
            }
            (Some(e), None) => {
                let exp = format!("'{}' es {:?}, sin tema claro en contexto", word, e.category);
                (None, 0.5, exp)
            }
            (None, Some((theme, _))) => {
                let exp = format!("'{}' desconocido, tema inferido '{}'", word, theme);
                (Some(theme.clone()), 0.3, exp)
            }
            (None, None) => {
                let exp = format!("'{}' desconocido, sin contexto claro", word);
                (None, 0.3, exp)
            }
        };

        SemanticAnalysis {
            word: word.to_string(),
            category: entry.map(|e| e.category.clone()),
            inferred_theme: theme_name,
            context_score,
            explanation,
        }
    }

    /// Número de palabras en la base
    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

impl Default for SemanticDB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let db = SemanticDB::new();

        let roma = db.lookup("roma").unwrap();
        assert!(matches!(roma.category, SemanticCategory::Place { .. }));

        let amor = db.lookup("amor").unwrap();
        assert!(matches!(amor.category, SemanticCategory::Emotion { .. }));
    }

    #[test]
    fn test_infer_theme() {
        let db = SemanticDB::new();

        let context = vec!["coliseo".to_string(), "romano".to_string()];
        let theme = db.infer_theme(&context);

        assert!(theme.is_some());
        assert_eq!(theme.unwrap().0, "arquitectura_romana");
    }

    #[test]
    fn test_compatibility() {
        let db = SemanticDB::new();

        // Roma es muy compatible con arquitectura_romana
        let score_roma = db.compatibility_score("roma", "arquitectura_romana");
        assert!(score_roma > 0.9);

        // Amor no es compatible con arquitectura_romana
        let score_amor = db.compatibility_score("amor", "arquitectura_romana");
        assert!(score_amor < 0.5);
    }

    #[test]
    fn test_full_analysis() {
        let db = SemanticDB::new();

        let context = vec!["coliseo".to_string(), "romano".to_string()];

        let analysis_roma = db.analyze("roma", &context);
        assert!(analysis_roma.context_score > 0.8);

        let analysis_amor = db.analyze("amor", &context);
        assert!(analysis_amor.context_score < 0.5);
    }
}
