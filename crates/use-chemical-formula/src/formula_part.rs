use std::collections::BTreeMap;
use std::fmt;

use crate::{FormulaTerm, FormulaValidationError};

/// A contiguous formula part made of terms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaPart {
    terms: Vec<FormulaTerm>,
}

impl FormulaPart {
    /// Creates a formula part.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaValidationError::EmptyPart`] when `terms` is empty.
    pub fn new(terms: Vec<FormulaTerm>) -> Result<Self, FormulaValidationError> {
        if terms.is_empty() {
            Err(FormulaValidationError::EmptyPart)
        } else {
            Ok(Self { terms })
        }
    }

    /// Returns the terms in this part.
    #[must_use]
    pub fn terms(&self) -> &[FormulaTerm] {
        &self.terms
    }

    pub(crate) fn add_counts(&self, counts: &mut BTreeMap<String, u64>, multiplier: u64) {
        for term in &self.terms {
            term.add_counts(counts, multiplier);
        }
    }
}

impl fmt::Display for FormulaPart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for term in &self.terms {
            write!(formatter, "{term}")?;
        }
        Ok(())
    }
}
