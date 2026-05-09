# use-atomic-number

<p align="center">
  <strong>Atomic-number validation and neutral-atom helpers.</strong><br>
  Small direct helpers for validating atomic numbers and mapping element names or symbols to proton counts.
</p>

## Surface

| Item                            | Purpose                                          |
| ------------------------------- | ------------------------------------------------ |
| `is_valid_atomic_number()`      | Range check for 1 through 118                    |
| `atomic_number_from_symbol()`   | Symbol-to-atomic-number lookup                   |
| `atomic_number_from_name()`     | Name-to-atomic-number lookup                     |
| `proton_count()`                | Proton count for a valid atomic number           |
| `electron_count_neutral_atom()` | Neutral electron count for a valid atomic number |

## Example

```rust
use use_atomic_number::{atomic_number_from_name, atomic_number_from_symbol, proton_count};

assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert_eq!(atomic_number_from_name("oxygen"), Some(8));
assert_eq!(proton_count(79), Some(79));
```
