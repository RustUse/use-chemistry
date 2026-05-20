# use-element

<p align="center">
  <strong>Small chemical element primitives and static lookup helpers.</strong><br>
  Auditable periodic-table data for direct Rust code without runtime fetches or heavyweight chemistry frameworks.
</p>

<p align="center">
  <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
  <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
  <img alt="118 elements" src="https://img.shields.io/badge/elements-118-1d4ed8">
  <img alt="Static data" src="https://img.shields.io/badge/data-static-c2410c">
  <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
  <a href="#what-this-crate-provides">Surface</a> ·
  <a href="#symbol-lookup-behavior">Symbol lookup</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#quick-examples">Examples</a> ·
  <a href="#scope">Scope</a>
</p>

`use-element` provides a small `Element` value type plus direct lookup helpers backed by a static table for elements 1 through 118. It is intended as the foundational chemistry primitive crate for the RustUse chemistry workspace.

## What this crate provides

| Item                         | Purpose                                                                                |
| ---------------------------- | -------------------------------------------------------------------------------------- |
| `Element`                    | Small copyable element value with atomic number, symbol, name, mass, period, and group |
| `all_elements()`             | Full static slice of the known elements                                                |
| `element_by_symbol()`        | Symbol lookup with ASCII case-insensitive matching                                     |
| `element_by_atomic_number()` | Direct atomic-number lookup                                                            |
| `element_name()`             | Symbol-to-name helper                                                                  |
| `element_symbol()`           | Atomic-number-to-symbol helper                                                         |

## Symbol lookup behavior

Symbol lookup trims surrounding whitespace and compares symbols with ASCII case-insensitive matching. That means `"Na"`, `"na"`, and `" NA "` all resolve to sodium.

## Installation

```toml
[dependencies]
use-element = "0.1.0"
```

## Quick examples

### Lookup by symbol

```rust
use use_element::{element_by_symbol, element_name};

let carbon = element_by_symbol("c").unwrap();

assert_eq!(carbon.atomic_number, 6);
assert_eq!(carbon.name, "Carbon");
assert_eq!(element_name("C"), Some("Carbon"));
```

### Lookup by atomic number

```rust
use use_element::{element_by_atomic_number, element_symbol};

let oxygen = element_by_atomic_number(8).unwrap();

assert_eq!(oxygen.symbol, "O");
assert_eq!(oxygen.period, 2);
assert_eq!(oxygen.group, Some(16));
assert_eq!(element_symbol(79), Some("Au"));
```

## Scope

- Static periodic-table data only.
- No runtime network access.
- Isotope identity and notation helpers live in `use-isotope`.
- No isotope abundance, mass, or decay tables.
- No molecular or reaction modeling.
- Conservative group assignment for lanthanides and actinides via `None`.
