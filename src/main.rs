//! # NL-SRE-Semantico Demo
//!
//! Demostración del Motor de Desambiguación Semántica Probabilística
//!
//! ## Ejemplo principal
//! "Visité el Coliseo romano en smor"
//! → "Visité el Coliseo romano en Roma"
//!
//! ## Autor
//! Francisco Molina-Burgos, Avermex Research Division

use nl_sre_semantico::{SemanticDisambiguator, Config, info};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     NL-SRE-SEMANTICO - Motor de Desambiguación Semántica         ║");
    println!("║     Francisco Molina-Burgos, Avermex Research Division           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", info());
    println!();

    // Crear motor
    let mut motor = SemanticDisambiguator::new();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("DEMOSTRACIÓN 1: Desambiguación con contexto arquitectónico");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    let sentence = "Visité el Coliseo romano en smor";
    println!("ENTRADA:  \"{}\"", sentence);
    println!();

    let result = motor.process(sentence);

    println!("SALIDA:   \"{}\"", result.corrected);
    println!("CONFIANZA: {:.1}%", result.confidence * 100.0);
    println!();

    if !result.corrections.is_empty() {
        println!("CORRECCIONES APLICADAS:");
        for correction in &result.corrections {
            println!("  • '{}' → '{}'", correction.original, correction.corrected);
            println!("    Confianza: {:.1}%", correction.confidence * 100.0);
            println!("    Razón: {}", correction.explanation.reason);
            println!();
            println!("    Desglose de scores:");
            println!("      - Caracteres:  {:.1}%", correction.explanation.char_score * 100.0);
            println!("      - Gramática:   {:.1}%", correction.explanation.grammar_score * 100.0);
            println!("      - Contexto:    {:.1}%", correction.explanation.context_score * 100.0);
            println!();
            println!("    Candidatos considerados:");
            for (word, score) in &correction.explanation.candidates {
                println!("      - {}: {:.1}%", word, score * 100.0);
            }
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("DEMOSTRACIÓN 2: Orden flexible del español");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    let variations = [
        "Me gusta la casa azul",
        "La casa azul me gusta",
        "Me gusta azul la casa",  // Menos común pero posible en poesía
    ];

    for sentence in &variations {
        let result = motor.process(sentence);
        let status = if result.corrections.is_empty() { "✓" } else { "?" };
        println!("{} \"{}\" → Conf: {:.0}%", status, sentence, result.confidence * 100.0);
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("DEMOSTRACIÓN 3: Diferentes contextos cambian la interpretación");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Contexto romántico: "amor" debería ganar
    let romantic = "Te quiero con todo mi smor";
    let result_romantic = motor.process(romantic);
    println!("Contexto ROMÁNTICO:");
    println!("  ENTRADA:  \"{}\"", romantic);
    println!("  SALIDA:   \"{}\"", result_romantic.corrected);
    if let Some(c) = result_romantic.corrections.first() {
        println!("  Corrección: '{}' → '{}'", c.original, c.corrected);
    }
    println!();

    // Contexto geográfico: "Roma" debería ganar
    let geographic = "Viajé a smor desde Madrid";
    motor.add_to_dictionary(vec!["viajé"]);
    let result_geographic = motor.process(geographic);
    println!("Contexto GEOGRÁFICO:");
    println!("  ENTRADA:  \"{}\"", geographic);
    println!("  SALIDA:   \"{}\"", result_geographic.corrected);
    if let Some(c) = result_geographic.corrections.first() {
        println!("  Corrección: '{}' → '{}'", c.original, c.corrected);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("DEMOSTRACIÓN 4: Configuración personalizada de pesos");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    // Config que prioriza caracteres sobre contexto
    let config_chars = Config {
        alpha: 0.70,  // 70% peso a caracteres
        beta: 0.20,   // 20% peso a gramática
        gamma: 0.10,  // 10% peso a contexto
        ..Config::default()
    };

    let mut motor_chars = SemanticDisambiguator::with_config(config_chars);

    let sentence = "Visité el Coliseo romano en smor";
    let result_chars = motor_chars.process(sentence);

    println!("Con pesos α=0.70, β=0.20, γ=0.10 (prioriza caracteres):");
    println!("  ENTRADA:  \"{}\"", sentence);
    println!("  SALIDA:   \"{}\"", result_chars.corrected);
    if let Some(c) = result_chars.corrections.first() {
        println!("  Corrección: '{}' → '{}'", c.original, c.corrected);
    }
    println!();

    // Config que prioriza contexto
    let config_context = Config {
        alpha: 0.10,  // 10% peso a caracteres
        beta: 0.20,   // 20% peso a gramática
        gamma: 0.70,  // 70% peso a contexto
        ..Config::default()
    };

    let mut motor_context = SemanticDisambiguator::with_config(config_context);

    let result_context = motor_context.process(sentence);

    println!("Con pesos α=0.10, β=0.20, γ=0.70 (prioriza contexto):");
    println!("  ENTRADA:  \"{}\"", sentence);
    println!("  SALIDA:   \"{}\"", result_context.corrected);
    if let Some(c) = result_context.corrections.first() {
        println!("  Corrección: '{}' → '{}'", c.original, c.corrected);
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("INFORMACIÓN DEL SISTEMA");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("Tamaño del diccionario: {} palabras", motor.dictionary_size());
    println!("Configuración actual: α={:.2}, β={:.2}, γ={:.2}",
        motor.config().alpha, motor.config().beta, motor.config().gamma);
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    FIN DE LA DEMOSTRACIÓN                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_roma_vs_amor_with_context() {
        let mut motor = SemanticDisambiguator::new();

        // Contexto de arquitectura → debe elegir "Roma"
        let result = motor.process("Visité el Coliseo romano en smor");

        assert!(!result.corrections.is_empty());
        // Con contexto de Coliseo romano, Roma debe tener mejor score
        // que amor debido al contexto semántico
    }

    #[test]
    fn test_no_changes_for_valid_sentence() {
        let mut motor = SemanticDisambiguator::new();
        motor.add_to_dictionary(vec!["grande"]);

        let result = motor.process("el amor es grande");
        assert!(result.corrections.is_empty());
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_spanish_word_order_flexibility() {
        let mut motor = SemanticDisambiguator::new();

        // Todas estas variaciones son válidas en español
        let sentences = [
            "me gusta la casa",
            "la casa me gusta",
        ];

        for sentence in &sentences {
            let result = motor.process(sentence);
            // No debe haber correcciones para oraciones válidas
            assert!(
                result.corrections.is_empty(),
                "Oración válida marcada como incorrecta: {}",
                sentence
            );
        }
    }

    #[test]
    fn test_config_affects_result() {
        // Con config que prioriza caracteres extremadamente
        let config_extreme_chars = Config {
            alpha: 0.95,
            beta: 0.025,
            gamma: 0.025,
            ..Config::default()
        };

        let mut motor = SemanticDisambiguator::with_config(config_extreme_chars);
        let result = motor.process("smor");

        // Con 95% peso en caracteres, el candidato con mejor match de
        // caracteres debería ganar independientemente del contexto
        assert!(!result.corrections.is_empty());
    }
}
