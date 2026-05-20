# use-reaction

Small chemical reaction representation primitives for `RustUse` chemistry crates.

`use-reaction` represents reaction equations, reaction-side terms, arrows,
conditions, catalysts, solvents, and lightweight classification labels. It stays
structural and avoids balancing, reaction solving, kinetics, thermodynamics,
equilibrium calculations, mechanisms, electrochemistry, reaction databases, and
external data fetching.

## What this crate provides

| Item                      | Purpose                                              |
| ------------------------- | ---------------------------------------------------- |
| `ChemicalReaction`        | Reaction value with equation, conditions, and kinds  |
| `ReactionEquation`        | Reactants, products, and arrow                       |
| `ReactionTerm`            | Formula-backed term with stoichiometric coefficient  |
| `Reactant`                | Reactant-side term wrapper                           |
| `Product`                 | Product-side term wrapper                            |
| `ReactionArrow`           | Forward, reverse, reversible, and equilibrium arrow  |
| `ReactionDirection`       | Lightweight direction label                          |
| `ReactionCondition`       | Temperature, pressure, catalyst, solvent, and labels |
| `ReactionConditionSet`    | Ordered reaction condition collection                |
| `Catalyst`                | Validated catalyst descriptor                        |
| `Solvent`                 | Validated solvent descriptor                         |
| `ReactionKind`            | Lightweight reaction classification label            |
| `ReactionValidationError` | Structured construction and validation errors        |

## Installation

```toml
[dependencies]
use-reaction = "0.1.0"
```

## Quick Examples

### Create a simple reaction

```rust
use use_reaction::{ChemicalReaction, ReactionArrow, ReactionTerm};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let reaction = ChemicalReaction::new()
    .with_reactant(ReactionTerm::new("H2".parse()?).with_coefficient(2)?)
    .with_reactant(ReactionTerm::new("O2".parse()?))
    .with_product(ReactionTerm::new("H2O".parse()?).with_coefficient(2)?)
    .with_arrow(ReactionArrow::Forward);

assert_eq!(reaction.to_string(), "2H2 + O2 -> 2H2O");
reaction.validate()?;
# Ok(())
# }
```

### Attach reaction labels

```rust
use use_reaction::{ChemicalReaction, ReactionCondition, ReactionKind, ReactionTerm};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let reaction = ChemicalReaction::new()
    .with_reactant(ReactionTerm::new("CH4".parse()?))
    .with_reactant(ReactionTerm::new("O2".parse()?).with_coefficient(2)?)
    .with_product(ReactionTerm::new("CO2".parse()?))
    .with_product(ReactionTerm::new("H2O".parse()?).with_coefficient(2)?)
    .with_kind(ReactionKind::Combustion)
    .with_condition(ReactionCondition::Heat);

assert_eq!(reaction.to_string(), "CH4 + 2O2 -> CO2 + 2H2O");
assert_eq!(reaction.kinds(), &[ReactionKind::Combustion]);
assert_eq!(reaction.conditions().to_string(), "heat");
# Ok(())
# }
```

## Scope

- Represents reaction equations, reactants, products, arrows, conditions,
  catalysts, solvents, energy labels, and classification labels.
- Uses `use-chemical-formula` for chemical formula primitives.
- Uses `use-stoichiometry` for nonzero coefficient validation and term display.
- Validates that complete reactions have at least one reactant and one product.
- Omits coefficient `1` in conventional display formatting.
- Keeps conditions as labels and descriptors only.
- No automatic equation balancing.
- No product inference.
- No reaction mechanisms.
- No kinetics, rate laws, or thermodynamics.
- No equilibrium calculations.
- No electrochemical potentials.
- No hardcoded reaction database.
- No runtime network access or external chemistry data.

## Relationship to `use-chemistry`

`use-reaction` is a focused child crate for chemical reaction representation
primitives. The `use-chemistry` umbrella crate reexports it alongside
stoichiometry, formula, compound, molecule, ion, oxidation-state, bond, element,
isotope, periodic-table, atomic-number, atomic-mass, and electron-shell helpers.
