use std::fmt;

use crate::{OxidationState, OxidationStateValidationError};

/// A labeled oxidation-state assignment.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OxidationStateAssignment {
    label: String,
    state: OxidationState,
}

impl OxidationStateAssignment {
    /// Creates an oxidation-state assignment.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::EmptyAssignmentLabel`] when `label` is
    /// empty or whitespace only.
    pub fn new(label: &str, state: OxidationState) -> Result<Self, OxidationStateValidationError> {
        let label = label.trim();

        if label.is_empty() {
            Err(OxidationStateValidationError::EmptyAssignmentLabel)
        } else {
            Ok(Self {
                label: label.to_owned(),
                state,
            })
        }
    }

    /// Returns the assignment label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the assigned oxidation state.
    #[must_use]
    pub const fn state(&self) -> OxidationState {
        self.state
    }

    /// Consumes the assignment and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (String, OxidationState) {
        (self.label, self.state)
    }
}

impl fmt::Display for OxidationStateAssignment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.label, self.state)
    }
}
