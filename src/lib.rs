//! # NL-SRE-Semantico
//!
//! Motor de Desambiguación Semántica Probabilística para Español
//!
//! ## Arquitectura de Capas
//!
//! ```text
//! CAPA 4: UNIFORM (Kernel de Unificación)
//! CAPA 3: TAO (Encapsulamiento + Mensajes)
//! CAPA 2: APPLOG (Variables Compartidas + Constraints)
//! CAPA 1: Motores Base (Grammar + Semantic)
//! ```
//!
//! ## Autor
//! Francisco Molina-Burgos, Avermex Research Division
//!
//! ## Fecha
//! Enero 2026

pub mod uniform;
pub mod applog;
pub mod tao;
pub mod grammar;
pub mod semantic;
pub mod disambiguator;
pub mod chars;

// Re-exports principales
pub use disambiguator::SemanticDisambiguator;
pub use uniform::UnifyContext;
pub use applog::SharedContext;
pub use grammar::SpanishGrammar;
pub use semantic::{SemanticDB, SemanticCategory};

/// Resultado de procesamiento de una oración
#[derive(Debug, Clone)]
pub struct ProcessedSentence {
    /// Oración original
    pub original: String,
    /// Oración corregida
    pub corrected: String,
    /// Confianza global (0.0 - 1.0)
    pub confidence: f64,
    /// Correcciones individuales aplicadas
    pub corrections: Vec<Correction>,
}

/// Una corrección individual
#[derive(Debug, Clone)]
pub struct Correction {
    /// Posición en la oración (índice de token)
    pub position: usize,
    /// Palabra original (posiblemente errónea)
    pub original: String,
    /// Palabra corregida
    pub corrected: String,
    /// Confianza de esta corrección
    pub confidence: f64,
    /// Explicación de por qué se eligió esta corrección
    pub explanation: CorrectionExplanation,
}

/// Explicación detallada de una corrección
#[derive(Debug, Clone)]
pub struct CorrectionExplanation {
    /// Score de similitud de caracteres
    pub char_score: f64,
    /// Score gramatical
    pub grammar_score: f64,
    /// Score de contexto semántico
    pub context_score: f64,
    /// Candidatos considerados con sus scores
    pub candidates: Vec<(String, f64)>,
    /// Razón en texto legible
    pub reason: String,
}

/// Configuración del motor
#[derive(Debug, Clone)]
pub struct Config {
    /// Peso para similitud de caracteres (α)
    pub alpha: f64,
    /// Peso para validación gramatical (β)
    pub beta: f64,
    /// Peso para contexto semántico (γ)
    pub gamma: f64,
    /// Umbral mínimo de confianza para aceptar corrección
    pub min_confidence: f64,
    /// Número máximo de candidatos a considerar
    pub max_candidates: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alpha: 0.30,  // 30% peso a caracteres
            beta: 0.30,   // 30% peso a gramática
            gamma: 0.40,  // 40% peso a contexto semántico
            min_confidence: 0.60,
            max_candidates: 10,
        }
    }
}

/// Versión del motor
pub const VERSION: &str = "0.1.0";

/// Información del motor
pub fn info() -> String {
    format!(
        "NL-SRE-Semantico v{}\n\
         Motor de Desambiguación Semántica Probabilística\n\
         Autor: Francisco Molina-Burgos, Avermex Research Division\n\
         Zero dependencies - Pure Rust",
        VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert!((cfg.alpha + cfg.beta + cfg.gamma - 1.0).abs() < 0.001);
    }
}
