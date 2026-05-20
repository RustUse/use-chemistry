use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{Ion, IonCharge, IonKind, IonValidationError};

/// A positive ion wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cation(Ion);

impl Cation {
    /// Creates a cation from a formula and positive charge magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ZeroChargeMagnitude`] when `magnitude` is zero.
    pub fn new(formula: ChemicalFormula, magnitude: u8) -> Result<Self, IonValidationError> {
        Ok(Self(
            Ion::new(formula, IonCharge::positive(magnitude)?).with_kind(IonKind::Cation),
        ))
    }

    /// Wraps an existing positive ion.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedCation`] when `ion` has a negative charge.
    pub fn from_ion(ion: Ion) -> Result<Self, IonValidationError> {
        if ion.is_cation() {
            Ok(Self(ion.with_kind(IonKind::Cation)))
        } else {
            Err(IonValidationError::ExpectedCation)
        }
    }

    /// Returns the wrapped ion.
    #[must_use]
    pub const fn as_ion(&self) -> &Ion {
        &self.0
    }

    /// Consumes the wrapper and returns the ion.
    #[must_use]
    pub fn into_ion(self) -> Ion {
        self.0
    }
}

impl fmt::Display for Cation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
