# use-chemistry

Composable chemistry primitives for `RustUse`.

`use-chemistry` is the thin umbrella crate for the `RustUse` chemistry workspace. Use it when you
want the common prelude and reexports from `use-element`, `use-atomic-number`,
`use-atomic-mass`, `use-molar-mass`, `use-bond`, `use-chemical-formula`, `use-stoichiometry`,
`use-ion`, `use-reaction`, `use-oxidation-state`, `use-compound`, `use-molecule`,
`use-electron-shell`, `use-isotope`, and `use-periodic-table` in one dependency.

## Reexports

- `use_element`
- `use_atomic_number`
- `use_atomic_mass`
- `use_molar_mass`
- `use_bond`
- `use_chemical_formula`
- `use_stoichiometry`
- `use_reaction`
- `use_ion`
- `use_oxidation_state`
- `use_compound`
- `use_molecule`
- `use_electron_shell`
- `use_isotope`
- `use_periodic_table`

## Example

```rust
use use_chemistry::prelude::{
	AtomicMassEntry, AtomicMassLookup, Bond, BondKind, BondOrder, ChemicalFormula,
	ChemicalReaction, Compound, CompoundKind, ElementOxidationState, Ion, IonCharge,
	MolarMassCalculation, MoleRatio, Molecule, MoleculeKind, OxidationState,
	ReactionEntry, ReactionSide, ReactionTerm, StoichiometricCoefficient,
	atomic_mass_by_symbol, atomic_number_from_symbol, electron_shells, element_by_symbol,
	isotope_by_symbol,
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
let water_entry = ReactionEntry::new(
	StoichiometricCoefficient::new(2).unwrap(),
	ChemicalFormula::parse("H2O").unwrap(),
	ReactionSide::Product,
)
.unwrap();
let water_reaction = ChemicalReaction::new()
	.with_reactant(ReactionTerm::new(ChemicalFormula::parse("H2").unwrap()).with_coefficient(2).unwrap())
	.with_reactant(ReactionTerm::new(ChemicalFormula::parse("O2").unwrap()))
	.with_product(ReactionTerm::new(ChemicalFormula::parse("H2O").unwrap()).with_coefficient(2).unwrap());
let water_ratio = MoleRatio::from_values(2, 1).unwrap();
let molar_mass_lookup = AtomicMassLookup::from_entries([
	AtomicMassEntry::new("H", 1.008).unwrap(),
	AtomicMassEntry::new("O", 15.999).unwrap(),
]);
let water_molar_mass = MolarMassCalculation::new(
	ChemicalFormula::parse("H2O").unwrap(),
	molar_mass_lookup,
)
.calculate()
.unwrap();

assert_eq!(oxygen.atomic_number, 8);
assert_eq!(covalent_bond.order(), Some(BondOrder::Single));
assert_eq!(sodium_ion.to_string(), "Na+");
assert_eq!(iron_three.to_string(), "Fe(III)");
assert_eq!(water_entry.to_string(), "2H2O");
assert_eq!(water_reaction.to_string(), "2H2 + O2 -> 2H2O");
assert_eq!(water_ratio.to_string(), "2:1");
assert_eq!(water.formula().to_string(), "H2O");
assert_eq!(water_molecule.formula().to_string(), "H2O");
assert!((water_molar_mass.molar_mass().value() - 18.015).abs() < 0.001);
assert_eq!(atomic_number_from_symbol("Na"), Some(11));
assert!((atomic_mass_by_symbol("O").unwrap() - 15.999).abs() < 0.01);
assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
assert_eq!(carbon_14.neutron_count(), 8);
assert_eq!(calcium_hydroxide.element_counts().get("O"), Some(&2));
```
