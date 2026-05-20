use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{Ion, IonCharge, IonKind, IonValidationError};

/// A negative ion wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Anion(Ion);

impl Anion {
    /// Creates an anion from a formula and negative charge magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ZeroChargeMagnitude`] when `magnitude` is zero.
    pub fn new(formula: ChemicalFormula, magnitude: u8) -> Result<Self, IonValidationError> {
        Ok(Self(
            Ion::new(formula, IonCharge::negative(magnitude)?).with_kind(IonKind::Anion),
        ))
    }

    /// Wraps an existing negative ion.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedAnion`] when `ion` has a positive charge.
    pub fn from_ion(ion: Ion) -> Result<Self, IonValidationError> {
        if ion.is_anion() {
            Ok(Self(ion.with_kind(IonKind::Anion)))
        } else {
            Err(IonValidationError::ExpectedAnion)
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

impl fmt::Display for Anion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
