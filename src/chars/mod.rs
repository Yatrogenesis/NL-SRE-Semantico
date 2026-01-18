//! # Character Matching Module
//!
//! Matching probabilístico a nivel de caracteres.
//! Implementa la idea del usuario:
//! - Cada letra tiene un peso (100% / longitud)
//! - Letras coincidentes suman su peso
//! - También considera posición y conjunto de caracteres

use std::collections::{HashMap, HashSet};

/// Motor de matching de caracteres
#[derive(Debug, Clone)]
pub struct CharMatcher {
    /// Diccionario de palabras válidas
    dictionary: HashSet<String>,

    /// Índice invertido: letra -> palabras que la contienen
    letter_index: HashMap<char, Vec<String>>,

    /// Configuración
    config: CharMatchConfig,
}

/// Configuración del matcher
#[derive(Debug, Clone)]
pub struct CharMatchConfig {
    /// Peso para coincidencia de conjunto de caracteres (Jaccard)
    pub weight_jaccard: f64,

    /// Peso para coincidencia posicional
    pub weight_positional: f64,

    /// Peso para longitud similar
    pub weight_length: f64,

    /// Peso para Levenshtein normalizado
    pub weight_levenshtein: f64,

    /// Número máximo de candidatos a retornar
    pub max_candidates: usize,

    /// Umbral mínimo de similitud
    pub min_similarity: f64,
}

impl Default for CharMatchConfig {
    fn default() -> Self {
        Self {
            weight_jaccard: 0.40,      // Aumentado - prioriza conjunto de caracteres
            weight_positional: 0.15,   // Reducido - menos importante para anagramas
            weight_length: 0.15,
            weight_levenshtein: 0.30,
            max_candidates: 15,        // Más candidatos para considerar
            min_similarity: 0.25,      // Reducido - permite más candidatos semánticos
        }
    }
}

/// Resultado de matching
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// Palabra candidata
    pub word: String,
    /// Score total (0.0 - 1.0)
    pub score: f64,
    /// Desglose de scores
    pub breakdown: ScoreBreakdown,
}

/// Desglose de scores individuales
#[derive(Debug, Clone)]
pub struct ScoreBreakdown {
    /// Score Jaccard (conjunto de caracteres)
    pub jaccard: f64,
    /// Score posicional
    pub positional: f64,
    /// Score de longitud
    pub length: f64,
    /// Score Levenshtein
    pub levenshtein: f64,
}

impl CharMatcher {
    /// Crea matcher vacío
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::new(),
            letter_index: HashMap::new(),
            config: CharMatchConfig::default(),
        }
    }

    /// Crea con configuración personalizada
    pub fn with_config(config: CharMatchConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Carga diccionario de palabras
    pub fn load_dictionary<I: IntoIterator<Item = S>, S: AsRef<str>>(&mut self, words: I) {
        for word in words {
            let w = word.as_ref().to_lowercase();
            self.add_word(&w);
        }
    }

    /// Añade una palabra al diccionario
    pub fn add_word(&mut self, word: &str) {
        let normalized = normalize_word(word);

        if normalized.is_empty() {
            return;
        }

        // Añadir al diccionario
        self.dictionary.insert(normalized.clone());

        // Actualizar índice invertido
        for c in normalized.chars() {
            self.letter_index
                .entry(c)
                .or_insert_with(Vec::new)
                .push(normalized.clone());
        }
    }

    /// Verifica si una palabra está en el diccionario
    pub fn is_valid(&self, word: &str) -> bool {
        let normalized = normalize_word(word);
        self.dictionary.contains(&normalized)
    }

    /// Encuentra candidatos para una palabra (posiblemente mal escrita)
    pub fn find_candidates(&self, input: &str) -> Vec<MatchResult> {
        let normalized = normalize_word(input);

        if normalized.is_empty() {
            return Vec::new();
        }

        // Si ya está en diccionario, retornar con score 1.0
        if self.dictionary.contains(&normalized) {
            return vec![MatchResult {
                word: normalized,
                score: 1.0,
                breakdown: ScoreBreakdown {
                    jaccard: 1.0,
                    positional: 1.0,
                    length: 1.0,
                    levenshtein: 1.0,
                },
            }];
        }

        // Buscar candidatos usando índice invertido
        let input_chars: HashSet<char> = normalized.chars().collect();
        let mut candidate_scores: HashMap<String, usize> = HashMap::new();

        // Contar cuántas letras comparte cada palabra
        for c in &input_chars {
            if let Some(words) = self.letter_index.get(c) {
                for word in words {
                    *candidate_scores.entry(word.clone()).or_insert(0) += 1;
                }
            }
        }

        // Filtrar candidatos con al menos 50% de letras compartidas
        let min_shared = (input_chars.len() as f64 * 0.5).ceil() as usize;
        let candidates: Vec<_> = candidate_scores
            .into_iter()
            .filter(|(_, count)| *count >= min_shared.max(1))
            .map(|(word, _)| word)
            .collect();

        // Calcular scores para cada candidato
        let mut results: Vec<MatchResult> = candidates
            .iter()
            .map(|candidate| self.calculate_score(&normalized, candidate))
            .filter(|r| r.score >= self.config.min_similarity)
            .collect();

        // Ordenar por score descendente
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Limitar cantidad
        results.truncate(self.config.max_candidates);

        results
    }

    /// Calcula score de similitud entre dos palabras
    fn calculate_score(&self, input: &str, candidate: &str) -> MatchResult {
        let breakdown = ScoreBreakdown {
            jaccard: jaccard_similarity(input, candidate),
            positional: positional_similarity(input, candidate),
            length: length_similarity(input, candidate),
            levenshtein: levenshtein_similarity(input, candidate),
        };

        let score = self.config.weight_jaccard * breakdown.jaccard
            + self.config.weight_positional * breakdown.positional
            + self.config.weight_length * breakdown.length
            + self.config.weight_levenshtein * breakdown.levenshtein;

        MatchResult {
            word: candidate.to_string(),
            score,
            breakdown,
        }
    }

    /// Calcula similitud según la idea del usuario:
    /// Cada letra = 100%/longitud. Letras coincidentes suman su peso.
    pub fn user_method_similarity(&self, input: &str, candidate: &str) -> f64 {
        let input_chars: Vec<char> = input.chars().collect();
        let candidate_chars: Vec<char> = candidate.chars().collect();

        if input_chars.is_empty() {
            return if candidate_chars.is_empty() { 1.0 } else { 0.0 };
        }

        // Peso por letra del input
        let weight_per_char = 1.0 / input_chars.len() as f64;

        let mut total_score = 0.0;
        let mut candidate_used: Vec<bool> = vec![false; candidate_chars.len()];

        // Para cada caracter del input, buscar coincidencia en candidate
        for input_char in &input_chars {
            // Primero buscar coincidencia exacta en posición
            for (j, &cand_char) in candidate_chars.iter().enumerate() {
                if !candidate_used[j] && *input_char == cand_char {
                    total_score += weight_per_char;
                    candidate_used[j] = true;
                    break;
                }
            }
        }

        total_score
    }

    /// Tamaño del diccionario
    pub fn dictionary_size(&self) -> usize {
        self.dictionary.len()
    }
}

impl Default for CharMatcher {
    fn default() -> Self {
        Self::new()
    }
}

// === Funciones de similitud ===

/// Similitud Jaccard: |A ∩ B| / |A ∪ B|
fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();

    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Similitud posicional: letras en misma posición
fn positional_similarity(a: &str, b: &str) -> f64 {
    let chars_a: Vec<char> = a.chars().collect();
    let chars_b: Vec<char> = b.chars().collect();

    let max_len = chars_a.len().max(chars_b.len());
    if max_len == 0 {
        return 1.0;
    }

    let matches = chars_a
        .iter()
        .zip(chars_b.iter())
        .filter(|(ca, cb)| ca == cb)
        .count();

    matches as f64 / max_len as f64
}

/// Similitud de longitud
fn length_similarity(a: &str, b: &str) -> f64 {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let max_len = len_a.max(len_b);
    if max_len == 0 {
        return 1.0;
    }

    let diff = (len_a as i32 - len_b as i32).unsigned_abs() as usize;
    1.0 - (diff as f64 / max_len as f64)
}

/// Similitud basada en Levenshtein normalizada
fn levenshtein_similarity(a: &str, b: &str) -> f64 {
    let distance = levenshtein_distance(a, b);
    let max_len = a.chars().count().max(b.chars().count());

    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f64 / max_len as f64)
    }
}

/// Distancia de Levenshtein
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let m = a.len();
    let n = b.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Matriz de DP
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)        // deletion
                .min(dp[i][j - 1] + 1)           // insertion
                .min(dp[i - 1][j - 1] + cost);   // substitution
        }
    }

    dp[m][n]
}

/// Normaliza una palabra: minúsculas, sin acentos
fn normalize_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n', // Mantener ñ como n para matching flexible
            _ => c,
        })
        .filter(|c| c.is_alphabetic())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard() {
        // "amor" vs "roma" comparten {a, m, o, r} = 100%
        assert!((jaccard_similarity("amor", "roma") - 1.0).abs() < 0.001);

        // "smor" vs "amor": smor={s,m,o,r}, amor={a,m,o,r}
        // Intersección: {m,o,r} = 3, Unión: {s,m,o,r,a} = 5
        // Jaccard = 3/5 = 0.6
        assert!((jaccard_similarity("smor", "amor") - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        // amor → roma requires: a→r, m→o, o→m, r→a = multiple operations
        // Actually: amor → aomr (swap m,o) → roma (swap a,r)
        // Levenshtein counts substitutions, not swaps
        assert!(levenshtein_distance("amor", "roma") >= 2);
        assert_eq!(levenshtein_distance("smor", "amor"), 1); // s→a
    }

    #[test]
    fn test_user_method() {
        let matcher = CharMatcher::new();

        // "smor" vs "amor": s≠a, m=m, o=o, r=r → 3/4 = 0.75
        let score = matcher.user_method_similarity("smor", "amor");
        assert!((score - 0.75).abs() < 0.001);

        // "amor" vs "amor": perfecto
        let score = matcher.user_method_similarity("amor", "amor");
        assert!((score - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_find_candidates() {
        let mut matcher = CharMatcher::new();
        matcher.load_dictionary(vec!["amor", "roma", "ramo", "mora", "omar", "armo"]);

        let candidates = matcher.find_candidates("smor");
        assert!(!candidates.is_empty());

        // Debe encontrar "amor" como candidato
        assert!(candidates.iter().any(|c| c.word == "amor"));
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize_word("Ámor"), "amor");
        assert_eq!(normalize_word("ROMA"), "roma");
        assert_eq!(normalize_word("España"), "espana");
    }
}
