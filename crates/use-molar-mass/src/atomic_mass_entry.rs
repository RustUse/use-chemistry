use std::fmt;

use use_chemical_formula::is_valid_element_symbol;

use crate::MolarMassValidationError;

/// A validated element symbol and atomic mass pair.
#[derive(Clone, Debug, PartialEq)]
pub struct AtomicMassEntry {
    symbol: String,
    atomic_mass: f64,
}

impl AtomicMassEntry {
    /// Creates an atomic mass entry in grams per mole.
    ///
    /// # Errors
    ///
    /// Returns [`MolarMassValidationError::InvalidElementSymbol`] when `symbol`
    /// does not match a chemical element symbol shape. Returns an atomic-mass
    /// validation error when `atomic_mass` is not finite and positive.
    pub fn new(symbol: &str, atomic_mass: f64) -> Result<Self, MolarMassValidationError> {
        let symbol = symbol.trim();

        if !is_valid_element_symbol(symbol) {
            return Err(MolarMassValidationError::InvalidElementSymbol(
                symbol.to_owned(),
            ));
        }

        validate_atomic_mass(symbol, atomic_mass)?;

        Ok(Self {
            symbol: symbol.to_owned(),
            atomic_mass,
        })
    }

    pub(crate) fn from_validated(symbol: String, atomic_mass: f64) -> Self {
        Self {
            symbol,
            atomic_mass,
        }
    }

    /// Returns the element symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the atomic mass in grams per mole.
    #[must_use]
    pub const fn atomic_mass(&self) -> f64 {
        self.atomic_mass
    }
}

impl TryFrom<(&str, f64)> for AtomicMassEntry {
    type Error = MolarMassValidationError;

    fn try_from(value: (&str, f64)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl fmt::Display for AtomicMassEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {} g/mol", self.symbol, self.atomic_mass)
    }
}

pub(crate) fn validate_atomic_mass(
    symbol: &str,
    atomic_mass: f64,
) -> Result<(), MolarMassValidationError> {
    if !atomic_mass.is_finite() {
        Err(MolarMassValidationError::NonFiniteAtomicMass {
            symbol: symbol.to_owned(),
        })
    } else if atomic_mass <= 0.0 {
        Err(MolarMassValidationError::NonPositiveAtomicMass {
            symbol: symbol.to_owned(),
        })
    } else {
        Ok(())
    }
}
