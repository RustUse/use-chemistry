# use-isotope

Small isotope identity and notation helpers for `RustUse` chemistry crates.

`use-isotope` models chemistry-facing isotope identifiers such as `C-12`, `C-14`,
`O-16`, and `U-235`. It validates element numbers through the same 1 through 118
range used by `use-element`, exposes proton/neutron/nucleon counts, and formats
simple ASCII isotope notation.

## What this crate provides

| Item                         | Purpose                                                      |
| ---------------------------- | ------------------------------------------------------------ |
| `Isotope`                    | Small validated isotope identity with atomic and mass number |
| `isotope()`                  | Direct constructor helper by atomic number and mass number   |
| `isotope_by_symbol()`        | Symbol lookup through `use-element` plus mass-number input   |
| `is_valid_isotope_numbers()` | Structural isotope-number validation                         |
| `isotope_neutron_count()`    | Neutron count helper for isotope numbers                     |
| `isotope_symbol()`           | ASCII hyphen notation such as `C-12`                         |

## Installation

```toml
[dependencies]
use-isotope = "0.1.0"
```

## Quick Examples

### Lookup by element symbol

```rust
use use_isotope::isotope_by_symbol;

let carbon_12 = isotope_by_symbol(" c ", 12).unwrap();

assert_eq!(carbon_12.atomic_number(), 6);
assert_eq!(carbon_12.mass_number(), 12);
assert_eq!(carbon_12.neutron_count(), 6);
assert_eq!(carbon_12.hyphen_notation(), Some(String::from("C-12")));
```

### Construct by isotope numbers

```rust
use use_isotope::{isotope, isotope_neutron_count, isotope_symbol};

let uranium_235 = isotope(92, 235).unwrap();

assert_eq!(uranium_235.proton_count(), 92);
assert_eq!(uranium_235.neutron_count(), 143);
assert_eq!(isotope_neutron_count(6, 14), Some(8));
assert_eq!(isotope_symbol(8, 16), Some(String::from("O-16")));
```

## Scope

- Validates structural isotope identity, not experimental existence.
- Uses ASCII hyphen notation such as `C-12`.
- Depends on `use-element` for element symbols and names.
- No isotope abundance tables.
- No exact isotope masses.
- No decay chains, half-lives, binding-energy formulas, or radiation modeling.

## Relationship to `use-nuclear`

`use-isotope` is for chemistry-facing isotope identity and element notation. The
`use-nuclear` crate in the `RustUse` physics set remains the place for scalar
nuclear physics helpers such as decay, activity, mass defect, and binding energy.
