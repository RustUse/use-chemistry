#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Formula molar-mass values and lookup-backed calculations.

mod atomic_mass_entry;
mod atomic_mass_lookup;
mod calculation;
mod element_mass_contribution;
mod error;
mod formula_molar_mass;
mod mass_contribution_set;
mod molar_mass;
mod molar_mass_unit;

pub use atomic_mass_entry::AtomicMassEntry;
pub use atomic_mass_lookup::AtomicMassLookup;
pub use calculation::MolarMassCalculation;
pub use element_mass_contribution::ElementMassContribution;
pub use error::MolarMassValidationError;
pub use formula_molar_mass::FormulaMolarMass;
pub use mass_contribution_set::MassContributionSet;
pub use molar_mass::MolarMass;
pub use molar_mass_unit::MolarMassUnit;

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;

    use super::{
        AtomicMassEntry, AtomicMassLookup, ElementMassContribution, MolarMass,
        MolarMassCalculation, MolarMassUnit, MolarMassValidationError,
    };

    #[test]
    fn creates_molar_mass_values() {
        let grams = MolarMass::grams_per_mole(18.015).expect("mass should be valid");
        let kilograms = MolarMass::kilograms_per_mole(0.018_015).expect("mass should be valid");

        assert_close(grams.value(), 18.015);
        assert_eq!(grams.unit(), MolarMassUnit::GramsPerMole);
        assert_eq!(grams.to_string(), "18.015 g/mol");
        assert_close(kilograms.value(), 0.018_015);
        assert_eq!(kilograms.unit(), MolarMassUnit::KilogramsPerMole);
        assert_eq!(kilograms.to_string(), "0.018015 kg/mol");
    }

    #[test]
    fn rejects_invalid_molar_mass_values() {
        assert_eq!(
            MolarMass::grams_per_mole(0.0),
            Err(MolarMassValidationError::NonPositiveMolarMass)
        );
        assert_eq!(
            MolarMass::grams_per_mole(-1.0),
            Err(MolarMassValidationError::NonPositiveMolarMass)
        );
        assert_eq!(
            MolarMass::grams_per_mole(f64::NAN),
            Err(MolarMassValidationError::NonFiniteMolarMass)
        );
        assert_eq!(
            MolarMass::grams_per_mole(f64::INFINITY),
            Err(MolarMassValidationError::NonFiniteMolarMass)
        );
    }

    #[test]
    fn creates_atomic_mass_lookup_entries() {
        let entry = AtomicMassEntry::new("H", 1.008).expect("entry should be valid");
        let mut lookup = AtomicMassLookup::from_entries([entry]);

        assert_eq!(lookup.len(), 1);
        assert!(lookup.contains_symbol("H"));
        assert_close(lookup.atomic_mass("H").unwrap_or_default(), 1.008);

        let previous = lookup
            .insert_atomic_mass("H", 1.01)
            .expect("replacement should be valid")
            .expect("previous value should exist");

        assert_close(previous, 1.008);
        assert_close(lookup.atomic_mass("H").unwrap_or_default(), 1.01);
    }

    #[test]
    fn rejects_invalid_atomic_mass_entries() {
        assert_eq!(
            AtomicMassEntry::new("hydrogen", 1.008),
            Err(MolarMassValidationError::InvalidElementSymbol(
                String::from("hydrogen")
            ))
        );
        assert_eq!(
            AtomicMassEntry::new("H", f64::NAN),
            Err(MolarMassValidationError::NonFiniteAtomicMass {
                symbol: String::from("H")
            })
        );
        assert_eq!(
            AtomicMassEntry::new("H", 0.0),
            Err(MolarMassValidationError::NonPositiveAtomicMass {
                symbol: String::from("H")
            })
        );
    }

    #[test]
    fn calculates_common_formula_molar_masses() {
        assert_formula_mass("H2O", &[("H", 1.008), ("O", 15.999)], 18.015);
        assert_formula_mass("CO2", &[("C", 12.011), ("O", 15.999)], 44.009);
        assert_formula_mass("NaCl", &[("Na", 22.990), ("Cl", 35.45)], 58.44);
        assert_formula_mass(
            "C6H12O6",
            &[("C", 12.011), ("H", 1.008), ("O", 15.999)],
            180.156,
        );
        assert_formula_mass(
            "Ca(OH)2",
            &[("Ca", 40.078), ("O", 15.999), ("H", 1.008)],
            74.092,
        );
    }

    #[test]
    fn reports_missing_atomic_mass() {
        let formula = ChemicalFormula::parse("H2O").expect("formula should parse");
        let lookup = AtomicMassLookup::from_entries([
            AtomicMassEntry::new("H", 1.008).expect("entry should be valid")
        ]);
        let calculation = MolarMassCalculation::new(formula, lookup);

        assert_eq!(
            calculation.calculate(),
            Err(MolarMassValidationError::MissingAtomicMass {
                symbol: String::from("O")
            })
        );
    }

    #[test]
    fn exposes_contribution_totals_and_display() {
        let contribution =
            ElementMassContribution::new("H", 1.008, 2).expect("contribution should be valid");

        assert_eq!(contribution.symbol(), "H");
        assert_close(contribution.atomic_mass(), 1.008);
        assert_eq!(contribution.count(), 2);
        assert_close(contribution.total_mass_value(), 2.016);
        assert_eq!(contribution.to_string(), "H: 2 × 1.008 = 2.016 g/mol");
    }

    #[test]
    fn seeds_lookup_from_standard_atomic_masses() {
        let formula = ChemicalFormula::parse("H2O").expect("formula should parse");
        let calculation = MolarMassCalculation::with_standard_atomic_masses(formula)
            .expect("standard lookup should contain water elements");
        let result = calculation.calculate().expect("calculation should succeed");

        assert_close(result.molar_mass().value(), 18.015);
        assert_eq!(result.formula().to_string(), "H2O");
    }

    fn assert_formula_mass(formula: &str, entries: &[(&str, f64)], expected: f64) {
        let formula = ChemicalFormula::parse(formula).expect("formula should parse");
        let lookup = lookup(entries);
        let result = MolarMassCalculation::new(formula, lookup)
            .calculate()
            .expect("calculation should succeed");

        assert_close(result.molar_mass().value(), expected);
    }

    fn lookup(entries: &[(&str, f64)]) -> AtomicMassLookup {
        let mut lookup = AtomicMassLookup::new();

        for (symbol, atomic_mass) in entries {
            lookup
                .insert_atomic_mass(symbol, *atomic_mass)
                .expect("entry should be valid");
        }

        lookup
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {actual} to be close to {expected}"
        );
    }
}
