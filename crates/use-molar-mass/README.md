# use-molar-mass

<p align="center">
  <strong>Molar mass primitives for formula-level chemistry calculations.</strong><br>
  Small validated values and lookup-backed calculations built on RustUse formula and atomic-mass crates.
</p>

## Surface

| Item                      | Purpose                                                    |
| ------------------------- | ---------------------------------------------------------- |
| `MolarMass`               | Positive finite molar mass value with a display unit       |
| `MolarMassUnit`           | Supported molar-mass units, currently `g/mol` and `kg/mol` |
| `AtomicMassEntry`         | Validated element symbol and atomic mass pair              |
| `AtomicMassLookup`        | Caller-controlled lookup table for formula calculations    |
| `MolarMassCalculation`    | Formula plus lookup calculation helper                     |
| `FormulaMolarMass`        | Calculated molar mass with per-element contributions       |
| `ElementMassContribution` | Atomic mass, count, and contribution for one element       |

## Example

```rust
use std::error::Error;

use use_chemical_formula::ChemicalFormula;
use use_molar_mass::{AtomicMassEntry, AtomicMassLookup, MolarMassCalculation};

fn main() -> Result<(), Box<dyn Error>> {
    let formula = ChemicalFormula::parse("H2O")?;
    let lookup = AtomicMassLookup::from_entries([
        AtomicMassEntry::new("H", 1.008)?,
        AtomicMassEntry::new("O", 15.999)?,
    ]);

    let calculation = MolarMassCalculation::new(formula, lookup);
    let water = calculation.calculate()?;

    assert_eq!(water.molar_mass().to_string(), "18.015 g/mol");
    assert_eq!(water.contributions().as_slice()[0].to_string(), "H: 2 × 1.008 = 2.016 g/mol");

    Ok(())
}
```

## Standard atomic mass helper

Use `with_standard_atomic_masses()` when the existing RustUse atomic-mass table is the desired data source.

```rust
use std::error::Error;

use use_chemical_formula::ChemicalFormula;
use use_molar_mass::MolarMassCalculation;

fn main() -> Result<(), Box<dyn Error>> {
    let water = MolarMassCalculation::with_standard_atomic_masses(
        ChemicalFormula::parse("H2O")?,
    )?
    .calculate()?;

    assert!((water.molar_mass().value() - 18.015).abs() < 0.001);

    Ok(())
}
```

## Scope

`use-molar-mass` calculates formula molar masses from element counts and atomic masses. It does not balance reactions, model isotopic abundance, fetch external reference data, or provide a general dimensional-analysis system.

## Installation

```toml
[dependencies]
use-molar-mass = "0.1.0"
```

For a single umbrella dependency with the chemistry prelude, use `use-chemistry`.
