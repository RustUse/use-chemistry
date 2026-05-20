#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_atomic_mass;
pub use use_atomic_number;
pub use use_bond;
pub use use_chemical_formula;
pub use use_compound;
pub use use_electron_shell;
pub use use_element;
pub use use_ion;
pub use use_isotope;
pub use use_molecule;
pub use use_periodic_table;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        Bond, BondKind, BondOrder, ChemicalFormula, Compound, CompoundKind, Ion, IonCharge,
        Molecule, MoleculeKind, atomic_mass_by_symbol, atomic_number_from_symbol, electron_shells,
        element_by_symbol, isotope_by_symbol, period_for_atomic_number,
    };

    #[test]
    fn facade_exposes_focused_crates() {
        let oxygen = element_by_symbol("O").expect("oxygen should exist");
        let formula = ChemicalFormula::parse("Ca(OH)2").expect("formula should parse");
        let counts = formula.element_counts();
        let water = Compound::new(
            "water",
            ChemicalFormula::parse("H2O").expect("water should parse"),
        )
        .expect("compound should be valid")
        .with_kind(CompoundKind::Molecular);
        let water_molecule = Molecule::new(
            "water",
            ChemicalFormula::parse("H2O").expect("water should parse"),
        )
        .expect("molecule should be valid")
        .with_kind(MoleculeKind::Neutral);
        let covalent_bond = Bond::new(BondKind::Covalent).with_order(BondOrder::Single);
        let sodium_ion = Ion::new(
            ChemicalFormula::parse("Na").expect("sodium should parse"),
            IonCharge::positive(1).expect("charge should be valid"),
        );

        assert_eq!(oxygen.atomic_number, 8);
        assert_eq!(covalent_bond.order(), Some(BondOrder::Single));
        assert!(sodium_ion.is_cation());
        assert_eq!(sodium_ion.to_string(), "Na+");
        assert_eq!(water.name().as_str(), "water");
        assert_eq!(water.formula().to_string(), "H2O");
        assert_eq!(water_molecule.formula().to_string(), "H2O");
        assert_eq!(counts.get("Ca"), Some(&1));
        assert_eq!(counts.get("O"), Some(&2));
        assert_eq!(counts.get("H"), Some(&2));
        assert_eq!(atomic_number_from_symbol("Na"), Some(11));
        assert!((atomic_mass_by_symbol("O").unwrap_or_default() - 15.999).abs() < 0.01);
        assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
        assert_eq!(
            isotope_by_symbol("C", 14).map(|value| value.neutron_count()),
            Some(8)
        );
        assert_eq!(period_for_atomic_number(11), Some(3));
    }
}
