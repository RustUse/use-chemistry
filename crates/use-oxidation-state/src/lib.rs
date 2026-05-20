#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Oxidation-state primitives.

mod element_oxidation_state;
mod error;
mod formula_oxidation_state;
mod oxidation_magnitude;
mod oxidation_sign;
mod oxidation_state;
mod oxidation_state_assignment;
mod oxidation_state_set;
mod roman;

pub use element_oxidation_state::ElementOxidationState;
pub use error::OxidationStateValidationError;
pub use formula_oxidation_state::FormulaOxidationState;
pub use oxidation_magnitude::OxidationMagnitude;
pub use oxidation_sign::OxidationSign;
pub use oxidation_state::OxidationState;
pub use oxidation_state_assignment::OxidationStateAssignment;
pub use oxidation_state_set::OxidationStateSet;
pub use roman::{RomanOxidationState, roman_numeral};

#[cfg(test)]
mod tests {
    use super::{
        ElementOxidationState, FormulaOxidationState, OxidationMagnitude, OxidationSign,
        OxidationState, OxidationStateAssignment, OxidationStateSet, OxidationStateValidationError,
        RomanOxidationState, roman_numeral,
    };

    fn positive(magnitude: u8) -> OxidationState {
        OxidationState::positive(magnitude).expect("positive state should be valid")
    }

    fn negative(magnitude: u8) -> OxidationState {
        OxidationState::negative(magnitude).expect("negative state should be valid")
    }

    #[test]
    fn creates_positive_oxidation_states() {
        let hydrogen = positive(1);
        let iron = positive(3);

        assert_eq!(hydrogen.sign(), OxidationSign::Positive);
        assert_eq!(hydrogen.magnitude_value(), 1);
        assert_eq!(hydrogen.signed_value(), 1);
        assert!(hydrogen.is_positive());
        assert_eq!(hydrogen.to_string(), "+1");
        assert_eq!(iron.to_string(), "+3");
    }

    #[test]
    fn creates_negative_oxidation_states() {
        let oxygen = negative(2);
        let chloride = negative(1);

        assert_eq!(oxygen.sign(), OxidationSign::Negative);
        assert_eq!(oxygen.magnitude_value(), 2);
        assert_eq!(oxygen.signed_value(), -2);
        assert!(oxygen.is_negative());
        assert_eq!(oxygen.to_string(), "-2");
        assert_eq!(chloride.to_string(), "-1");
    }

    #[test]
    fn creates_zero_oxidation_state() {
        let elemental = OxidationState::zero();

        assert_eq!(elemental.sign(), OxidationSign::Zero);
        assert_eq!(elemental.magnitude(), OxidationMagnitude::ZERO);
        assert_eq!(elemental.signed_value(), 0);
        assert!(elemental.is_zero());
        assert_eq!(elemental.to_string(), "0");
        assert_eq!(elemental.to_roman(), None);
    }

    #[test]
    fn rejects_invalid_state_combinations() {
        assert_eq!(
            OxidationState::positive(0),
            Err(OxidationStateValidationError::ZeroSignedMagnitude)
        );
        assert_eq!(
            OxidationState::negative(0),
            Err(OxidationStateValidationError::ZeroSignedMagnitude)
        );
        assert_eq!(
            OxidationState::new(
                OxidationSign::Zero,
                OxidationMagnitude::new(1).expect("magnitude should be valid"),
            ),
            Err(OxidationStateValidationError::NonZeroZeroMagnitude)
        );
        assert_eq!(
            OxidationMagnitude::new(9),
            Err(OxidationStateValidationError::MagnitudeAboveMaximum {
                magnitude: 9,
                maximum: OxidationMagnitude::MAX,
            })
        );
    }

    #[test]
    fn converts_to_roman_numerals() {
        assert_eq!(roman_numeral(0), None);
        assert_eq!(roman_numeral(1), Some("I"));
        assert_eq!(roman_numeral(2), Some("II"));
        assert_eq!(roman_numeral(3), Some("III"));
        assert_eq!(roman_numeral(4), Some("IV"));
        assert_eq!(roman_numeral(5), Some("V"));
        assert_eq!(roman_numeral(6), Some("VI"));
        assert_eq!(roman_numeral(7), Some("VII"));
        assert_eq!(roman_numeral(8), Some("VIII"));
        assert_eq!(roman_numeral(9), None);
        assert_eq!(positive(2).to_roman(), Some(String::from("II")));
        assert_eq!(negative(2).to_roman(), Some(String::from("II")));
        assert_eq!(
            RomanOxidationState::from_state(positive(7)).map(|roman| roman.to_string()),
            Some(String::from("VII"))
        );
    }

    #[test]
    fn assigns_element_oxidation_states() {
        let iron_two = ElementOxidationState::new("Fe", positive(2))
            .expect("element assignment should be valid");
        let iron_three = ElementOxidationState::new("Fe", positive(3))
            .expect("element assignment should be valid");
        let copper_one = ElementOxidationState::new("Cu", positive(1))
            .expect("element assignment should be valid");
        let copper_two = ElementOxidationState::new("Cu", positive(2))
            .expect("element assignment should be valid");
        let manganese_seven = ElementOxidationState::new("Mn", positive(7))
            .expect("element assignment should be valid");
        let oxygen = ElementOxidationState::new("O", negative(2))
            .expect("element assignment should be valid");
        let elemental_iron = ElementOxidationState::new("Fe", OxidationState::zero())
            .expect("element assignment should be valid");

        assert_eq!(iron_two.to_string(), "Fe(II)");
        assert_eq!(iron_three.to_string(), "Fe(III)");
        assert_eq!(copper_one.to_string(), "Cu(I)");
        assert_eq!(copper_two.to_string(), "Cu(II)");
        assert_eq!(manganese_seven.to_string(), "Mn(VII)");
        assert_eq!(oxygen.to_string(), "O(-2)");
        assert_eq!(elemental_iron.to_string(), "Fe(0)");
        assert_eq!(iron_three.element_symbol(), "Fe");
        assert_eq!(iron_three.state(), positive(3));
    }

    #[test]
    fn rejects_invalid_element_symbols() {
        assert_eq!(
            ElementOxidationState::new(" ", positive(1)),
            Err(OxidationStateValidationError::EmptyElementSymbol)
        );
        assert_eq!(
            ElementOxidationState::new("iron", positive(2)),
            Err(OxidationStateValidationError::InvalidElementSymbol(
                String::from("iron")
            ))
        );
        assert_eq!(
            ElementOxidationState::new("FE", positive(2)),
            Err(OxidationStateValidationError::InvalidElementSymbol(
                String::from("FE")
            ))
        );
    }

    #[test]
    fn displays_generic_assignments() {
        let hydrogen =
            OxidationStateAssignment::new("H", positive(1)).expect("assignment should be valid");
        let oxygen =
            OxidationStateAssignment::new("O", negative(2)).expect("assignment should be valid");
        let sodium =
            OxidationStateAssignment::new("Na", positive(1)).expect("assignment should be valid");
        let chlorine =
            OxidationStateAssignment::new("Cl", negative(1)).expect("assignment should be valid");

        assert_eq!(hydrogen.to_string(), "H: +1");
        assert_eq!(oxygen.to_string(), "O: -2");
        assert_eq!(sodium.to_string(), "Na: +1");
        assert_eq!(chlorine.to_string(), "Cl: -1");
        assert_eq!(
            OxidationStateAssignment::new(" ", positive(1)),
            Err(OxidationStateValidationError::EmptyAssignmentLabel)
        );
    }

    #[test]
    fn stores_formula_context_entries() {
        let mut states = OxidationStateSet::new();

        assert!(states.is_empty());
        assert_eq!(
            states.insert(
                OxidationStateAssignment::new("Fe", positive(2))
                    .expect("assignment should be valid")
            ),
            None
        );
        assert_eq!(
            states.insert(
                OxidationStateAssignment::new("O", negative(2))
                    .expect("assignment should be valid")
            ),
            None
        );
        let replaced = states.insert(
            OxidationStateAssignment::new("Fe", positive(3)).expect("assignment should be valid"),
        );

        assert_eq!(states.len(), 2);
        assert!(states.contains_label("Fe"));
        assert_eq!(
            states.get("Fe").map(OxidationStateAssignment::state),
            Some(positive(3))
        );
        assert_eq!(
            replaced.map(|assignment| assignment.state()),
            Some(positive(2))
        );
        assert_eq!(states.to_string(), "Fe: +3, O: -2");

        let formula =
            FormulaOxidationState::new("Fe2O3", states).expect("formula context should be valid");

        assert_eq!(formula.formula_label(), "Fe2O3");
        assert_eq!(formula.states().len(), 2);
        assert_eq!(formula.to_string(), "Fe2O3 [Fe: +3, O: -2]");
        assert_eq!(
            FormulaOxidationState::new(" ", OxidationStateSet::new()),
            Err(OxidationStateValidationError::EmptyFormulaLabel)
        );
    }

    #[test]
    fn builds_sets_from_assignments() {
        let assignments = [
            OxidationStateAssignment::new("S", positive(4)).expect("assignment should be valid"),
            OxidationStateAssignment::new("O", negative(2)).expect("assignment should be valid"),
            OxidationStateAssignment::new("S", positive(6)).expect("assignment should be valid"),
        ];
        let states = OxidationStateSet::from_assignments(assignments);

        assert_eq!(states.len(), 2);
        assert_eq!(
            states.get("S").map(OxidationStateAssignment::state),
            Some(positive(6))
        );
        assert_eq!(states.iter().count(), 2);
        assert_eq!(states.into_iter().count(), 2);
    }
}
