# use-chemistry

Composable chemistry primitives for Rust.

`use-chemistry` starts with periodic-table primitives, element lookup, atomic numbers, atomic masses, simple electron shell helpers, and isotope identity helpers.

It is a sibling RustUse set beside `use-math`, `use-color`, `use-text`, and `use-wave`. The workspace stays one layer deep, direct crates stay independently useful, and the public APIs stay small, explicit, documented, and dependency-light.

## Workspace crates

- `use-chemistry`: umbrella crate that reexports the full workspace with a shared prelude
- `use-element`: basic chemical element primitives and lookup helpers
- `use-periodic-table`: periodic-table lookup and conservative classification helpers
- `use-isotope`: chemistry-facing isotope identity and notation helpers
- `use-atomic-number`: atomic-number validation and neutral-atom helpers
- `use-atomic-mass`: average atomic mass and molar mass helpers for elements
- `use-electron-shell`: simple shell distribution helpers for introductory chemistry use cases

## Umbrella crate

If you want a single dependency for the full workspace, use `use-chemistry`. It reexports the
focused crates and provides a `prelude` with the most common chemistry helpers.

```rust
use use_chemistry::prelude::{
	atomic_mass_by_symbol, atomic_number_from_symbol, electron_shells, element_by_symbol,
	isotope_by_symbol,
};

let oxygen = element_by_symbol("O").unwrap();
let carbon_14 = isotope_by_symbol("C", 14).unwrap();

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert!((atomic_mass_by_symbol("O").unwrap() - 15.999).abs() < 0.01);
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
assert_eq!(carbon_14.hyphen_notation(), Some(String::from("C-14")));
```

## Examples

### Lookup by symbol

```rust
use use_element::{element_by_atomic_number, element_by_symbol};

let carbon = element_by_symbol("C").unwrap();
assert_eq!(carbon.atomic_number, 6);
assert_eq!(carbon.name, "Carbon");

let oxygen = element_by_atomic_number(8).unwrap();
assert_eq!(oxygen.symbol, "O");
```

### Atomic mass lookup

```rust
use use_atomic_mass::atomic_mass_by_symbol;

let oxygen_mass = atomic_mass_by_symbol("O").unwrap();

assert!((oxygen_mass - 15.999).abs() < 0.01);
```

### Period and group helpers

```rust
use use_periodic_table::{group_for_atomic_number, period_for_atomic_number};

assert_eq!(period_for_atomic_number(11), Some(3));
assert_eq!(group_for_atomic_number(11), Some(1));
assert_eq!(group_for_atomic_number(92), None);
```

### Electron shells

```rust
use use_electron_shell::electron_shells;

assert_eq!(electron_shells(1), Some(vec![1]));
assert_eq!(electron_shells(10), Some(vec![2, 8]));
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
```

### Isotope identity

```rust
use use_isotope::{isotope_by_symbol, isotope_neutron_count};

let uranium_235 = isotope_by_symbol("U", 235).unwrap();

assert_eq!(uranium_235.atomic_number(), 92);
assert_eq!(uranium_235.neutron_count(), 143);
assert_eq!(isotope_neutron_count(6, 14), Some(8));
```

## Relationship to RustUse

- `use-chemistry` is a sibling set to `use-math`, `use-color`, `use-text`, and `use-wave`.
- Crates stay one layer deep.
- Each crate is designed to be independently useful.

## Status

This focused v0.1 workspace keeps to small, static, auditable chemistry primitives. It intentionally stops short of molecule parsing, reaction balancing, stoichiometry, isotope abundance/mass/decay tables, thermochemistry, and broader framework-style abstractions.
