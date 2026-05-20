# use-bond

Small chemical bond primitives for `RustUse` chemistry crates.

`use-bond` represents bond identity, kind labels, orders, atom endpoint
references, participants, polarity labels, strength labels, optional displayable
lengths, optional angle/reference labels, and lightweight descriptors. It stays
structural and intentionally avoids molecular simulation, quantum chemistry,
force fields, reactions, molecule parsing, and external chemistry formats.

## What this crate provides

| Item                  | Purpose                                      |
| --------------------- | -------------------------------------------- |
| `Bond`                | Chemical bond identity and optional metadata |
| `BondKind`            | Bond kind labels                             |
| `BondOrder`           | Bond order labels                            |
| `FractionalBondOrder` | Rational fractional bond order               |
| `BondEndpoint`        | Validated atom endpoint reference            |
| `BondParticipant`     | Validated bond participant reference         |
| `BondLength`          | Positive displayable bond length             |
| `BondPolarity`        | Bond polarity labels                         |
| `BondStrength`        | Bond strength labels                         |
| `BondDescriptor`      | Lightweight descriptor or reference label    |
| `BondValidationError` | Structured validation errors                 |

## Installation

```toml
[dependencies]
use-bond = "0.1.0"
```

## Quick Examples

### Create a simple covalent bond

```rust
use use_bond::{Bond, BondKind, BondOrder};

let bond = Bond::new(BondKind::Covalent).with_order(BondOrder::Single);

assert_eq!(bond.kind(), BondKind::Covalent);
assert_eq!(bond.order(), Some(BondOrder::Single));
```

### Create a bond between endpoint references

```rust
use use_bond::{Bond, BondEndpoint, BondKind, BondOrder};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let bond = Bond::between(
    BondEndpoint::new("O")?,
    BondEndpoint::new("H")?,
    BondKind::Covalent,
)
.with_order(BondOrder::Single);

assert_eq!(bond.endpoints().len(), 2);
assert_eq!(bond.to_string(), "O-H covalent bond (single)");
# Ok(())
# }
```

### Attach lightweight descriptors

```rust
use use_bond::{Bond, BondKind, BondLength, BondPolarity, BondStrength};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let bond = Bond::new(BondKind::Hydrogen)
    .with_polarity(BondPolarity::Polar)
    .with_strength(BondStrength::Weak)
    .with_length(BondLength::new(190.0, "pm")?)
    .try_with_angle_label("donor-H-acceptor")?;

assert_eq!(bond.kind(), BondKind::Hydrogen);
assert_eq!(bond.length().map(ToString::to_string), Some(String::from("190 pm")));
# Ok(())
# }
```

## Scope

- Represents chemical bond identity, kind, order, endpoint references,
  participants, polarity, strength, optional displayable length, optional
  angle/reference labels, and descriptors.
- Keeps endpoints as structural labels. It does not validate periodic-table
  membership or parse molecule formats.
- Keeps bond length as a positive finite value with a caller-provided unit
  string. It does not convert units or perform dimensional analysis.
- Uses rational fractional bond orders instead of floating-point enum payloads.
- No molecular geometry.
- No force-field calculations.
- No orbital theory or quantum chemistry.
- No electronegativity calculations.
- No reactions or stoichiometry.
- No molecule parsing.
- No SMILES, InChI, MOL, SDF, PDB, or other external chemical file formats.
- No runtime network access or external chemistry database.

## Relationship to `use-chemistry`

`use-bond` is a focused child crate for chemical bond primitives. The
`use-chemistry` umbrella crate reexports it alongside element, formula,
compound, molecule, isotope, periodic-table, atomic-number, atomic-mass, and
electron-shell helpers.
