use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{Ion, IonCharge, IonKind, IonValidationError};

/// A monatomic ion wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonatomicIon(Ion);

impl MonatomicIon {
    /// Creates a monatomic ion from a formula and charge.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedMonatomicFormula`] when `formula` does not contain
    /// exactly one atom.
    pub fn new(formula: ChemicalFormula, charge: IonCharge) -> Result<Self, IonValidationError> {
        Self::from_ion(Ion::new(formula, charge))
    }

    /// Wraps an existing ion after monatomic formula validation.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedMonatomicFormula`] when the ion formula does not
    /// contain exactly one atom.
    pub fn from_ion(ion: Ion) -> Result<Self, IonValidationError> {
        if is_monatomic(ion.formula()) {
            Ok(Self(ion.with_kind(IonKind::Monatomic)))
        } else {
            Err(IonValidationError::ExpectedMonatomicFormula)
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

impl fmt::Display for MonatomicIon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

fn is_monatomic(formula: &ChemicalFormula) -> bool {
    let counts = formula.element_counts();
    counts.len() == 1 && counts.values().copied().sum::<u64>() == 1
}
