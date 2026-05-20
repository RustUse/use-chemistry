use std::collections::BTreeMap;
use std::fmt;

use crate::{FormulaMultiplier, FormulaTerm, FormulaValidationError};

/// Parenthesized formula terms with a multiplier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaGroup {
    terms: Vec<FormulaTerm>,
    multiplier: FormulaMultiplier,
}

impl FormulaGroup {
    /// Creates a formula group.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaValidationError::EmptyGroup`] when `terms` is empty.
    pub fn new(
        terms: Vec<FormulaTerm>,
        multiplier: FormulaMultiplier,
    ) -> Result<Self, FormulaValidationError> {
        if terms.is_empty() {
            Err(FormulaValidationError::EmptyGroup)
        } else {
            Ok(Self { terms, multiplier })
        }
    }

    /// Returns the terms inside the group.
    #[must_use]
    pub fn terms(&self) -> &[FormulaTerm] {
        &self.terms
    }

    /// Returns the group multiplier.
    #[must_use]
    pub const fn multiplier(&self) -> FormulaMultiplier {
        self.multiplier
    }

    pub(crate) fn add_counts(&self, counts: &mut BTreeMap<String, u64>, multiplier: u64) {
        let combined_multiplier = multiplier.saturating_mul(u64::from(self.multiplier.get()));
        for term in &self.terms {
            term.add_counts(counts, combined_multiplier);
        }
    }
}

impl fmt::Display for FormulaGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("(")?;
        for term in &self.terms {
            write!(formatter, "{term}")?;
        }
        formatter.write_str(")")?;
        if !self.multiplier.is_one() {
            write!(formatter, "{}", self.multiplier)?;
        }
        Ok(())
    }
}
