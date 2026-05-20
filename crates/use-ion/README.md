# use-ion

Small ion identity and charge primitives for `RustUse` chemistry crates.

`use-ion` represents charged atoms and charged groups with a formula, validated
charge, optional name, optional oxidation-state label, and lightweight kind
labels. It stays structural and intentionally avoids reaction modeling,
electrochemistry simulation, acid/base behavior, naming engines, solubility
rules, and common-ion databases.

## What this crate provides

| Item                 | Purpose                                      |
| -------------------- | -------------------------------------------- |
| `Ion`                | Formula-backed charged atom or group         |
| `IonName`            | Validated optional ion name                  |
| `IonFormula`         | Ion-facing formula wrapper                   |
| `IonCharge`          | Nonzero signed ionic charge                  |
| `ChargeSign`         | Positive or negative charge sign             |
| `ChargeMagnitude`    | Nonzero charge magnitude                     |
| `Cation`             | Positive-ion wrapper                         |
| `Anion`              | Negative-ion wrapper                         |
| `MonatomicIon`       | Monatomic-ion wrapper                        |
| `PolyatomicIon`      | Polyatomic-ion wrapper                       |
| `IonKind`            | Lightweight ion classification label         |
| `IonValidationError` | Structured construction and validation error |

## Installation

```toml
[dependencies]
use-ion = "0.1.0"
```

## Quick Examples

### Create a monatomic cation

```rust
use use_chemical_formula::ChemicalFormula;
use use_ion::{Ion, IonCharge};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let sodium = Ion::new(
    ChemicalFormula::parse("Na")?,
    IonCharge::positive(1)?,
);

assert!(sodium.is_cation());
assert_eq!(sodium.charge().magnitude(), 1);
assert_eq!(sodium.to_string(), "Na+");
# Ok(())
# }
```

### Create a polyatomic anion

```rust
use use_chemical_formula::ChemicalFormula;
use use_ion::{Ion, IonCharge, IonKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let sulfate = Ion::new(
    ChemicalFormula::parse("SO4")?,
    IonCharge::negative(2)?,
)
.with_kind(IonKind::Polyatomic);

assert!(sulfate.is_anion());
assert_eq!(sulfate.to_string(), "SO4^2-");
# Ok(())
# }
```

### Use focused wrappers

```rust
use use_chemical_formula::ChemicalFormula;
use use_ion::{Cation, IonCharge, PolyatomicIon};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let calcium = Cation::new(ChemicalFormula::parse("Ca")?, 2)?;
let ammonium = PolyatomicIon::new(
    ChemicalFormula::parse("NH4")?,
    IonCharge::positive(1)?,
)?;

assert_eq!(calcium.to_string(), "Ca^2+");
assert_eq!(ammonium.to_string(), "NH4+");
# Ok(())
# }
```

## Scope

- Represents ion identity, formula, nonzero charge, optional names,
  cation/anion labels, monatomic/polyatomic labels, optional oxidation-state
  labels, and stable display formatting.
- Uses `use-chemical-formula` for formula primitives and parsing examples.
- Keeps common-ion knowledge out of the crate. Callers provide formulas,
  charges, names, and labels.
- No electrochemical potential.
- No redox reactions.
- No acid/base behavior.
- No solubility rules.
- No compound naming.
- No oxidation-state inference.
- No hardcoded common-ion database.
- No runtime network access or external chemistry data.

## Relationship to `use-chemistry`

`use-ion` is a focused child crate for ion identity and charge primitives. The
`use-chemistry` umbrella crate reexports it alongside element, formula, bond,
compound, molecule, isotope, periodic-table, atomic-number, atomic-mass, and
electron-shell helpers.
