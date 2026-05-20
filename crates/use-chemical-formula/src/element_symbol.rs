use std::fmt;

use crate::FormulaValidationError;

/// A validated chemical element symbol shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElementSymbol(String);

impl ElementSymbol {
    /// Creates an element symbol after basic shape validation.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaValidationError::InvalidSymbol`] when the input is not an ASCII
    /// uppercase letter followed by zero or one ASCII lowercase letter.
    pub fn new(symbol: &str) -> Result<Self, FormulaValidationError> {
        if is_valid_element_symbol(symbol) {
            Ok(Self(symbol.to_owned()))
        } else {
            Err(FormulaValidationError::InvalidSymbol(symbol.to_owned()))
        }
    }

    /// Returns the symbol text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the symbol and returns the owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ElementSymbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for ElementSymbol {
    type Error = FormulaValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for ElementSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Returns `true` when the value has a basic chemical element-symbol shape.
#[must_use]
pub fn is_valid_element_symbol(symbol: &str) -> bool {
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
