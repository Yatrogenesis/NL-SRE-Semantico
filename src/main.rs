//! # NL-SRE-Semantico Demo
//!
//! Demostración del Motor de Desambiguación Semántica Probabilística
//!
//! ## Uso
//! ```
//! cargo run --release
//! ```
//!
//! ## Con diccionario completo RAE
//! ```
//! cargo run --release -- --full
//! ```
//!
//! ## Autor
//! Francisco Molina-Burgos, Avermex Research Division

use nl_sre_semantico::{SemanticDisambiguator, SpanishDictionary, Config, info, CommandParser};
use std::env;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     NL-SRE-SEMANTICO - Motor de Desambiguación Semántica         ║");
    println!("║     Francisco Molina-Burgos, Avermex Research Division           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", info());
    println!();

    // Check for flags
    let args: Vec<String> = env::args().collect();
    let use_full_dictionary = args.iter().any(|a| a == "--full" || a == "-f");
    let interactive_mode = args.iter().any(|a| a == "--repl" || a == "-i" || a == "--interactive");

    // Create motor
    let mut motor = if use_full_dictionary {
        load_full_motor()
    } else {
        println!("Usando diccionario básico (para diccionario completo: --full)");
        println!();
        SemanticDisambiguator::new()
    };

    // Interactive REPL mode
    if interactive_mode {
        run_repl();
        return;
    }

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

    // Show dictionary stats if external dictionary loaded
    if motor.has_external_dictionary() {
        if let Some(stats) = motor.dictionary_stats() {
            println!();
            println!("Estadísticas del diccionario RAE/LATAM:");
            println!("  - Total entradas: {}", stats.total_entries);
            println!("  - Entradas RAE: {}", stats.rae_entries);
            println!("  - Conjugaciones: {}", stats.total_conjugations);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("DEMOSTRACIÓN 5: Parser Semántico → Predicados PIRS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();

    let parser = CommandParser::new();

    // Ejemplo canónico del documento de diseño
    let command = "Requiero que me diseñes un producto que me ayude a sustituir al propofol \
                   y tiene que ser súper seguro y mucho mejor que él, más barato";

    println!("ENTRADA (Lenguaje Natural):");
    println!("  \"{}\"", command);
    println!();

    let parsed = parser.parse(command);

    println!("ANÁLISIS SEMÁNTICO:");
    println!("  Acción:      {:?}", parsed.action);
    println!("  Solicitante: {:?}", parsed.requester);
    println!("  Ejecutor:    {:?}", parsed.executor);
    println!("  Target:      {:?}", parsed.target);
    println!("  Confianza:   {:.0}%", parsed.confidence * 100.0);
    println!();

    if let Some(goal) = &parsed.goal {
        println!("  META/PROPÓSITO:");
        println!("    Acción: {}", goal.action);
        println!("    Target: {}", goal.target);
        println!();
    }

    if !parsed.constraints.is_empty() {
        println!("  RESTRICCIONES EXTRAÍDAS:");
        for c in &parsed.constraints {
            println!("    • {} {:?} → {:?}", c.attribute, c.constraint_type, c.value);
        }
        println!();
    }

    if !parsed.verbs.is_empty() {
        println!("  VERBOS ANALIZADOS:");
        for v in &parsed.verbs {
            println!("    • \"{}\" ({}) → {:?} persona, modo {:?}",
                v.conjugated, v.lemma, v.person, v.mode);
        }
        println!();
    }

    println!("SALIDA PIRS (Predicados Prolog):");
    println!("─────────────────────────────────────────");
    println!("{}", parsed.to_prolog_string());
    println!("─────────────────────────────────────────");

    // Segundo ejemplo: imperativo directo
    println!();
    println!("EJEMPLO 2: Imperativo directo");
    println!();

    let command2 = "Ayúdame a encontrar una alternativa más económica";
    println!("ENTRADA: \"{}\"", command2);
    let parsed2 = parser.parse(command2);
    println!();
    println!("SALIDA PIRS:");
    println!("{}", parsed2.to_prolog_string());

    // Tercer ejemplo: búsqueda
    println!();
    println!("EJEMPLO 3: Búsqueda con restricciones");
    println!();

    let command3 = "Busco información sobre compuestos más seguros que el fentanilo";
    println!("ENTRADA: \"{}\"", command3);
    let parsed3 = parser.parse(command3);
    println!();
    println!("SALIDA PIRS:");
    println!("{}", parsed3.to_prolog_string());

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    FIN DE LA DEMOSTRACIÓN                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

/// Carga el motor con diccionario completo RAE
fn load_full_motor() -> SemanticDisambiguator {
    println!("Cargando diccionario completo RAE/LATAM...");
    println!();

    // Try different data paths
    let possible_paths = [
        "data",
        "./data",
        "C:\\Users\\pakom\\NL-SRE-Semantico\\data",
    ];

    for data_path in possible_paths {
        let path = Path::new(data_path);
        if path.exists() {
            match SpanishDictionary::load_from_directory(path) {
                Ok(dict) => {
                    println!("Diccionario cargado exitosamente desde: {}", data_path);
                    println!("  - Palabras válidas: {}", dict.len());
                    println!("  - Entradas RAE: {}", dict.stats.rae_entries);
                    println!("  - Conjugaciones: {}", dict.stats.total_conjugations);
                    println!();
                    return SemanticDisambiguator::with_dictionary(dict);
                }
                Err(e) => {
                    println!("Error cargando desde {}: {}", data_path, e);
                }
            }
        }
    }

    // Fallback to basic dictionary
    println!("No se encontró diccionario completo, usando básico");
    println!();
    SemanticDisambiguator::new()
}

/// REPL interactivo para testing del parser semántico
fn run_repl() {
    println!("═══════════════════════════════════════════════════════════════════");
    println!("     NL-SRE-SEMANTICO :: REPL INTERACTIVO");
    println!("     Escribe comandos en español → genera predicados PIRS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("Comandos especiales:");
    println!("  /salir, /exit, /q  - Terminar");
    println!("  /ayuda, /help      - Mostrar ayuda");
    println!("  /verbose           - Toggle modo detallado");
    println!();

    let parser = CommandParser::new();
    let mut verbose = false;

    loop {
        // Prompt
        print!("NL> ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(_) => break,
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Handle special commands
        match input.to_lowercase().as_str() {
            "/salir" | "/exit" | "/q" | "salir" | "exit" => {
                println!("¡Hasta luego!");
                break;
            }
            "/ayuda" | "/help" => {
                print_repl_help();
                continue;
            }
            "/verbose" => {
                verbose = !verbose;
                println!("Modo verbose: {}", if verbose { "ON" } else { "OFF" });
                continue;
            }
            _ => {}
        }

        // Parse the command
        let parsed = parser.parse(input);

        // Output
        if verbose {
            println!();
            println!("┌─ ANÁLISIS ─────────────────────────────────────────────────────");
            println!("│ Acción:      {:?}", parsed.action);
            println!("│ Target:      {:?}", parsed.target);
            println!("│ Confianza:   {:.0}%", parsed.confidence * 100.0);
            if let Some(goal) = &parsed.goal {
                println!("│ Meta:        {}({})", goal.action, goal.target);
            }
            if !parsed.constraints.is_empty() {
                println!("│ Restricciones:");
                for c in &parsed.constraints {
                    println!("│   • {} {:?}", c.attribute, c.constraint_type);
                }
            }
            if !parsed.verbs.is_empty() {
                println!("│ Verbos:");
                for v in &parsed.verbs {
                    println!("│   • {} → {:?} pers, {:?}", v.conjugated, v.person, v.mode);
                }
            }
            println!("└─────────────────────────────────────────────────────────────────");
        }

        println!();
        println!("PIRS>");
        println!("{}", parsed.to_prolog_string());
    }
}

/// Muestra ayuda del REPL
fn print_repl_help() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    NL-SRE-SEMANTICO AYUDA                        ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Este sistema convierte español natural → predicados PIRS        ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ EJEMPLOS DE ENTRADA:                                            ║");
    println!("║   • Requiero un producto que sustituya al propofol              ║");
    println!("║   • Ayúdame a encontrar alternativas más baratas                ║");
    println!("║   • Busco información sobre compuestos seguros                  ║");
    println!("║   • Necesito algo mejor que el fentanilo                        ║");
    println!("║   • Diseña una molécula más estable                             ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ VERBOS RECONOCIDOS:                                             ║");
    println!("║   Solicitud: requiero, quiero, necesito, busco, pido            ║");
    println!("║   Delegación: ayúdame, diseña, crea, encuentra, analiza         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ RESTRICCIONES:                                                  ║");
    println!("║   Superlativo: súper, muy, extremadamente                       ║");
    println!("║   Comparativo: más X que, mejor que, más barato                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
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

    #[test]
    fn test_load_full_dictionary() {
        // This test will only pass if data directory exists
        let data_path = Path::new("data");
        if data_path.exists() {
            let dict = SpanishDictionary::load_from_directory(data_path);
            assert!(dict.is_ok());
            let dict = dict.unwrap();
            assert!(dict.len() > 0);
        }
    }
}
