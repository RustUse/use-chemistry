use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{
    CommonName, CompoundFormula, CompoundIdentifier, CompoundKind, CompoundName,
    CompoundValidationError, EmpiricalFormula, MolecularFormula, SystematicName,
};

/// A named chemical compound with formula and lightweight descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compound {
    name: CompoundName,
    formula: CompoundFormula,
    common_name: Option<CommonName>,
    systematic_name: Option<SystematicName>,
    empirical_formula: Option<EmpiricalFormula>,
    molecular_formula: Option<MolecularFormula>,
    kinds: Vec<CompoundKind>,
    identifiers: Vec<CompoundIdentifier>,
}

impl Compound {
    /// Creates a compound from a name and formula.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn new(name: &str, formula: ChemicalFormula) -> Result<Self, CompoundValidationError> {
        Ok(Self {
            name: CompoundName::new(name)?,
            formula: CompoundFormula::new(formula),
            common_name: None,
            systematic_name: None,
            empirical_formula: None,
            molecular_formula: None,
            kinds: Vec::new(),
            identifiers: Vec::new(),
        })
    }

    /// Returns the primary compound name.
    #[must_use]
    pub const fn name(&self) -> &CompoundName {
        &self.name
    }

    /// Returns the primary formula.
    #[must_use]
    pub fn formula(&self) -> &ChemicalFormula {
        self.formula.as_formula()
    }

    /// Returns the compound formula wrapper.
    #[must_use]
    pub const fn compound_formula(&self) -> &CompoundFormula {
        &self.formula
    }

    /// Returns the optional common name.
    #[must_use]
    pub const fn common_name(&self) -> Option<&CommonName> {
        self.common_name.as_ref()
    }

    /// Returns the optional systematic name.
    #[must_use]
    pub const fn systematic_name(&self) -> Option<&SystematicName> {
        self.systematic_name.as_ref()
    }

    /// Returns the optional empirical formula.
    #[must_use]
    pub const fn empirical_formula(&self) -> Option<&EmpiricalFormula> {
        self.empirical_formula.as_ref()
    }

    /// Returns the optional molecular formula.
    #[must_use]
    pub const fn molecular_formula(&self) -> Option<&MolecularFormula> {
        self.molecular_formula.as_ref()
    }

    /// Returns compound kind labels in insertion order.
    #[must_use]
    pub fn kinds(&self) -> &[CompoundKind] {
        &self.kinds
    }

    /// Returns compound identifiers in insertion order.
    #[must_use]
    pub fn identifiers(&self) -> &[CompoundIdentifier] {
        &self.identifiers
    }

    /// Adds a kind label if it is not already present.
    #[must_use]
    pub fn with_kind(mut self, kind: CompoundKind) -> Self {
        if !self.kinds.contains(&kind) {
            self.kinds.push(kind);
        }
        self
    }

    /// Sets the common name from a validated value.
    #[must_use]
    pub fn with_common_name(mut self, common_name: CommonName) -> Self {
        self.common_name = Some(common_name);
        self
    }

    /// Sets the common name after validation.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyCommonName`] when `common_name` is empty after trimming.
    pub fn try_with_common_name(self, common_name: &str) -> Result<Self, CompoundValidationError> {
        Ok(self.with_common_name(CommonName::new(common_name)?))
    }

    /// Sets the systematic name from a validated value.
    #[must_use]
    pub fn with_systematic_name(mut self, systematic_name: SystematicName) -> Self {
        self.systematic_name = Some(systematic_name);
        self
    }

    /// Sets the systematic name after validation.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptySystematicName`] when `systematic_name` is empty after trimming.
    pub fn try_with_systematic_name(
        self,
        systematic_name: &str,
    ) -> Result<Self, CompoundValidationError> {
        Ok(self.with_systematic_name(SystematicName::new(systematic_name)?))
    }

    /// Sets the empirical formula.
    #[must_use]
    pub fn with_empirical_formula(mut self, formula: EmpiricalFormula) -> Self {
        self.empirical_formula = Some(formula);
        self
    }

    /// Sets the molecular formula.
    #[must_use]
    pub fn with_molecular_formula(mut self, formula: MolecularFormula) -> Self {
        self.molecular_formula = Some(formula);
        self
    }

    /// Adds an identifier if it is not already present.
    ///
    /// # Errors
    ///
    /// This method currently cannot fail because [`CompoundIdentifier`] values are validated at
    /// construction, but it returns `Result` so builder chains can stay consistently fallible.
    pub fn try_with_identifier(
        mut self,
        identifier: CompoundIdentifier,
    ) -> Result<Self, CompoundValidationError> {
        if !self.identifiers.contains(&identifier) {
            self.identifiers.push(identifier);
        }
        Ok(self)
    }
}

impl fmt::Display for Compound {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.formula)
    }
}
