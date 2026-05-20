use std::fmt;

use crate::{OxidationState, OxidationStateValidationError};

/// An oxidation-state assignment for an element symbol.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ElementOxidationState {
    element_symbol: String,
    state: OxidationState,
}

impl ElementOxidationState {
    /// Creates an element oxidation-state assignment.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::EmptyElementSymbol`] when `element_symbol`
    /// is empty or whitespace only, or
    /// [`OxidationStateValidationError::InvalidElementSymbol`] when the symbol does not
    /// match the supported shape.
    pub fn new(
        element_symbol: &str,
        state: OxidationState,
    ) -> Result<Self, OxidationStateValidationError> {
        let element_symbol = validate_element_symbol(element_symbol)?;

        Ok(Self {
            element_symbol,
            state,
        })
    }

    /// Returns the element symbol.
    #[must_use]
    pub fn element_symbol(&self) -> &str {
        &self.element_symbol
    }

    /// Returns the assigned oxidation state.
    #[must_use]
    pub const fn state(&self) -> OxidationState {
        self.state
    }

    /// Consumes the assignment and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (String, OxidationState) {
        (self.element_symbol, self.state)
    }
}

impl fmt::Display for ElementOxidationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.state.is_positive() {
            if let Some(roman) = self.state.to_roman() {
                write!(formatter, "{}({roman})", self.element_symbol)
            } else {
                write!(formatter, "{}({})", self.element_symbol, self.state)
            }
        } else {
            write!(formatter, "{}({})", self.element_symbol, self.state)
        }
    }
}

fn validate_element_symbol(symbol: &str) -> Result<String, OxidationStateValidationError> {
    let symbol = symbol.trim();

    if symbol.is_empty() {
        return Err(OxidationStateValidationError::EmptyElementSymbol);
    }

    if is_valid_element_symbol(symbol) {
        Ok(symbol.to_owned())
    } else {
        Err(OxidationStateValidationError::InvalidElementSymbol(
            symbol.to_owned(),
        ))
    }
}

fn is_valid_element_symbol(symbol: &str) -> bool {
    let mut characters = symbol.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    if !first.is_ascii_uppercase() {
        return false;
    }

    match characters.next() {
        None => true,
        Some(second) if second.is_ascii_lowercase() => characters.next().is_none(),
        Some(_) => false,
    }
}
