//! # Dictionary Module
//!
//! Carga y maneja diccionarios de español (RAE, mexicanismos, variantes LATAM)
//!
//! ## Fuentes soportadas:
//! - RAE Corpus (85,811 palabras)
//! - Frequency data (70K lemas con frecuencias)
//! - Wiktionary ES (873,990 entradas)
//! - Mexicanismos
//! - Variantes LATAM

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Entrada de diccionario con metadata completa
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    /// Palabra normalizada (minúsculas, sin acentos)
    pub word: String,
    /// Palabra original con acentos
    pub original: String,
    /// Parte del discurso (noun, verb, adj, etc.)
    pub pos: Vec<PartOfSpeech>,
    /// Definiciones
    pub definitions: Vec<String>,
    /// Frecuencia de uso (mayor = más común)
    pub frequency: u64,
    /// Región/variante (RAE, MX, AR, etc.)
    pub region: Region,
    /// Categoría semántica inferida
    pub semantic_category: Option<String>,
}

/// Partes del discurso
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartOfSpeech {
    Noun,       // sustantivo
    Verb,       // verbo
    Adjective,  // adjetivo
    Adverb,     // adverbio
    Preposition,// preposición
    Article,    // artículo
    Pronoun,    // pronombre
    Conjunction,// conjunción
    Interjection,// interjección
    Prefix,     // prefijo
    Suffix,     // sufijo
    Unknown,
}

impl PartOfSpeech {
    /// Parse desde string de RAE corpus
    pub fn from_rae_str(s: &str) -> Vec<Self> {
        let mut result = Vec::new();
        let lower = s.to_lowercase();

        if lower.contains("sust") || lower.contains("m.") || lower.contains("f.")
           || lower.contains(" m") || lower.contains(" f") {
            result.push(PartOfSpeech::Noun);
        }
        if lower.contains("verb") || lower.contains(" v") || lower.contains("tr.")
           || lower.contains("intr.") || lower.contains("prnl.") {
            result.push(PartOfSpeech::Verb);
        }
        if lower.contains("adj") {
            result.push(PartOfSpeech::Adjective);
        }
        if lower.contains("adv") {
            result.push(PartOfSpeech::Adverb);
        }
        if lower.contains("prep") {
            result.push(PartOfSpeech::Preposition);
        }
        if lower.contains("art") {
            result.push(PartOfSpeech::Article);
        }
        if lower.contains("pron") {
            result.push(PartOfSpeech::Pronoun);
        }
        if lower.contains("conj") {
            result.push(PartOfSpeech::Conjunction);
        }
        if lower.contains("interj") {
            result.push(PartOfSpeech::Interjection);
        }
        if lower.contains("pref") {
            result.push(PartOfSpeech::Prefix);
        }
        if lower.contains("suf") {
            result.push(PartOfSpeech::Suffix);
        }

        if result.is_empty() {
            result.push(PartOfSpeech::Unknown);
        }

        result
    }
}

/// Región/variante del español
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    /// Español estándar (RAE)
    Standard,
    /// España
    Spain,
    /// México
    Mexico,
    /// Argentina
    Argentina,
    /// Colombia
    Colombia,
    /// Perú
    Peru,
    /// Chile
    Chile,
    /// Venezuela
    Venezuela,
    /// Cuba
    Cuba,
    /// Uruguay
    Uruguay,
    /// Centroamérica
    CentralAmerica,
    /// Otro/desconocido
    Other(String),
}

/// Diccionario completo del español
#[derive(Debug)]
pub struct SpanishDictionary {
    /// Entradas por palabra normalizada
    entries: HashMap<String, Vec<DictionaryEntry>>,
    /// Set de palabras válidas (para búsqueda rápida)
    valid_words: HashSet<String>,
    /// Frecuencias de palabras
    frequencies: HashMap<String, u64>,
    /// Formas conjugadas -> lema
    conjugations: HashMap<String, String>,
    /// Estadísticas
    pub stats: DictionaryStats,
}

/// Estadísticas del diccionario
#[derive(Debug, Clone, Default)]
pub struct DictionaryStats {
    pub total_entries: usize,
    pub rae_entries: usize,
    pub mexican_entries: usize,
    pub latam_entries: usize,
    pub total_conjugations: usize,
}

impl SpanishDictionary {
    /// Crear diccionario vacío
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            valid_words: HashSet::new(),
            frequencies: HashMap::new(),
            conjugations: HashMap::new(),
            stats: DictionaryStats::default(),
        }
    }

    /// Cargar desde directorio de datos
    pub fn load_from_directory<P: AsRef<Path>>(data_dir: P) -> Result<Self, DictionaryError> {
        let mut dict = Self::new();
        let data_path = data_dir.as_ref();

        // Cargar RAE corpus
        let rae_path = data_path.join("rae").join("rae_corpus.json");
        if rae_path.exists() {
            dict.load_rae_corpus(&rae_path)?;
        }

        // Cargar frecuencias
        let freq_path = data_path.join("rae").join("frequency.csv");
        if freq_path.exists() {
            dict.load_frequency_csv(&freq_path)?;
        }

        Ok(dict)
    }

    /// Cargar RAE corpus JSON
    fn load_rae_corpus<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DictionaryError> {
        let file = File::open(path.as_ref())
            .map_err(|e| DictionaryError::IoError(e.to_string()))?;
        let reader = BufReader::new(file);

        // Parse JSON manualmente (sin serde para zero-deps)
        let content: String = reader.lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<_>>()
            .join("\n");

        // Simple JSON parsing
        let entries = parse_rae_json(&content)?;

        for entry in entries {
            let normalized = normalize_word(&entry.word);
            self.valid_words.insert(normalized.clone());
            self.entries.entry(normalized).or_insert_with(Vec::new).push(entry);
            self.stats.rae_entries += 1;
        }

        self.stats.total_entries = self.valid_words.len();
        Ok(())
    }

    /// Cargar frequency CSV
    fn load_frequency_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DictionaryError> {
        let file = File::open(path.as_ref())
            .map_err(|e| DictionaryError::IoError(e.to_string()))?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        // Skip header
        let _ = lines.next();

        for line in lines {
            let line = line.map_err(|e| DictionaryError::IoError(e.to_string()))?;
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() >= 5 {
                if let Ok(count) = parts[0].parse::<u64>() {
                    let word = parts[1].to_string();
                    let normalized = normalize_word(&word);

                    self.frequencies.insert(normalized.clone(), count);
                    self.valid_words.insert(normalized.clone());

                    // Parse conjugations
                    if parts.len() >= 5 {
                        let usage = parts[4];
                        for form_count in usage.split('|') {
                            if let Some(colon_pos) = form_count.find(':') {
                                let form = &form_count[colon_pos + 1..];
                                let normalized_form = normalize_word(form);
                                if !normalized_form.is_empty() && normalized_form != normalized {
                                    self.conjugations.insert(normalized_form.clone(), normalized.clone());
                                    self.valid_words.insert(normalized_form);
                                    self.stats.total_conjugations += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.stats.total_entries = self.valid_words.len();
        Ok(())
    }

    /// Verificar si una palabra es válida
    pub fn is_valid(&self, word: &str) -> bool {
        let normalized = normalize_word(word);
        self.valid_words.contains(&normalized)
    }

    /// Obtener frecuencia de una palabra
    pub fn frequency(&self, word: &str) -> u64 {
        let normalized = normalize_word(word);
        // Primero buscar directamente
        if let Some(&freq) = self.frequencies.get(&normalized) {
            return freq;
        }
        // Buscar si es conjugación
        if let Some(lemma) = self.conjugations.get(&normalized) {
            if let Some(&freq) = self.frequencies.get(lemma) {
                return freq;
            }
        }
        0
    }

    /// Obtener lema de una forma conjugada
    pub fn get_lemma(&self, word: &str) -> Option<String> {
        let normalized = normalize_word(word);
        if self.frequencies.contains_key(&normalized) {
            return Some(normalized);
        }
        self.conjugations.get(&normalized).cloned()
    }

    /// Obtener entradas de una palabra
    pub fn get_entries(&self, word: &str) -> Vec<&DictionaryEntry> {
        let normalized = normalize_word(word);
        self.entries.get(&normalized)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Obtener todas las palabras válidas (para CharMatcher)
    pub fn all_words(&self) -> impl Iterator<Item = &String> {
        self.valid_words.iter()
    }

    /// Número total de palabras
    pub fn len(&self) -> usize {
        self.valid_words.len()
    }

    /// Verificar si está vacío
    pub fn is_empty(&self) -> bool {
        self.valid_words.is_empty()
    }

    /// Agregar palabra manualmente
    pub fn add_word(&mut self, word: &str, pos: Vec<PartOfSpeech>, region: Region) {
        let normalized = normalize_word(word);
        let entry = DictionaryEntry {
            word: normalized.clone(),
            original: word.to_string(),
            pos,
            definitions: Vec::new(),
            frequency: 1,
            region,
            semantic_category: None,
        };
        self.valid_words.insert(normalized.clone());
        self.entries.entry(normalized).or_insert_with(Vec::new).push(entry);
        self.stats.total_entries = self.valid_words.len();
    }
}

impl Default for SpanishDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Errores del diccionario
#[derive(Debug)]
pub enum DictionaryError {
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::IoError(s) => write!(f, "IO Error: {}", s),
            DictionaryError::ParseError(s) => write!(f, "Parse Error: {}", s),
        }
    }
}

impl std::error::Error for DictionaryError {}

/// Normaliza una palabra (minúsculas, sin acentos)
pub fn normalize_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            _ => c,
        })
        .filter(|c| c.is_alphabetic())
        .collect()
}

/// Parser simple de JSON para RAE corpus (sin serde)
fn parse_rae_json(content: &str) -> Result<Vec<DictionaryEntry>, DictionaryError> {
    let mut entries = Vec::new();

    // Estado del parser
    let mut in_object = false;
    let mut current_word = String::new();
    let mut current_pos = String::new();
    let mut current_defs: Vec<String> = Vec::new();
    let mut current_key = String::new();
    let mut in_string = false;
    let mut in_array = false;
    let mut string_buffer = String::new();
    let mut escape_next = false;

    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if escape_next {
            if in_string {
                string_buffer.push(c);
            }
            escape_next = false;
            i += 1;
            continue;
        }

        if c == '\\' {
            escape_next = true;
            i += 1;
            continue;
        }

        if c == '"' && !escape_next {
            if in_string {
                // Fin de string
                in_string = false;

                if current_key.is_empty() {
                    current_key = string_buffer.clone();
                } else {
                    match current_key.as_str() {
                        "word" => current_word = string_buffer.clone(),
                        "pos" => current_pos = string_buffer.clone(),
                        _ => {
                            if in_array && current_key == "definitions" {
                                current_defs.push(string_buffer.clone());
                            }
                        }
                    }
                }
                string_buffer.clear();
            } else {
                // Inicio de string
                in_string = true;
            }
            i += 1;
            continue;
        }

        if in_string {
            string_buffer.push(c);
            i += 1;
            continue;
        }

        match c {
            '{' => {
                in_object = true;
                current_word.clear();
                current_pos.clear();
                current_defs.clear();
                current_key.clear();
            }
            '}' => {
                if in_object && !current_word.is_empty() {
                    let normalized = normalize_word(&current_word);
                    entries.push(DictionaryEntry {
                        word: normalized,
                        original: current_word.clone(),
                        pos: PartOfSpeech::from_rae_str(&current_pos),
                        definitions: current_defs.clone(),
                        frequency: 1,
                        region: Region::Standard,
                        semantic_category: None,
                    });
                }
                in_object = false;
                current_key.clear();
            }
            '[' => {
                in_array = true;
            }
            ']' => {
                in_array = false;
            }
            ':' => {
                // Key ya está en current_key
            }
            ',' => {
                if !in_array {
                    current_key.clear();
                }
            }
            _ => {}
        }

        i += 1;
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize_word("España"), "espana");
        assert_eq!(normalize_word("ÁRBOL"), "arbol");
        assert_eq!(normalize_word("niño"), "nino");
    }

    #[test]
    fn test_pos_parsing() {
        let pos = PartOfSpeech::from_rae_str("m. f. adj.");
        assert!(pos.contains(&PartOfSpeech::Noun));
        assert!(pos.contains(&PartOfSpeech::Adjective));
    }

    #[test]
    fn test_dictionary_basic() {
        let mut dict = SpanishDictionary::new();
        dict.add_word("casa", vec![PartOfSpeech::Noun], Region::Standard);
        assert!(dict.is_valid("casa"));
        assert!(dict.is_valid("Casa"));
        assert!(!dict.is_valid("xyz"));
    }
}
