# use-molecule

Small molecular identity primitives for `RustUse` chemistry crates.

`use-molecule` represents molecules as discrete chemical entities with a name,
formula, optional explicit atoms, optional simple atom connections, formal
charge, and lightweight classification labels. It stays structural and avoids
simulation, force fields, orbital theory, reaction modeling, file formats,
external data fetching, and molecule databases.

## What this crate provides

| Item                      | Purpose                                           |
| ------------------------- | ------------------------------------------------- |
| `Molecule`                | Named molecular entity with formula and metadata  |
| `MoleculeBuilder`         | Builder for optional atom-level molecule assembly |
| `MoleculeName`            | Validated molecule name                           |
| `MolecularFormula`        | Molecule-facing formula wrapper                   |
| `MolecularAtom`           | Explicit atom entry                               |
| `MolecularAtomId`         | Optional validated atom identifier                |
| `AtomIndex`               | Zero-based explicit atom index                    |
| `AtomLabel`               | Validated atom element-label shape                |
| `AtomCount`               | Explicit atom count wrapper                       |
| `AtomConnection`          | Simple index-to-index atom connection             |
| `MoleculeCharge`          | Formal molecule charge                            |
| `MoleculeKind`            | Lightweight classification label                  |
| `MoleculeValidationError` | Structured construction and validation errors     |

## Installation

```toml
[dependencies]
use-molecule = "0.1.0"
```

## Quick Examples

### Create a simple molecule

```rust
use use_chemical_formula::ChemicalFormula;
use use_molecule::{Molecule, MoleculeKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let water = Molecule::new("water", ChemicalFormula::parse("H2O")?)?
    .with_kind(MoleculeKind::Neutral);

assert_eq!(water.name().as_str(), "water");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water.kinds(), &[MoleculeKind::Neutral]);
# Ok(())
# }
```

### Build a molecule with explicit atoms

```rust
use use_molecule::{MolecularAtom, Molecule};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let molecule = Molecule::builder("water")
    .formula("H2O".parse()?)
    .atom(MolecularAtom::new("O")?)
    .atom(MolecularAtom::new("H")?)
    .atom(MolecularAtom::new("H")?)
    .build()?;

assert_eq!(molecule.atom_count(), 3);
# Ok(())
# }
```

## Scope

- Represents molecule identity, names, formulas, atoms, connections, charge,
  and kind labels.
- Uses `use-chemical-formula` for formula primitives and atom label shape
  validation.
- Validates that molecule names, atom identifiers, atom labels, and connection
  indices are structurally usable.
- Keeps atom connections as simple index pairs with an optional order.
- No molecular geometry.
- No force fields.
- No orbital theory or quantum chemistry.
- No reactions or stoichiometry.
- No molar mass calculation.
- No SMILES, InChI, MOL, SDF, PDB, or other external chemical file formats.
- No runtime network access or hardcoded molecule database.

## Relationship to `use-chemistry`

`use-molecule` is a focused child crate for molecular identity primitives. The
`use-chemistry` umbrella crate reexports it alongside compound, formula,
element, isotope, periodic-table, atomic-number, atomic-mass, and electron-shell
helpers.
