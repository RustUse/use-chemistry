use std::fmt;

use crate::{CompoundRegistry, CompoundValidationError};

/// A lightweight compound registry identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompoundIdentifier {
    registry: CompoundRegistry,
    value: String,
}

impl CompoundIdentifier {
    /// Creates a CAS Registry Number identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn cas_number(value: &str) -> Result<Self, CompoundValidationError> {
        Self::new(CompoundRegistry::CasNumber, value)
    }

    /// Creates a PubChem CID identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn pub_chem_cid(value: &str) -> Result<Self, CompoundValidationError> {
        Self::new(CompoundRegistry::PubChemCid, value)
    }

    /// Creates an InChI identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn inchi(value: &str) -> Result<Self, CompoundValidationError> {
        Self::new(CompoundRegistry::Inchi, value)
    }

    /// Creates an InChIKey identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn inchi_key(value: &str) -> Result<Self, CompoundValidationError> {
        Self::new(CompoundRegistry::InchiKey, value)
    }

    /// Creates a SMILES identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn smiles(value: &str) -> Result<Self, CompoundValidationError> {
        Self::new(CompoundRegistry::Smiles, value)
    }

    /// Creates a custom registry identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierNamespace`] when `namespace` is empty after
    /// trimming, or [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn custom(namespace: &str, value: &str) -> Result<Self, CompoundValidationError> {
        let trimmed_namespace = namespace.trim();
        if trimmed_namespace.is_empty() {
            return Err(CompoundValidationError::EmptyIdentifierNamespace);
        }

        Self::new(
            CompoundRegistry::Custom(trimmed_namespace.to_owned()),
            value,
        )
    }

    /// Creates an identifier from a registry and value.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyIdentifierValue`] when `value` is empty after trimming.
    pub fn new(registry: CompoundRegistry, value: &str) -> Result<Self, CompoundValidationError> {
        let trimmed_value = value.trim();
        if trimmed_value.is_empty() {
            Err(CompoundValidationError::EmptyIdentifierValue)
        } else {
            Ok(Self {
                registry,
                value: trimmed_value.to_owned(),
            })
        }
    }

    /// Returns the registry namespace.
    #[must_use]
    pub const fn registry(&self) -> &CompoundRegistry {
        &self.registry
    }

    /// Returns the identifier value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for CompoundIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.registry, self.value)
    }
}
