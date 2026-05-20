#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Structural chemical formula primitives and lightweight parsing.

mod chemical_formula;
mod element_count;
mod element_symbol;
mod error;
mod formula_group;
mod formula_part;
mod formula_term;
mod hydrate_part;
mod parse;

pub use chemical_formula::ChemicalFormula;
pub use element_count::{ElementCount, FormulaMultiplier};
pub use element_symbol::{ElementSymbol, is_valid_element_symbol};
pub use error::{FormulaParseError, FormulaValidationError};
pub use formula_group::FormulaGroup;
pub use formula_part::FormulaPart;
pub use formula_term::FormulaTerm;
pub use hydrate_part::HydratePart;

#[cfg(test)]
mod tests {
    use super::{ChemicalFormula, ElementSymbol, FormulaParseError};

    #[test]
    fn parses_simple_binary_formulas() {
        let water = ChemicalFormula::parse("H2O").expect("water formula should parse");
        let carbon_dioxide = ChemicalFormula::parse("CO2").expect("carbon dioxide should parse");
        let sodium_chloride = ChemicalFormula::parse("NaCl").expect("sodium chloride should parse");

        assert_eq!(water.to_string(), "H2O");
        assert_eq!(carbon_dioxide.to_string(), "CO2");
        assert_eq!(sodium_chloride.to_string(), "NaCl");
        assert_eq!(water.element_counts().get("H"), Some(&2));
        assert_eq!(water.element_counts().get("O"), Some(&1));
    }

    #[test]
    fn parses_one_and_two_letter_symbols() {
        let iron_oxide = ChemicalFormula::parse("Fe2O3").expect("iron oxide should parse");
        let counts = iron_oxide.element_counts();

        assert_eq!(iron_oxide.to_string(), "Fe2O3");
        assert_eq!(counts.get("Fe"), Some(&2));
        assert_eq!(counts.get("O"), Some(&3));
        assert_eq!(
            ElementSymbol::new("Cl").map(|symbol| symbol.to_string()),
            Ok(String::from("Cl"))
        );
    }

    #[test]
    fn parses_numeric_counts() {
        let glucose = ChemicalFormula::parse("C6H12O6").expect("glucose formula should parse");
        let ammonium_nitrate =
            ChemicalFormula::parse("NH4NO3").expect("ammonium nitrate should parse");
        let glucose_counts = glucose.element_counts();
        let nitrate_counts = ammonium_nitrate.element_counts();

        assert_eq!(glucose_counts.get("C"), Some(&6));
        assert_eq!(glucose_counts.get("H"), Some(&12));
        assert_eq!(glucose_counts.get("O"), Some(&6));
        assert_eq!(nitrate_counts.get("N"), Some(&2));
        assert_eq!(nitrate_counts.get("H"), Some(&4));
        assert_eq!(nitrate_counts.get("O"), Some(&3));
    }

    #[test]
    fn parses_groups_and_group_multipliers() {
        let calcium_hydroxide =
            ChemicalFormula::parse("Ca(OH)2").expect("calcium hydroxide should parse");
        let aluminum_sulfate =
            ChemicalFormula::parse("Al2(SO4)3").expect("aluminum sulfate should parse");
        let hydroxide_counts = calcium_hydroxide.element_counts();
        let sulfate_counts = aluminum_sulfate.element_counts();

        assert_eq!(calcium_hydroxide.to_string(), "Ca(OH)2");
        assert_eq!(hydroxide_counts.get("Ca"), Some(&1));
        assert_eq!(hydroxide_counts.get("O"), Some(&2));
        assert_eq!(hydroxide_counts.get("H"), Some(&2));
        assert_eq!(aluminum_sulfate.to_string(), "Al2(SO4)3");
        assert_eq!(sulfate_counts.get("Al"), Some(&2));
        assert_eq!(sulfate_counts.get("S"), Some(&3));
        assert_eq!(sulfate_counts.get("O"), Some(&12));
    }

    #[test]
    fn parses_hydrate_formulas() {
        let hydrate = ChemicalFormula::parse("CuSO4·5H2O").expect("hydrate should parse");
        let ascii_dot = ChemicalFormula::parse("CuSO4.5H2O").expect("dot hydrate should parse");
        let counts = hydrate.element_counts();

        assert_eq!(hydrate.to_string(), "CuSO4·5H2O");
        assert_eq!(ascii_dot.to_string(), "CuSO4·5H2O");
        assert_eq!(counts.get("Cu"), Some(&1));
        assert_eq!(counts.get("S"), Some(&1));
        assert_eq!(counts.get("O"), Some(&9));
        assert_eq!(counts.get("H"), Some(&10));
    }

    #[test]
    fn parses_nested_groups() {
        let formula = ChemicalFormula::parse("K4(ON(SO3)2)2").expect("nested formula should parse");
        let counts = formula.element_counts();

        assert_eq!(formula.to_string(), "K4(ON(SO3)2)2");
        assert_eq!(counts.get("K"), Some(&4));
        assert_eq!(counts.get("O"), Some(&14));
        assert_eq!(counts.get("N"), Some(&2));
        assert_eq!(counts.get("S"), Some(&4));
    }

    #[test]
    fn rejects_invalid_formulas() {
        assert_eq!(
            ChemicalFormula::parse(""),
            Err(FormulaParseError::EmptyFormula)
        );
        assert!(matches!(
            ChemicalFormula::parse("h2O"),
            Err(FormulaParseError::InvalidSymbol(_))
        ));
        assert_eq!(
            ChemicalFormula::parse("Ca(OH2"),
            Err(FormulaParseError::UnmatchedOpenGroup)
        );
        assert_eq!(
            ChemicalFormula::parse("Ca)OH("),
            Err(FormulaParseError::UnmatchedCloseGroup)
        );
        assert_eq!(
            ChemicalFormula::parse("Ca()2"),
            Err(FormulaParseError::EmptyGroup)
        );
        assert_eq!(
            ChemicalFormula::parse("H0O"),
            Err(FormulaParseError::ZeroCount)
        );
        assert_eq!(
            ChemicalFormula::parse("Ca(OH)0"),
            Err(FormulaParseError::ZeroMultiplier)
        );
        assert_eq!(
            ChemicalFormula::parse("CuSO4·0H2O"),
            Err(FormulaParseError::ZeroMultiplier)
        );
        assert_eq!(
            ChemicalFormula::parse("CuSO4·"),
            Err(FormulaParseError::TrailingSeparator)
        );
        assert!(matches!(
            ChemicalFormula::parse("H 2O"),
            Err(FormulaParseError::UnexpectedCharacter(' '))
        ));
    }
}
