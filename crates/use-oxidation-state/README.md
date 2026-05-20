# use-oxidation-state

Small oxidation-state primitives for `RustUse` chemistry crates.

`use-oxidation-state` represents oxidation-state values, signs, magnitudes,
Roman numeral labels, element assignments, formula-context assignments, and
small assignment sets. It stays structural and intentionally avoids redox
solving, reaction modeling, electrochemistry simulation, compound naming,
oxidation-state inference, and common oxidation-state databases.

## What this crate provides

| Item                            | Purpose                                      |
| ------------------------------- | -------------------------------------------- |
| `OxidationState`                | Validated signed oxidation-state value       |
| `OxidationSign`                 | Positive, negative, or zero sign             |
| `OxidationMagnitude`            | Bounded oxidation-state magnitude            |
| `RomanOxidationState`           | Roman numeral display for common magnitudes  |
| `ElementOxidationState`         | Element-symbol oxidation-state assignment    |
| `FormulaOxidationState`         | Formula/context assignment group             |
| `OxidationStateAssignment`      | Label plus oxidation-state entry             |
| `OxidationStateSet`             | Small insertion-order assignment collection  |
| `OxidationStateValidationError` | Structured construction and validation error |

## Installation

```toml
[dependencies]
use-oxidation-state = "0.1.0"
```

## Quick Examples

### Create oxidation states

```rust
use use_oxidation_state::OxidationState;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let iron_ii = OxidationState::positive(2)?;
let oxygen = OxidationState::negative(2)?;
let elemental = OxidationState::zero();

assert_eq!(iron_ii.to_string(), "+2");
assert_eq!(oxygen.to_string(), "-2");
assert_eq!(elemental.to_string(), "0");
assert_eq!(iron_ii.to_roman(), Some(String::from("II")));
# Ok(())
# }
```

### Display element oxidation states

```rust
use use_oxidation_state::{ElementOxidationState, OxidationState};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let iron_iii = ElementOxidationState::new("Fe", OxidationState::positive(3)?)?;
let copper_i = ElementOxidationState::new("Cu", OxidationState::positive(1)?)?;

assert_eq!(iron_iii.to_string(), "Fe(III)");
assert_eq!(copper_i.to_string(), "Cu(I)");
# Ok(())
# }
```

### Store formula-context entries

```rust
use use_oxidation_state::{
    FormulaOxidationState, OxidationState, OxidationStateAssignment, OxidationStateSet,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut states = OxidationStateSet::new();
states.insert(OxidationStateAssignment::new("Fe", OxidationState::positive(3)?)?);
states.insert(OxidationStateAssignment::new("O", OxidationState::negative(2)?)?);

let hematite = FormulaOxidationState::new("Fe2O3", states)?;

assert_eq!(hematite.formula_label(), "Fe2O3");
assert_eq!(hematite.states().len(), 2);
assert_eq!(hematite.to_string(), "Fe2O3 [Fe: +3, O: -2]");
# Ok(())
# }
```

## Scope

- Represents oxidation-state signs, magnitudes, zero states, positive states,
  negative states, Roman numeral labels, element assignments, formula-context
  assignments, and stable display formatting.
- Keeps all chemistry knowledge caller-provided. Callers choose symbols,
  labels, formulas, and state assignments.
- Uses no runtime network access and no external chemistry data.
- No redox balancing.
- No reaction logic.
- No oxidation-state inference from formulas.
- No compound naming.
- No electrochemical potentials.
- No hardcoded common oxidation-state table.
- No full chemistry toolkit behavior.

## Relationship to `use-chemistry`

`use-oxidation-state` is a focused child crate for oxidation-state primitives.
The `use-chemistry` umbrella crate reexports it alongside element, formula,
bond, ion, compound, molecule, isotope, periodic-table, atomic-number,
atomic-mass, and electron-shell helpers.
