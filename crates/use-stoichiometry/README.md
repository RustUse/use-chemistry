# use-stoichiometry

Small stoichiometry primitives for `RustUse` chemistry crates.

`use-stoichiometry` represents stoichiometric coefficients, formula terms,
mole ratios, reaction-side entries, reagent labels, and simple yield values. It
stays structural and intentionally avoids equation balancing, product
inference, reaction kinetics, thermochemistry, equilibrium calculations,
electrochemistry, lab automation, and reaction databases.

## What this crate provides

| Item                           | Purpose                                      |
| ------------------------------ | -------------------------------------------- |
| `StoichiometricCoefficient`    | Nonzero stoichiometric coefficient           |
| `StoichiometricTerm`           | Coefficient plus formula                     |
| `StoichiometricRatio`          | Coefficient ratio                            |
| `MoleRatio`                    | Mole-ratio wrapper                           |
| `ReactionSide`                 | Reactant or product side label               |
| `ReactionEntry`                | Formula entry on one reaction side           |
| `ReactantEntry`                | Reactant-side entry wrapper                  |
| `ProductEntry`                 | Product-side entry wrapper                   |
| `FormulaQuantity`              | Formula plus stoichiometric coefficient      |
| `LimitingReagent`              | Validated limiting-reagent label             |
| `ExcessReagent`                | Validated excess-reagent label               |
| `TheoreticalYield`             | Positive finite theoretical yield value      |
| `ActualYield`                  | Nonnegative finite actual yield value        |
| `PercentYield`                 | Nonnegative finite percent yield value       |
| `StoichiometryValidationError` | Structured construction and validation error |

## Installation

```toml
[dependencies]
use-stoichiometry = "0.1.0"
```

## Quick Examples

### Create a reaction entry

```rust
use use_chemical_formula::ChemicalFormula;
use use_stoichiometry::{ReactionEntry, ReactionSide, StoichiometricCoefficient};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let water = ReactionEntry::new(
    StoichiometricCoefficient::new(2)?,
    ChemicalFormula::parse("H2O")?,
    ReactionSide::Product,
)?;

assert_eq!(water.coefficient().value(), 2);
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water.side(), ReactionSide::Product);
assert_eq!(water.to_string(), "2H2O");
# Ok(())
# }
```

### Create a mole ratio

```rust
use use_stoichiometry::{MoleRatio, StoichiometricCoefficient};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let ratio = MoleRatio::new(
    StoichiometricCoefficient::new(2)?,
    StoichiometricCoefficient::new(1)?,
)?;

assert_eq!(ratio.numerator().value(), 2);
assert_eq!(ratio.denominator().value(), 1);
assert_eq!(ratio.to_string(), "2:1");
# Ok(())
# }
```

### Calculate percent yield

```rust
use use_stoichiometry::PercentYield;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let yield_percent = PercentYield::from_actual_and_theoretical(8.0, 10.0)?;

assert_eq!(yield_percent.value(), 80.0);
assert_eq!(yield_percent.to_string(), "80%");
# Ok(())
# }
```

## Scope

- Represents stoichiometric coefficients, formula terms, mole ratios,
  reaction-side labels, formula quantities, limiting-reagent labels,
  excess-reagent labels, theoretical yield values, actual yield values,
  percent yield values, and stable display formatting.
- Uses `use-chemical-formula` for formula primitives and parsing examples.
- Keeps all chemistry knowledge caller-provided. Callers choose formulas,
  coefficients, sides, labels, and yield values.
- No chemical equation balancing.
- No product inference from reactants.
- No reaction kinetics.
- No thermochemistry.
- No equilibrium calculations.
- No electrochemistry.
- No unit-heavy mass/mole conversion.
- No hardcoded reaction database.
- No runtime network access or external chemistry data.
- No lab automation toolkit behavior.

## Relationship to `use-chemistry`

`use-stoichiometry` is a focused child crate for stoichiometry primitives. The
`use-chemistry` umbrella crate reexports it alongside formula, compound,
molecule, ion, oxidation-state, bond, element, isotope, periodic-table,
atomic-number, atomic-mass, and electron-shell helpers.
