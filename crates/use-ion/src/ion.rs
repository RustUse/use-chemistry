use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{IonCharge, IonFormula, IonKind, IonName, IonValidationError};

/// A charged atom or charged group represented by formula and charge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ion {
    formula: IonFormula,
    charge: IonCharge,
    name: Option<IonName>,
    kinds: Vec<IonKind>,
    oxidation_state_label: Option<String>,
}

impl Ion {
    /// Creates an ion from a parsed formula and validated charge.
    #[must_use]
    pub fn new(formula: ChemicalFormula, charge: IonCharge) -> Self {
        Self {
            formula: IonFormula::new(formula),
            charge,
            name: None,
            kinds: Vec::new(),
            oxidation_state_label: None,
        }
    }

    /// Returns the chemical formula.
    #[must_use]
    pub fn formula(&self) -> &ChemicalFormula {
        self.formula.as_formula()
    }

    /// Returns the ion formula wrapper.
    #[must_use]
    pub const fn ion_formula(&self) -> &IonFormula {
        &self.formula
    }

    /// Returns the ionic charge.
    #[must_use]
    pub const fn charge(&self) -> IonCharge {
        self.charge
    }

    /// Returns the optional ion name.
    #[must_use]
    pub const fn name(&self) -> Option<&IonName> {
        self.name.as_ref()
    }

    /// Returns ion kind labels in insertion order.
    #[must_use]
    pub fn kinds(&self) -> &[IonKind] {
        &self.kinds
    }

    /// Returns the optional oxidation-state label.
    #[must_use]
    pub fn oxidation_state_label(&self) -> Option<&str> {
        self.oxidation_state_label.as_deref()
    }

    /// Returns `true` when the ion has a positive charge.
    #[must_use]
    pub const fn is_cation(&self) -> bool {
        self.charge.is_cation()
    }

    /// Returns `true` when the ion has a negative charge.
    #[must_use]
    pub const fn is_anion(&self) -> bool {
        self.charge.is_anion()
    }

    /// Adds a kind label if it is not already present.
    #[must_use]
    pub fn with_kind(mut self, kind: IonKind) -> Self {
        if !self.kinds.contains(&kind) {
            self.kinds.push(kind);
        }
        self
    }

    /// Sets the ion name from a validated value.
    #[must_use]
    pub fn with_name(mut self, name: IonName) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the ion name after validation.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn try_with_name(self, name: &str) -> Result<Self, IonValidationError> {
        Ok(self.with_name(IonName::new(name)?))
    }

    /// Sets the oxidation-state label from a prevalidated value.
    #[must_use]
    pub fn with_oxidation_state_label(mut self, label: String) -> Self {
        self.oxidation_state_label = Some(label);
        self
    }

    /// Sets the oxidation-state label after validation.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::EmptyOxidationStateLabel`] when `label` is empty after trimming.
    pub fn try_with_oxidation_state_label(self, label: &str) -> Result<Self, IonValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(IonValidationError::EmptyOxidationStateLabel)
        } else {
            Ok(self.with_oxidation_state_label(trimmed.to_owned()))
        }
    }
}

impl fmt::Display for Ion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.charge.magnitude() == 1 {
            write!(formatter, "{}{}", self.formula, self.charge.sign())
        } else {
            write!(
                formatter,
                "{}^{}{}",
                self.formula,
                self.charge.magnitude(),
                self.charge.sign()
            )
        }
    }
}
