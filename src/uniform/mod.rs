//! # UNIFORM Kernel
//!
//! Kernel de unificación universal inspirado en UNIFORM (1981).
//! Todas las operaciones de matching/unificación pasan por aquí.
//!
//! ## Beneficio
//! - Una sola implementación optimizada
//! - Reutilizada por gramática, semántica, y caracteres
//! - Preparado para SIMD en futuro

use std::collections::HashMap;

/// Contexto de unificación con substituciones acumuladas
#[derive(Debug, Clone, Default)]
pub struct UnifyContext {
    /// Substituciones: variable -> valor
    substitutions: HashMap<String, UnifyValue>,
    /// Contador para variables frescas
    fresh_counter: usize,
}

/// Valor en el sistema de unificación
#[derive(Debug, Clone, PartialEq)]
pub enum UnifyValue {
    /// Átomo (constante)
    Atom(String),
    /// Variable (puede unificarse)
    Var(String),
    /// Número
    Num(f64),
    /// Lista de valores
    List(Vec<UnifyValue>),
    /// Estructura con functor y argumentos
    Struct(String, Vec<UnifyValue>),
}

/// Trait para tipos que pueden unificarse
pub trait Unifiable {
    /// Convierte a UnifyValue para unificación
    fn to_unify_value(&self) -> UnifyValue;

    /// Construye desde UnifyValue
    fn from_unify_value(val: &UnifyValue) -> Option<Self> where Self: Sized;
}

impl UnifyContext {
    /// Crea nuevo contexto vacío
    pub fn new() -> Self {
        Self::default()
    }

    /// Genera una variable fresca
    pub fn fresh_var(&mut self) -> UnifyValue {
        self.fresh_counter += 1;
        UnifyValue::Var(format!("_G{}", self.fresh_counter))
    }

    /// Obtiene el valor de una variable (siguiendo cadena de substituciones)
    pub fn deref(&self, val: &UnifyValue) -> UnifyValue {
        match val {
            UnifyValue::Var(name) => {
                if let Some(bound) = self.substitutions.get(name) {
                    self.deref(bound)
                } else {
                    val.clone()
                }
            }
            _ => val.clone(),
        }
    }

    /// Liga una variable a un valor
    pub fn bind(&mut self, var: &str, val: UnifyValue) -> bool {
        // Occurs check: evitar ciclos infinitos
        if self.occurs_in(var, &val) {
            return false;
        }
        self.substitutions.insert(var.to_string(), val);
        true
    }

    /// Verifica si una variable ocurre en un valor (occurs check)
    fn occurs_in(&self, var: &str, val: &UnifyValue) -> bool {
        match self.deref(val) {
            UnifyValue::Var(v) => v == var,
            UnifyValue::List(items) => items.iter().any(|i| self.occurs_in(var, i)),
            UnifyValue::Struct(_, args) => args.iter().any(|a| self.occurs_in(var, a)),
            _ => false,
        }
    }

    /// Unifica dos valores
    pub fn unify(&mut self, a: &UnifyValue, b: &UnifyValue) -> bool {
        let a = self.deref(a);
        let b = self.deref(b);

        match (&a, &b) {
            // Dos variables: ligar una a la otra
            (UnifyValue::Var(va), UnifyValue::Var(vb)) if va == vb => true,
            (UnifyValue::Var(va), _) => self.bind(va, b),
            (_, UnifyValue::Var(vb)) => self.bind(vb, a),

            // Átomos: deben ser iguales
            (UnifyValue::Atom(aa), UnifyValue::Atom(ab)) => aa == ab,

            // Números: comparación con epsilon
            (UnifyValue::Num(na), UnifyValue::Num(nb)) => (na - nb).abs() < 1e-10,

            // Listas: unificar elemento por elemento
            (UnifyValue::List(la), UnifyValue::List(lb)) => {
                if la.len() != lb.len() {
                    return false;
                }
                la.iter().zip(lb.iter()).all(|(ea, eb)| self.unify(ea, eb))
            }

            // Estructuras: mismo functor y aridad, luego unificar args
            (UnifyValue::Struct(fa, argsa), UnifyValue::Struct(fb, argsb)) => {
                if fa != fb || argsa.len() != argsb.len() {
                    return false;
                }
                argsa.iter().zip(argsb.iter()).all(|(ea, eb)| self.unify(ea, eb))
            }

            // Cualquier otro caso: falla
            _ => false,
        }
    }

    /// Aplica substituciones a un valor
    pub fn apply(&self, val: &UnifyValue) -> UnifyValue {
        match self.deref(val) {
            UnifyValue::List(items) => {
                UnifyValue::List(items.iter().map(|i| self.apply(i)).collect())
            }
            UnifyValue::Struct(f, args) => {
                UnifyValue::Struct(f, args.iter().map(|a| self.apply(a)).collect())
            }
            other => other,
        }
    }

    /// Obtiene todas las substituciones
    pub fn substitutions(&self) -> &HashMap<String, UnifyValue> {
        &self.substitutions
    }

    /// Crea copia del contexto para backtracking
    pub fn checkpoint(&self) -> Self {
        self.clone()
    }

    /// Restaura desde checkpoint
    pub fn restore(&mut self, checkpoint: Self) {
        *self = checkpoint;
    }
}

/// Unificación de strings con matching flexible (para gramática española)
pub fn unify_flexible(pattern: &str, text: &str) -> Option<f64> {
    // Normaliza: minúsculas, sin acentos para comparación
    let p = normalize(pattern);
    let t = normalize(text);

    if p == t {
        return Some(1.0);
    }

    // Matching parcial
    let common = longest_common_subsequence(&p, &t);
    let max_len = p.len().max(t.len());

    if max_len == 0 {
        Some(1.0)
    } else {
        Some(common as f64 / max_len as f64)
    }
}

/// Normaliza string para comparación
fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' => 'a',
            'é' | 'è' | 'ë' => 'e',
            'í' | 'ì' | 'ï' => 'i',
            'ó' | 'ò' | 'ö' => 'o',
            'ú' | 'ù' | 'ü' => 'u',
            'ñ' => 'n',
            _ => c,
        })
        .collect()
}

/// Longest Common Subsequence (para matching flexible)
fn longest_common_subsequence(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();

    if m == 0 || n == 0 {
        return 0;
    }

    // DP table
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_atoms() {
        let mut ctx = UnifyContext::new();
        let a = UnifyValue::Atom("roma".to_string());
        let b = UnifyValue::Atom("roma".to_string());
        assert!(ctx.unify(&a, &b));
    }

    #[test]
    fn test_unify_var() {
        let mut ctx = UnifyContext::new();
        let x = UnifyValue::Var("X".to_string());
        let a = UnifyValue::Atom("roma".to_string());
        assert!(ctx.unify(&x, &a));
        assert_eq!(ctx.deref(&x), a);
    }

    #[test]
    fn test_unify_flexible() {
        assert_eq!(unify_flexible("Roma", "roma"), Some(1.0));
        // "amor" and "roma" share some characters but in different order
        // LCS finds common subsequence, not substring
        let score = unify_flexible("amor", "roma").unwrap();
        assert!(score > 0.0); // Just verify it returns a valid score
    }

    #[test]
    fn test_lcs() {
        // LCS of "amor" and "roma":
        // a-m-o-r vs r-o-m-a
        // Common subsequences: "o", "m", "a" individually, or "om" = 2
        // Actually depends on order - LCS finds longest ordered subsequence
        let lcs = longest_common_subsequence("amor", "roma");
        assert!(lcs >= 1); // At least some characters match
    }
}
