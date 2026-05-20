# use-chemical-formula

Small chemical formula primitives for `RustUse` chemistry crates.

`use-chemical-formula` represents structural formulas such as `H2O`, `CO2`,
`NaCl`, `Ca(OH)2`, `Al2(SO4)3`, and `CuSO4·5H2O`. It provides lightweight
parsing, display formatting, and expanded element counts without becoming a
chemistry engine.

## What this crate provides

| Item                     | Purpose                                             |
| ------------------------ | --------------------------------------------------- |
| `ChemicalFormula`        | Parsed chemical formula with optional hydrate parts |
| `FormulaPart`            | A contiguous formula part                           |
| `FormulaTerm`            | An element term or grouped term                     |
| `ElementSymbol`          | Basic chemical-symbol shape validation              |
| `ElementCount`           | Positive element count                              |
| `FormulaGroup`           | Parenthesized formula terms with a multiplier       |
| `FormulaMultiplier`      | Positive group or hydrate multiplier                |
| `HydratePart`            | Dot-separated hydrate formula part                  |
| `FormulaParseError`      | Structured parse errors                             |
| `FormulaValidationError` | Structured construction and validation errors       |

## Installation

```toml
[dependencies]
use-chemical-formula = "0.1.0"
```

## Quick Examples

### Parse a grouped formula

```rust
use use_chemical_formula::ChemicalFormula;

# fn main() -> Result<(), use_chemical_formula::FormulaParseError> {
let formula = ChemicalFormula::parse("Ca(OH)2")?;
let counts = formula.element_counts();

assert_eq!(formula.to_string(), "Ca(OH)2");
assert_eq!(counts.get("Ca"), Some(&1));
assert_eq!(counts.get("O"), Some(&2));
assert_eq!(counts.get("H"), Some(&2));
# Ok(())
# }
```

### Parse a hydrate formula

```rust
use use_chemical_formula::ChemicalFormula;

# fn main() -> Result<(), use_chemical_formula::FormulaParseError> {
let formula = ChemicalFormula::parse("CuSO4.5H2O")?;
let counts = formula.element_counts();

assert_eq!(formula.to_string(), "CuSO4·5H2O");
assert_eq!(counts.get("Cu"), Some(&1));
assert_eq!(counts.get("S"), Some(&1));
assert_eq!(counts.get("O"), Some(&9));
assert_eq!(counts.get("H"), Some(&10));
# Ok(())
# }
```

## Scope

- Represents formula structure with elements, counts, groups, and hydrate parts.
- Parses common formula notation with `.` or `·` hydrate separators.
- Expands total element counts after group and hydrate multipliers.
- Validates basic element-symbol shape only.
- No periodic-table database validation.
- No molecular geometry, bonding, reactions, stoichiometry, molar mass, pH,
  solutions, thermochemistry, or simulation.
- No parser-framework ambitions; the parser is intentionally small and direct.

## Relationship to `use-chemistry`

`use-chemical-formula` is a focused child crate for formula primitives. The
`use-chemistry` umbrella crate reexports it alongside element, isotope,
periodic-table, atomic-number, atomic-mass, and electron-shell helpers.
