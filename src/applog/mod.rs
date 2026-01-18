//! # APPLOG Layer
//!
//! Variables compartidas entre motores con validación de constraints.
//! Inspirado en APPLOG (1984, UC Berkeley).
//!
//! ## Seguridad
//! Toda regla o dato compartido pasa por validación antes de aceptarse.
//! Esto previene "improvisación de lógica" peligrosa.

use std::collections::HashMap;
use crate::uniform::UnifyValue;

/// Contexto compartido entre todos los componentes del sistema
#[derive(Debug, Clone)]
pub struct SharedContext {
    /// Variables compartidas (bindings)
    bindings: HashMap<String, SharedValue>,

    /// Reglas activas (pueden ser modificadas dinámicamente)
    rules: Vec<SharedRule>,

    /// Validador de constraints
    validator: ConstraintValidator,

    /// Historial de cambios para rollback
    history: Vec<ContextChange>,

    /// Modo estricto: rechaza cualquier violación
    strict_mode: bool,
}

/// Valor compartido con metadatos
#[derive(Debug, Clone)]
pub struct SharedValue {
    /// El valor propiamente
    pub value: UnifyValue,
    /// Quién lo escribió
    pub source: Source,
    /// Nivel de confianza (0.0 - 1.0)
    pub confidence: f64,
    /// Timestamp de creación
    pub created_at: u64,
    /// Es inmutable después de crearse?
    pub immutable: bool,
}

/// Fuente de un valor o regla
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// Sistema base (reglas iniciales)
    System,
    /// Motor de gramática
    Grammar,
    /// Motor semántico
    Semantic,
    /// Desambiguador
    Disambiguator,
    /// Usuario (entrada directa)
    User,
    /// Improvisación (requiere validación extra)
    Improvised,
}

/// Regla compartida
#[derive(Debug, Clone)]
pub struct SharedRule {
    /// Identificador único
    pub id: String,
    /// Nombre del predicado
    pub predicate: String,
    /// Aridad
    pub arity: usize,
    /// Cuerpo de la regla en formato interno
    pub body: RuleBody,
    /// Fuente
    pub source: Source,
    /// Confianza
    pub confidence: f64,
    /// Nivel de flexibilidad para improvisación (0.0 = rígida, 1.0 = muy flexible)
    pub flexibility: f64,
}

/// Cuerpo de una regla
#[derive(Debug, Clone)]
pub enum RuleBody {
    /// Hecho (siempre verdadero)
    Fact,
    /// Conjunción de goals
    Conjunction(Vec<Goal>),
    /// Disyunción de goals
    Disjunction(Vec<Goal>),
}

/// Un goal dentro de una regla
#[derive(Debug, Clone)]
pub struct Goal {
    pub predicate: String,
    pub args: Vec<UnifyValue>,
}

/// Cambio en el contexto (para historial/rollback)
#[derive(Debug, Clone)]
enum ContextChange {
    BindingAdded(String),
    BindingModified(String, SharedValue),
    RuleAdded(String),
    RuleRemoved(String, SharedRule),
}

/// Validador de constraints
#[derive(Debug, Clone)]
pub struct ConstraintValidator {
    /// Constraints que nunca pueden violarse (invariantes)
    invariants: Vec<Constraint>,

    /// Predicados que requieren evidencia mínima
    evidence_required: HashMap<String, usize>,

    /// Predicados protegidos (no modificables por improvisación)
    protected_predicates: Vec<String>,
}

/// Un constraint
#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: String,
    pub check: ConstraintType,
}

/// Tipos de constraints
#[derive(Debug, Clone)]
pub enum ConstraintType {
    /// No puede existir cierto predicado
    Forbidden(String),

    /// Debe existir al menos N instancias
    MinInstances(String, usize),

    /// No puede haber contradicciones
    NoContradiction(String, String),

    /// Aridad fija
    FixedArity(String, usize),

    /// Regla no puede ser tautología
    NoTautology,

    /// Custom check
    Custom(String),
}

/// Error de validación
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvariantViolation(String),
    InsufficientEvidence(String, usize, usize),
    ProtectedPredicate(String),
    TautologyDetected,
    ContradictionDetected(String),
    ArityMismatch(String, usize, usize),
    ImmutableBinding(String),
    UnauthorizedSource(Source),
}

impl SharedContext {
    /// Crea nuevo contexto con configuración por defecto
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            rules: Vec::new(),
            validator: ConstraintValidator::default(),
            history: Vec::new(),
            strict_mode: true,
        }
    }

    /// Crea con validador personalizado
    pub fn with_validator(validator: ConstraintValidator) -> Self {
        Self {
            validator,
            ..Self::new()
        }
    }

    // === BINDINGS ===

    /// Lee un binding
    pub fn get(&self, key: &str) -> Option<&SharedValue> {
        self.bindings.get(key)
    }

    /// Escribe un binding con validación
    pub fn set(
        &mut self,
        key: &str,
        value: UnifyValue,
        source: Source,
        confidence: f64,
    ) -> Result<(), ValidationError> {
        // Verificar si existe y es inmutable
        if let Some(existing) = self.bindings.get(key) {
            if existing.immutable {
                return Err(ValidationError::ImmutableBinding(key.to_string()));
            }
            // Guardar para rollback
            self.history.push(ContextChange::BindingModified(
                key.to_string(),
                existing.clone(),
            ));
        } else {
            self.history.push(ContextChange::BindingAdded(key.to_string()));
        }

        // Validar según fuente
        if source == Source::Improvised && self.strict_mode {
            // Improvisaciones requieren alta confianza
            if confidence < 0.8 {
                return Err(ValidationError::UnauthorizedSource(source));
            }
        }

        let shared = SharedValue {
            value,
            source,
            confidence,
            created_at: timestamp_now(),
            immutable: false,
        };

        self.bindings.insert(key.to_string(), shared);
        Ok(())
    }

    /// Escribe binding inmutable (solo una vez)
    pub fn set_immutable(
        &mut self,
        key: &str,
        value: UnifyValue,
        source: Source,
    ) -> Result<(), ValidationError> {
        if self.bindings.contains_key(key) {
            return Err(ValidationError::ImmutableBinding(key.to_string()));
        }

        self.history.push(ContextChange::BindingAdded(key.to_string()));

        let shared = SharedValue {
            value,
            source,
            confidence: 1.0,
            created_at: timestamp_now(),
            immutable: true,
        };

        self.bindings.insert(key.to_string(), shared);
        Ok(())
    }

    // === RULES ===

    /// Añade una regla con validación completa
    pub fn add_rule(&mut self, rule: SharedRule) -> Result<(), ValidationError> {
        // Validar contra constraints
        self.validator.validate_rule(&rule, &self.rules)?;

        self.history.push(ContextChange::RuleAdded(rule.id.clone()));
        self.rules.push(rule);
        Ok(())
    }

    /// Busca reglas por predicado
    pub fn find_rules(&self, predicate: &str) -> Vec<&SharedRule> {
        self.rules.iter().filter(|r| r.predicate == predicate).collect()
    }

    /// Busca reglas por predicado y aridad
    pub fn find_rules_exact(&self, predicate: &str, arity: usize) -> Vec<&SharedRule> {
        self.rules
            .iter()
            .filter(|r| r.predicate == predicate && r.arity == arity)
            .collect()
    }

    // === ROLLBACK ===

    /// Crea checkpoint
    pub fn checkpoint(&self) -> usize {
        self.history.len()
    }

    /// Rollback a checkpoint
    pub fn rollback(&mut self, checkpoint: usize) {
        while self.history.len() > checkpoint {
            if let Some(change) = self.history.pop() {
                match change {
                    ContextChange::BindingAdded(key) => {
                        self.bindings.remove(&key);
                    }
                    ContextChange::BindingModified(key, old) => {
                        self.bindings.insert(key, old);
                    }
                    ContextChange::RuleAdded(id) => {
                        self.rules.retain(|r| r.id != id);
                    }
                    ContextChange::RuleRemoved(id, rule) => {
                        // Re-insertar con mismo id
                        self.rules.push(SharedRule { id, ..rule });
                    }
                }
            }
        }
    }

    // === QUERIES ===

    /// Obtiene todos los bindings de una fuente
    pub fn bindings_from(&self, source: Source) -> Vec<(&String, &SharedValue)> {
        self.bindings
            .iter()
            .filter(|(_, v)| v.source == source)
            .collect()
    }

    /// Obtiene todas las reglas de una fuente
    pub fn rules_from(&self, source: Source) -> Vec<&SharedRule> {
        self.rules.iter().filter(|r| r.source == source).collect()
    }
}

impl Default for SharedContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstraintValidator {
    /// Crea validador con constraints por defecto
    pub fn default() -> Self {
        let mut validator = Self {
            invariants: Vec::new(),
            evidence_required: HashMap::new(),
            protected_predicates: Vec::new(),
        };

        // Constraints básicos
        validator.add_invariant(Constraint {
            name: "no_tautology".to_string(),
            check: ConstraintType::NoTautology,
        });

        validator
    }

    /// Añade invariante
    pub fn add_invariant(&mut self, constraint: Constraint) {
        self.invariants.push(constraint);
    }

    /// Marca predicado como protegido
    pub fn protect_predicate(&mut self, predicate: &str) {
        self.protected_predicates.push(predicate.to_string());
    }

    /// Requiere evidencia mínima para un predicado
    pub fn require_evidence(&mut self, predicate: &str, min_count: usize) {
        self.evidence_required.insert(predicate.to_string(), min_count);
    }

    /// Valida una regla antes de añadirla
    pub fn validate_rule(
        &self,
        rule: &SharedRule,
        existing: &[SharedRule],
    ) -> Result<(), ValidationError> {
        // 1. Verificar predicados protegidos (solo si es improvisación)
        if rule.source == Source::Improvised {
            if self.protected_predicates.contains(&rule.predicate) {
                return Err(ValidationError::ProtectedPredicate(rule.predicate.clone()));
            }
        }

        // 2. Verificar invariantes
        for inv in &self.invariants {
            match &inv.check {
                ConstraintType::NoTautology => {
                    if is_tautology(rule) {
                        return Err(ValidationError::TautologyDetected);
                    }
                }
                ConstraintType::Forbidden(pred) => {
                    if &rule.predicate == pred {
                        return Err(ValidationError::InvariantViolation(inv.name.clone()));
                    }
                }
                ConstraintType::FixedArity(pred, expected) => {
                    if &rule.predicate == pred && rule.arity != *expected {
                        return Err(ValidationError::ArityMismatch(
                            pred.clone(),
                            *expected,
                            rule.arity,
                        ));
                    }
                }
                ConstraintType::NoContradiction(pred1, pred2) => {
                    // Verificar que no contradice reglas existentes
                    if &rule.predicate == pred1 {
                        for ex in existing {
                            if &ex.predicate == pred2 {
                                // Simplificado: detectar contradicción obvia
                                return Err(ValidationError::ContradictionDetected(
                                    format!("{} vs {}", pred1, pred2),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // 3. Verificar evidencia requerida
        if let Some(min) = self.evidence_required.get(&rule.predicate) {
            let count = existing.iter().filter(|r| r.predicate == rule.predicate).count();
            if count < *min && rule.source == Source::Improvised {
                return Err(ValidationError::InsufficientEvidence(
                    rule.predicate.clone(),
                    *min,
                    count,
                ));
            }
        }

        Ok(())
    }
}

/// Verifica si una regla es tautología (ej: p(X) :- p(X))
fn is_tautology(rule: &SharedRule) -> bool {
    match &rule.body {
        RuleBody::Fact => false,
        RuleBody::Conjunction(goals) => {
            // Tautología si el único goal es igual a la cabeza
            if goals.len() == 1 {
                let goal = &goals[0];
                if goal.predicate == rule.predicate && goal.args.len() == rule.arity {
                    // Simplificado: si mismo predicado y aridad, probablemente tautología
                    return true;
                }
            }
            false
        }
        RuleBody::Disjunction(_) => false,
    }
}

/// Genera timestamp simple (segundos desde epoch simplificado)
fn timestamp_now() -> u64 {
    // Sin dependencias externas, usamos 0 o un contador
    // En implementación real, usar std::time
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_context_basic() {
        let mut ctx = SharedContext::new();

        ctx.set("tema", UnifyValue::Atom("arquitectura".to_string()), Source::Semantic, 0.9)
            .unwrap();

        let val = ctx.get("tema").unwrap();
        assert_eq!(val.confidence, 0.9);
    }

    #[test]
    fn test_immutable_binding() {
        let mut ctx = SharedContext::new();

        ctx.set_immutable("const", UnifyValue::Num(42.0), Source::System).unwrap();

        // Intentar modificar debe fallar
        let result = ctx.set("const", UnifyValue::Num(0.0), Source::User, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rollback() {
        let mut ctx = SharedContext::new();

        let cp = ctx.checkpoint();

        ctx.set("temp", UnifyValue::Atom("valor".to_string()), Source::User, 0.5)
            .unwrap();

        assert!(ctx.get("temp").is_some());

        ctx.rollback(cp);

        assert!(ctx.get("temp").is_none());
    }

    #[test]
    fn test_tautology_detection() {
        let rule = SharedRule {
            id: "test".to_string(),
            predicate: "p".to_string(),
            arity: 1,
            body: RuleBody::Conjunction(vec![Goal {
                predicate: "p".to_string(),
                args: vec![UnifyValue::Var("X".to_string())],
            }]),
            source: Source::Improvised,
            confidence: 0.9,
            flexibility: 0.5,
        };

        assert!(is_tautology(&rule));
    }
}
