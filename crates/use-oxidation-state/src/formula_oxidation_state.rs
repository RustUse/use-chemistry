use std::fmt;

use crate::{OxidationStateSet, OxidationStateValidationError};

/// Oxidation-state assignments in a formula or caller-defined formula context.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormulaOxidationState {
    formula_label: String,
    states: OxidationStateSet,
}

impl FormulaOxidationState {
    /// Creates formula-context oxidation-state assignments.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::EmptyFormulaLabel`] when `formula_label`
    /// is empty or whitespace only.
    pub fn new(
        formula_label: &str,
        states: OxidationStateSet,
    ) -> Result<Self, OxidationStateValidationError> {
        let formula_label = formula_label.trim();

        if formula_label.is_empty() {
            Err(OxidationStateValidationError::EmptyFormulaLabel)
        } else {
            Ok(Self {
                formula_label: formula_label.to_owned(),
                states,
            })
        }
    }

    /// Returns the formula or formula-context label.
    #[must_use]
    pub fn formula_label(&self) -> &str {
        &self.formula_label
    }

    /// Returns the oxidation-state assignments.
    #[must_use]
    pub const fn states(&self) -> &OxidationStateSet {
        &self.states
    }

    /// Consumes the value and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (String, OxidationStateSet) {
        (self.formula_label, self.states)
    }
}

impl fmt::Display for FormulaOxidationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.states.is_empty() {
            formatter.write_str(&self.formula_label)
        } else {
            write!(formatter, "{} [{}]", self.formula_label, self.states)
        }
    }
}
