# use-chemistry

Composable chemistry primitives for Rust.

`use-chemistry` starts with periodic-table primitives, element lookup, formula structures, bond primitives, oxidation-state primitives, ion identity, compound identity, molecule identity, atomic numbers, atomic masses, simple electron shell helpers, and isotope identity helpers.

It is a sibling RustUse set beside `use-math`, `use-color`, `use-text`, and `use-wave`. The workspace stays one layer deep, direct crates stay independently useful, and the public APIs stay small, explicit, documented, and dependency-light.

## Workspace crates

- `use-chemistry`: umbrella crate that reexports the full workspace with a shared prelude
- `use-element`: basic chemical element primitives and lookup helpers
- `use-chemical-formula`: structural chemical formula primitives and lightweight parsing
- `use-bond`: chemical bond identity, order, endpoint, polarity, and strength primitives
- `use-oxidation-state`: oxidation-state values, Roman labels, and assignment primitives
- `use-ion`: charged atom and charged group primitives backed by formulas
- `use-compound`: named chemical compound identity primitives and lightweight descriptors
- `use-molecule`: discrete molecule identity primitives with optional atom-level structure
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
	Bond, BondKind, BondOrder, ChemicalFormula, Compound, CompoundKind, Molecule, MoleculeKind,
	ElementOxidationState, Ion, IonCharge, OxidationState, atomic_mass_by_symbol,
	atomic_number_from_symbol, electron_shells, element_by_symbol, isotope_by_symbol,
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
let iron_three = ElementOxidationState::new("Fe", OxidationState::positive(3).unwrap()).unwrap();

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(covalent_bond.order(), Some(BondOrder::Single));
assert_eq!(sodium_ion.to_string(), "Na+");
assert_eq!(iron_three.to_string(), "Fe(III)");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water_molecule.formula().to_string(), "H2O");
assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert!((atomic_mass_by_symbol("O").unwrap() - 15.999).abs() < 0.01);
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
assert_eq!(carbon_14.hyphen_notation(), Some(String::from("C-14")));
assert_eq!(calcium_hydroxide.element_counts().get("O"), Some(&2));
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

### Formula parsing

```rust
use use_chemical_formula::ChemicalFormula;

let formula = ChemicalFormula::parse("Al2(SO4)3").unwrap();
let counts = formula.element_counts();

assert_eq!(formula.to_string(), "Al2(SO4)3");
assert_eq!(counts.get("Al"), Some(&2));
assert_eq!(counts.get("S"), Some(&3));
assert_eq!(counts.get("O"), Some(&12));
```

### Compound identity

```rust
use use_chemical_formula::ChemicalFormula;
use use_compound::{Compound, CompoundIdentifier, CompoundKind};

let glucose = Compound::new("glucose", ChemicalFormula::parse("C6H12O6").unwrap())
	.unwrap()
	.try_with_common_name("dextrose")
	.unwrap()
	.with_kind(CompoundKind::Organic)
	.try_with_identifier(CompoundIdentifier::pub_chem_cid("5793").unwrap())
	.unwrap();

assert_eq!(glucose.name().as_str(), "glucose");
assert_eq!(glucose.formula().to_string(), "C6H12O6");
assert_eq!(glucose.common_name().map(|name| name.as_str()), Some("dextrose"));
```

### Bond primitives

```rust
use use_bond::{Bond, BondEndpoint, BondKind, BondOrder, BondPolarity};

let bond = Bond::between(
	BondEndpoint::new("O").unwrap(),
	BondEndpoint::new("H").unwrap(),
	BondKind::Covalent,
)
.with_order(BondOrder::Single)
.with_polarity(BondPolarity::Polar);

assert_eq!(bond.kind(), BondKind::Covalent);
assert_eq!(bond.order(), Some(BondOrder::Single));
assert_eq!(bond.to_string(), "O-H covalent bond (single)");
```

### Ion identity

```rust
use use_chemical_formula::ChemicalFormula;
use use_ion::{Ion, IonCharge, IonKind};

let sodium = Ion::new(ChemicalFormula::parse("Na").unwrap(), IonCharge::positive(1).unwrap());
let sulfate = Ion::new(ChemicalFormula::parse("SO4").unwrap(), IonCharge::negative(2).unwrap())
	.with_kind(IonKind::Polyatomic);

assert!(sodium.is_cation());
assert!(sulfate.is_anion());
assert_eq!(sodium.to_string(), "Na+");
assert_eq!(sulfate.to_string(), "SO4^2-");
```

### Oxidation states

```rust
use use_oxidation_state::{ElementOxidationState, OxidationState, OxidationStateAssignment};

let iron_two = ElementOxidationState::new("Fe", OxidationState::positive(2).unwrap()).unwrap();
let oxygen = OxidationStateAssignment::new("O", OxidationState::negative(2).unwrap()).unwrap();

assert_eq!(iron_two.to_string(), "Fe(II)");
assert_eq!(oxygen.to_string(), "O: -2");
assert_eq!(OxidationState::zero().to_string(), "0");
```

### Molecule identity

```rust
use use_chemical_formula::ChemicalFormula;
use use_molecule::{MolecularAtom, Molecule, MoleculeKind};

let water = Molecule::builder("water")
	.formula(ChemicalFormula::parse("H2O").unwrap())
	.atom(MolecularAtom::new("O").unwrap())
	.atom(MolecularAtom::new("H").unwrap())
	.atom(MolecularAtom::new("H").unwrap())
	.kind(MoleculeKind::Neutral)
	.build()
	.unwrap();

assert_eq!(water.name().as_str(), "water");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water.atom_count(), 3);
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

This focused v0.1 workspace keeps to small, static, auditable chemistry primitives. It intentionally stops short of compound, molecule, ion, or oxidation-state databases, naming engines, molecular geometry, force fields, bond inference, oxidation-state inference, electrochemistry simulation, acid/base behavior, redox reactions, reaction balancing, stoichiometry, isotope abundance/mass/decay tables, thermochemistry, chemical file-format parsing, and broader framework-style abstractions.
