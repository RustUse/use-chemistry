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
pub use use_molar_mass;
pub use use_molecule;
pub use use_oxidation_state;
pub use use_periodic_table;
pub use use_reaction;
pub use use_stoichiometry;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        AtomicMassEntry, AtomicMassLookup, Bond, BondKind, BondOrder, ChemicalFormula,
        ChemicalReaction, Compound, CompoundKind, ElementOxidationState, Ion, IonCharge,
        MolarMassCalculation, MoleRatio, Molecule, MoleculeKind, OxidationState, ReactionEntry,
        ReactionSide, ReactionTerm, StoichiometricCoefficient, atomic_mass_by_symbol,
        atomic_number_from_symbol, electron_shells, element_by_symbol, isotope_by_symbol,
        period_for_atomic_number,
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
        let iron_three = ElementOxidationState::new(
            "Fe",
            OxidationState::positive(3).expect("oxidation state should be valid"),
        )
        .expect("element oxidation state should be valid");
        let water_entry = ReactionEntry::new(
            StoichiometricCoefficient::new(2).expect("coefficient should be valid"),
            ChemicalFormula::parse("H2O").expect("water should parse"),
            ReactionSide::Product,
        )
        .expect("reaction entry should be valid");
        let water_reaction = ChemicalReaction::new()
            .with_reactant(
                ReactionTerm::new(ChemicalFormula::parse("H2").expect("hydrogen should parse"))
                    .with_coefficient(2)
                    .expect("coefficient should be valid"),
            )
            .with_reactant(ReactionTerm::new(
                ChemicalFormula::parse("O2").expect("oxygen should parse"),
            ))
            .with_product(
                ReactionTerm::new(ChemicalFormula::parse("H2O").expect("water should parse"))
                    .with_coefficient(2)
                    .expect("coefficient should be valid"),
            );
        let water_ratio = MoleRatio::from_values(2, 1).expect("ratio should be valid");
        let molar_mass_lookup = AtomicMassLookup::from_entries([
            AtomicMassEntry::new("H", 1.008).expect("hydrogen mass should be valid"),
            AtomicMassEntry::new("O", 15.999).expect("oxygen mass should be valid"),
        ]);
        let water_molar_mass = MolarMassCalculation::new(
            ChemicalFormula::parse("H2O").expect("water should parse"),
            molar_mass_lookup,
        )
        .calculate()
        .expect("molar mass should calculate");

        assert_eq!(oxygen.atomic_number, 8);
        assert_eq!(covalent_bond.order(), Some(BondOrder::Single));
        assert!(sodium_ion.is_cation());
        assert_eq!(sodium_ion.to_string(), "Na+");
        assert_eq!(iron_three.to_string(), "Fe(III)");
        assert_eq!(water_entry.to_string(), "2H2O");
        assert_eq!(water_reaction.to_string(), "2H2 + O2 -> 2H2O");
        assert_eq!(water_ratio.to_string(), "2:1");
        assert_eq!(water.name().as_str(), "water");
        assert_eq!(water.formula().to_string(), "H2O");
        assert_eq!(water_molecule.formula().to_string(), "H2O");
        assert!((water_molar_mass.molar_mass().value() - 18.015).abs() < 0.001);
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
