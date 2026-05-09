# use-periodic-table

<p align="center">
  <strong>Periodic-table lookup and conservative classification helpers.</strong><br>
  Direct period, group, and broad family helpers backed by the `use-element` static element table.
</p>

`use-periodic-table` exposes full element iteration, period and group filtering, atomic-number metadata helpers, and a small set of conservative chemical family checks.

## Surface

| Item                          | Purpose                           |
| ----------------------------- | --------------------------------- |
| `all_elements()`              | Full static element slice         |
| `period_elements()`           | Elements in a given period        |
| `group_elements()`            | Elements in a given group         |
| `period_for_atomic_number()`  | Period lookup from atomic number  |
| `group_for_atomic_number()`   | Group lookup from atomic number   |
| `is_valid_atomic_number()`    | Range check for 1 through 118     |
| `is_alkali_metal()` and peers | Conservative broad-family helpers |

## Examples

```rust
use use_periodic_table::{group_elements, is_noble_gas, period_elements};

assert_eq!(period_elements(2).len(), 8);
assert_eq!(group_elements(18).len(), 7);
assert!(is_noble_gas(10));
```
