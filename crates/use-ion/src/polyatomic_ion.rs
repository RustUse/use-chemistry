use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{Ion, IonCharge, IonKind, IonValidationError};

/// A polyatomic ion wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolyatomicIon(Ion);

impl PolyatomicIon {
    /// Creates a polyatomic ion from a formula and charge.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedPolyatomicFormula`] when `formula` does not contain
    /// more than one atom.
    pub fn new(formula: ChemicalFormula, charge: IonCharge) -> Result<Self, IonValidationError> {
        Self::from_ion(Ion::new(formula, charge))
    }

    /// Wraps an existing ion after polyatomic formula validation.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ExpectedPolyatomicFormula`] when the ion formula does not
    /// contain more than one atom.
    pub fn from_ion(ion: Ion) -> Result<Self, IonValidationError> {
        if is_polyatomic(ion.formula()) {
            Ok(Self(ion.with_kind(IonKind::Polyatomic)))
        } else {
            Err(IonValidationError::ExpectedPolyatomicFormula)
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

impl fmt::Display for PolyatomicIon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

fn is_polyatomic(formula: &ChemicalFormula) -> bool {
    formula.element_counts().values().copied().sum::<u64>() > 1
}
