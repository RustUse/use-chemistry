# use-chemistry

Composable chemistry primitives for RustUse.

`use-chemistry` is the thin umbrella crate for the RustUse chemistry workspace. Use it when you
want the common prelude and reexports from `use-element`, `use-atomic-number`,
`use-atomic-mass`, `use-electron-shell`, and `use-periodic-table` in one dependency.

## Reexports

- `use_element`
- `use_atomic_number`
- `use_atomic_mass`
- `use_electron_shell`
- `use_periodic_table`

## Example

```rust
use use_chemistry::prelude::{atomic_mass_by_symbol, atomic_number_from_symbol, electron_shells, element_by_symbol};

let oxygen = element_by_symbol("O").unwrap();

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert!((atomic_mass_by_symbol("O").unwrap() - 15.999).abs() < 0.01);
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
```