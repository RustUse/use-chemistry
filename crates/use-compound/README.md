# use-compound

Small chemical compound identity primitives for `RustUse` chemistry crates.

`use-compound` represents named chemical substances composed from formulas and
lightweight descriptors. It is intentionally structural: no compound database,
no naming engine, no reactions, no molecular geometry, no bonding model, and no
runtime data fetching.

## What this crate provides

| Item                      | Purpose                                       |
| ------------------------- | --------------------------------------------- |
| `Compound`                | Named compound with formula and descriptors   |
| `CompoundName`            | Validated primary compound name               |
| `CommonName`              | Optional validated common name                |
| `SystematicName`          | Optional validated systematic name            |
| `CompoundFormula`         | Compound-facing formula wrapper               |
| `EmpiricalFormula`        | Empirical formula wrapper                     |
| `MolecularFormula`        | Molecular formula wrapper                     |
| `CompoundKind`            | Lightweight classification label              |
| `CompoundIdentifier`      | Optional registry identifier                  |
| `CompoundRegistry`        | Registry namespace for identifiers            |
| `CompoundValidationError` | Structured construction and validation errors |

## Installation

```toml
[dependencies]
use-compound = "0.1.0"
```

## Quick Examples

### Create a simple compound

```rust
use use_chemical_formula::ChemicalFormula;
use use_compound::{Compound, CompoundKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let water = Compound::new("water", ChemicalFormula::parse("H2O")?)?
    .with_kind(CompoundKind::Molecular);

assert_eq!(water.name().as_str(), "water");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water.kinds(), &[CompoundKind::Molecular]);
# Ok(())
# }
```

### Add names and identifiers

```rust
use use_chemical_formula::ChemicalFormula;
use use_compound::{Compound, CompoundIdentifier, CompoundKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let glucose = Compound::new("glucose", ChemicalFormula::parse("C6H12O6")?)?
    .try_with_common_name("dextrose")?
    .try_with_systematic_name("D-glucose")?
    .with_kind(CompoundKind::Organic)
    .try_with_identifier(CompoundIdentifier::pub_chem_cid("5793")?)?;

assert_eq!(glucose.common_name().map(|name| name.as_str()), Some("dextrose"));
assert_eq!(glucose.identifiers().len(), 1);
# Ok(())
# }
```

## Scope

- Represents compound identity, names, formulas, kind labels, and identifiers.
- Uses `use-chemical-formula` for formula primitives.
- Validates that names and identifier values are not empty.
- Keeps registry identifiers as lightweight wrappers only.
- No hardcoded compound database.
- No CAS checksum validation, PubChem lookup, InChI parsing, or SMILES parsing.
- No molecular geometry, bonding models, reactions, stoichiometry, molar mass,
  pH, solutions, thermochemistry, or simulation.
- No naming engine.

## Relationship to `use-chemistry`

`use-compound` is a focused child crate for compound identity primitives. The
`use-chemistry` umbrella crate reexports it alongside formula, element,
isotope, periodic-table, atomic-number, atomic-mass, and electron-shell helpers.
