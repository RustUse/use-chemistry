#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Stoichiometry primitives.

mod coefficient;
mod error;
mod excess_reagent;
mod formula_quantity;
mod limiting_reagent;
mod mole_ratio;
mod ratio;
mod reaction_entry;
mod reaction_side;
mod term;
mod r#yield;

pub use coefficient::StoichiometricCoefficient;
pub use error::StoichiometryValidationError;
pub use excess_reagent::ExcessReagent;
pub use formula_quantity::FormulaQuantity;
pub use limiting_reagent::LimitingReagent;
pub use mole_ratio::MoleRatio;
pub use ratio::StoichiometricRatio;
pub use reaction_entry::{ProductEntry, ReactantEntry, ReactionEntry};
pub use reaction_side::ReactionSide;
pub use term::StoichiometricTerm;
pub use r#yield::{ActualYield, PercentYield, TheoreticalYield};

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;

    use super::{
        ActualYield, ExcessReagent, FormulaQuantity, LimitingReagent, MoleRatio, PercentYield,
        ProductEntry, ReactantEntry, ReactionEntry, ReactionSide, StoichiometricCoefficient,
        StoichiometricRatio, StoichiometricTerm, StoichiometryValidationError, TheoreticalYield,
    };

    fn formula(input: &str) -> ChemicalFormula {
        ChemicalFormula::parse(input).expect("test formula should parse")
    }

    fn coefficient(value: u32) -> StoichiometricCoefficient {
        StoichiometricCoefficient::new(value).expect("coefficient should be valid")
    }

    #[test]
    fn creates_coefficients() {
        let one = coefficient(1);
        let two = coefficient(2);

        assert_eq!(one.value(), 1);
        assert!(one.is_one());
        assert_eq!(two.value(), 2);
        assert!(!two.is_one());
        assert_eq!(two.to_string(), "2");
    }

    #[test]
    fn rejects_zero_coefficients() {
        assert_eq!(
            StoichiometricCoefficient::new(0),
            Err(StoichiometryValidationError::ZeroCoefficient)
        );
        assert_eq!(
            StoichiometricCoefficient::try_from(0),
            Err(StoichiometryValidationError::ZeroCoefficient)
        );
    }

    #[test]
    fn creates_reaction_entries() {
        let hydrogen = ReactionEntry::new(coefficient(2), formula("H2"), ReactionSide::Reactant)
            .expect("entry should be valid");
        let oxygen = ReactionEntry::new(coefficient(1), formula("O2"), ReactionSide::Reactant)
            .expect("entry should be valid");
        let water = ReactionEntry::new(coefficient(2), formula("H2O"), ReactionSide::Product)
            .expect("entry should be valid");

        assert_eq!(hydrogen.coefficient().value(), 2);
        assert_eq!(hydrogen.formula().to_string(), "H2");
        assert_eq!(hydrogen.side(), ReactionSide::Reactant);
        assert_eq!(oxygen.to_string(), "O2");
        assert_eq!(water.side(), ReactionSide::Product);
        assert_eq!(water.to_string(), "2H2O");
    }

    #[test]
    fn wraps_reactant_and_product_entries() {
        let methane =
            ReactantEntry::new(coefficient(1), formula("CH4")).expect("reactant should be valid");
        let oxygen =
            ReactantEntry::new(coefficient(2), formula("O2")).expect("reactant should be valid");
        let carbon_dioxide =
            ProductEntry::new(coefficient(1), formula("CO2")).expect("product should be valid");
        let ammonia =
            ProductEntry::new(coefficient(2), formula("NH3")).expect("product should be valid");

        assert_eq!(methane.to_string(), "CH4");
        assert_eq!(oxygen.to_string(), "2O2");
        assert_eq!(carbon_dioxide.to_string(), "CO2");
        assert_eq!(ammonia.to_string(), "2NH3");
        assert_eq!(methane.as_entry().side(), ReactionSide::Reactant);
        assert_eq!(carbon_dioxide.as_entry().side(), ReactionSide::Product);
        assert_eq!(
            ProductEntry::from_entry(methane.as_entry().clone()),
            Err(StoichiometryValidationError::ExpectedProduct)
        );
        assert_eq!(
            ReactantEntry::from_entry(carbon_dioxide.as_entry().clone()),
            Err(StoichiometryValidationError::ExpectedReactant)
        );
    }

    #[test]
    fn displays_stoichiometric_terms() {
        let calcium_carbonate = StoichiometricTerm::new(coefficient(1), formula("CaCO3"))
            .expect("term should be valid");
        let calcium_oxide =
            StoichiometricTerm::new(coefficient(1), formula("CaO")).expect("term should be valid");
        let nitrogen =
            StoichiometricTerm::new(coefficient(1), formula("N2")).expect("term should be valid");
        let hydrogen =
            StoichiometricTerm::new(coefficient(3), formula("H2")).expect("term should be valid");

        assert_eq!(calcium_carbonate.to_string(), "CaCO3");
        assert_eq!(calcium_oxide.to_string(), "CaO");
        assert_eq!(nitrogen.to_string(), "N2");
        assert_eq!(hydrogen.to_string(), "3H2");
        assert_eq!(
            StoichiometricTerm::from_value(0, formula("H2")),
            Err(StoichiometryValidationError::ZeroCoefficient)
        );
    }

    #[test]
    fn creates_mole_ratios() {
        let ratio = MoleRatio::new(coefficient(2), coefficient(1)).expect("ratio should be valid");
        let raw_ratio = MoleRatio::from_values(3, 2).expect("ratio should be valid");
        let stoichiometric = StoichiometricRatio::from_values(4, 1).expect("ratio should be valid");

        assert_eq!(ratio.numerator().value(), 2);
        assert_eq!(ratio.denominator().value(), 1);
        assert_eq!(ratio.to_string(), "2:1");
        assert_eq!(raw_ratio.to_string(), "3:2");
        assert_eq!(stoichiometric.to_string(), "4:1");
    }

    #[test]
    fn rejects_invalid_ratio_denominators() {
        assert_eq!(
            MoleRatio::from_values(2, 0),
            Err(StoichiometryValidationError::ZeroRatioDenominator)
        );
        assert_eq!(
            StoichiometricRatio::from_values(0, 2),
            Err(StoichiometryValidationError::ZeroCoefficient)
        );
    }

    #[test]
    fn creates_formula_quantities() {
        let quantity =
            FormulaQuantity::new(coefficient(2), formula("H2O")).expect("quantity should be valid");

        assert_eq!(quantity.coefficient().value(), 2);
        assert_eq!(quantity.formula().to_string(), "H2O");
        assert_eq!(quantity.term().to_string(), "2H2O");
        assert_eq!(quantity.to_string(), "2H2O");
    }

    #[test]
    fn validates_reagent_labels() {
        let limiting = LimitingReagent::new(" O2 ").expect("label should be valid");
        let excess = ExcessReagent::new("CH4").expect("label should be valid");

        assert_eq!(limiting.as_str(), "O2");
        assert_eq!(limiting.to_string(), "O2");
        assert_eq!(excess.as_str(), "CH4");
        assert_eq!(excess.to_string(), "CH4");
        assert_eq!(
            LimitingReagent::new(" "),
            Err(StoichiometryValidationError::EmptyLimitingReagentLabel)
        );
        assert_eq!(
            ExcessReagent::new(""),
            Err(StoichiometryValidationError::EmptyExcessReagentLabel)
        );
    }

    #[test]
    fn creates_yield_values() {
        let theoretical = TheoreticalYield::new(10.0).expect("yield should be valid");
        let actual = ActualYield::new(8.0).expect("yield should be valid");
        let percent =
            PercentYield::from_yields(actual, theoretical).expect("percent yield should be valid");
        let direct_percent = PercentYield::from_actual_and_theoretical(8.0, 10.0)
            .expect("percent yield should be valid");

        assert!((theoretical.value() - 10.0).abs() < f64::EPSILON);
        assert!((actual.value() - 8.0).abs() < f64::EPSILON);
        assert!((percent.value() - 80.0).abs() < f64::EPSILON);
        assert!((direct_percent.value() - 80.0).abs() < f64::EPSILON);
        assert_eq!(percent.to_string(), "80%");
    }

    #[test]
    fn rejects_invalid_yield_values() {
        assert_eq!(
            ActualYield::new(-1.0),
            Err(StoichiometryValidationError::NegativeYield)
        );
        assert_eq!(
            PercentYield::new(-1.0),
            Err(StoichiometryValidationError::NegativeYield)
        );
        assert_eq!(
            TheoreticalYield::new(0.0),
            Err(StoichiometryValidationError::NonPositiveTheoreticalYield)
        );
        assert_eq!(
            TheoreticalYield::new(f64::INFINITY),
            Err(StoichiometryValidationError::NonFiniteYield)
        );
        assert_eq!(
            PercentYield::from_actual_and_theoretical(8.0, 0.0),
            Err(StoichiometryValidationError::NonPositiveTheoreticalYield)
        );
    }

    #[test]
    fn exposes_reaction_side_helpers() {
        assert!(ReactionSide::Reactant.is_reactant());
        assert!(ReactionSide::Product.is_product());
        assert_eq!(ReactionSide::Reactant.to_string(), "reactant");
        assert_eq!(ReactionSide::Product.to_string(), "product");
    }
}
