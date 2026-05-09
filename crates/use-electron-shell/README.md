# use-electron-shell

<p align="center">
  <strong>Simple electron shell distribution helpers.</strong><br>
  Static shell distributions for known elements with conservative main-group valence helpers for introductory and general utility use.
</p>

`use-electron-shell` is intentionally simple. It does not model full electron configurations, orbital ordering nuance, or transition-metal valence chemistry.

## Surface

| Item                             | Purpose                                                  |
| -------------------------------- | -------------------------------------------------------- |
| `electron_shells()`              | Shell population list for an atomic number               |
| `shell_count()`                  | Number of occupied shells in the static distribution     |
| `outer_shell_electrons()`        | Electrons in the outermost occupied shell                |
| `valence_electrons_main_group()` | Conservative valence helper for main-group elements only |

## Example

```rust
use use_electron_shell::{electron_shells, valence_electrons_main_group};

assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
assert_eq!(valence_electrons_main_group(17), Some(7));
assert_eq!(valence_electrons_main_group(26), None);
```
