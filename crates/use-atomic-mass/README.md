# use-atomic-mass

<p align="center">
  <strong>Average atomic mass helpers for element-level calculations.</strong><br>
  Static elemental mass values suitable for general utility work, not high-precision scientific reference use.
</p>

## Surface

| Item                             | Purpose                                                 |
| -------------------------------- | ------------------------------------------------------- |
| `atomic_mass_by_symbol()`        | Look up an average atomic mass by element symbol        |
| `atomic_mass_by_atomic_number()` | Look up an average atomic mass by atomic number         |
| `average_atomic_mass()`          | Named alias for symbol-based average atomic mass lookup |
| `molar_mass_element()`           | Elemental molar mass helper in grams per mole           |

## Example

```rust
use use_atomic_mass::{atomic_mass_by_symbol, molar_mass_element};

assert!((atomic_mass_by_symbol("C").unwrap() - 12.011).abs() < 0.01);
assert!((molar_mass_element("O").unwrap() - 15.999).abs() < 0.01);
```
