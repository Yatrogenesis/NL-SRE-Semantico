# NL-SRE-Semantico

Motor de Desambiguación Semántica Probabilística para Español

**Autor:** Francisco Molina-Burgos, Avermex Research Division
**Fecha:** Enero 2026

## Características

- **Zero dependencies** - Pure Rust
- **31 tests** pasando
- **Explicable** - Cada decisión tiene justificación detallada
- **Determinista** - Mismo input → mismo output

## Arquitectura de 4 Capas

```
┌─────────────────────────────────────────────────────┐
│ CAPA 4: UNIFORM Kernel                              │
│ → Unificación universal como meta-operador          │
├─────────────────────────────────────────────────────┤
│ CAPA 3: TAO Layer                                   │
│ → Encapsulamiento + message-passing (Smalltalk)    │
├─────────────────────────────────────────────────────┤
│ CAPA 2: APPLOG Layer                                │
│ → Variables compartidas + validación de constraints │
├─────────────────────────────────────────────────────┤
│ CAPA 1: Motores Base                                │
│ → CharMatcher, SpanishGrammar, SemanticDB          │
└─────────────────────────────────────────────────────┘
```

## Ejemplo de Uso

```
ENTRADA:  "Visité el Coliseo romano en smor"
SALIDA:   "Visité el Coliseo romano en roma"

Confianza: 77.9%
- Caracteres: 39%
- Gramática: 90%
- Contexto: 98%  ← El contexto arquitectónico romano favorece "Roma"
```

### Diferentes Contextos, Diferentes Resultados

| Contexto | Entrada | Resultado |
|----------|---------|-----------|
| Arquitectónico (Coliseo romano) | smor | roma |
| Romántico (te quiero) | smor | amor |
| Geográfico (viajé a) | smor | roma |

### Configuración de Pesos

```
Score = α·char + β·grammar + γ·context

Default: α=0.30, β=0.30, γ=0.40

Con α=0.70 (prioriza caracteres): smor → amor
Con γ=0.70 (prioriza contexto): smor → roma
```

## Gramática Española Flexible

Soporta múltiples ordenamientos válidos en español:

- **SVO**: "Juan come manzanas"
- **OVS**: "Manzanas come Juan"
- **VSO**: "Come Juan manzanas"
- **SV**: "Lupita corre"

```
✓ "Me gusta la casa azul"     → 100%
✓ "La casa azul me gusta"     → 100%
✓ "Me gusta azul la casa"     → 100%
```

## Compilación

```bash
cargo build --release
cargo test
cargo run
```

## Estructura

```
src/
├── lib.rs              # Módulo principal
├── main.rs             # Demo ejecutable
├── uniform/mod.rs      # UNIFORM kernel
├── applog/mod.rs       # APPLOG shared context
├── tao/mod.rs          # TAO message passing
├── chars/mod.rs        # Character matcher
├── grammar/mod.rs      # Spanish grammar
├── semantic/mod.rs     # Semantic database
└── disambiguator/mod.rs # Main disambiguator
```

## Fundamentos Teóricos

- **UNIFORM (1981)**: Unificación como meta-operador
- **APPLOG (1984)**: Variables compartidas LISP↔Prolog
- **TAO (1983)**: Nesting mutuo + message-passing

## Licencia

MIT
