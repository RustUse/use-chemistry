# use-chemistry

Composable chemistry primitives for `RustUse`.

`use-chemistry` is the thin umbrella crate for the `RustUse` chemistry workspace. Use it when you
want the common prelude and reexports from `use-element`, `use-atomic-number`,
`use-atomic-mass`, `use-bond`, `use-chemical-formula`, `use-ion`,
`use-compound`, `use-molecule`, `use-electron-shell`, `use-isotope`, and
`use-periodic-table` in one dependency.

## Reexports

- `use_element`
- `use_atomic_number`
- `use_atomic_mass`
- `use_bond`
- `use_chemical_formula`
- `use_ion`
- `use_compound`
- `use_molecule`
- `use_electron_shell`
- `use_isotope`
- `use_periodic_table`

## Example

```rust
use use_chemistry::prelude::{
	Bond, BondKind, BondOrder, ChemicalFormula, Compound, CompoundKind, Molecule, MoleculeKind,
	Ion, IonCharge, atomic_mass_by_symbol, atomic_number_from_symbol, electron_shells,
	element_by_symbol, isotope_by_symbol,
};

let oxygen = element_by_symbol("O").unwrap();
let carbon_14 = isotope_by_symbol("C", 14).unwrap();
let calcium_hydroxide = ChemicalFormula::parse("Ca(OH)2").unwrap();
let water = Compound::new("water", ChemicalFormula::parse("H2O").unwrap())
	.unwrap()
	.with_kind(CompoundKind::Molecular);
let water_molecule = Molecule::new("water", ChemicalFormula::parse("H2O").unwrap())
	.unwrap()
	.with_kind(MoleculeKind::Neutral);
let covalent_bond = Bond::new(BondKind::Covalent).with_order(BondOrder::Single);
let sodium_ion = Ion::new(
	ChemicalFormula::parse("Na").unwrap(),
	IonCharge::positive(1).unwrap(),
);

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(covalent_bond.order(), Some(BondOrder::Single));
assert_eq!(sodium_ion.to_string(), "Na+");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water_molecule.formula().to_string(), "H2O");
assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert!((atomic_mass_by_symbol("O").unwrap() - 15.999).abs() < 0.01);
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
assert_eq!(carbon_14.neutron_count(), 8);
assert_eq!(calcium_hydroxide.element_counts().get("O"), Some(&2));
```
