use std::collections::BTreeMap;

use use_atomic_mass::atomic_mass_by_symbol;
use use_chemical_formula::{ChemicalFormula, is_valid_element_symbol};

use crate::{AtomicMassEntry, MolarMassValidationError};

/// A caller-controlled element atomic-mass lookup table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AtomicMassLookup {
    entries: BTreeMap<String, f64>,
}

impl AtomicMassLookup {
    /// Creates an empty lookup table.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Creates a lookup table from validated entries.
    #[must_use]
    pub fn from_entries(entries: impl IntoIterator<Item = AtomicMassEntry>) -> Self {
        let mut lookup = Self::new();

        for entry in entries {
            lookup.insert(entry);
        }

        lookup
    }

    /// Creates a lookup table from symbol and mass pairs.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error when any entry is invalid.
    pub fn from_pairs<'a>(
        entries: impl IntoIterator<Item = (&'a str, f64)>,
    ) -> Result<Self, MolarMassValidationError> {
        let mut lookup = Self::new();

        for (symbol, atomic_mass) in entries {
            lookup.insert_atomic_mass(symbol, atomic_mass)?;
        }

        Ok(lookup)
    }

    /// Creates a lookup table for the symbols present in a formula using RustUse atomic masses.
    ///
    /// # Errors
    ///
    /// Returns [`MolarMassValidationError::MissingAtomicMass`] if a formula symbol
    /// is not present in the RustUse atomic-mass table. Returns another
    /// molar-mass validation error if a generated entry is invalid.
    pub fn from_formula(formula: &ChemicalFormula) -> Result<Self, MolarMassValidationError> {
        let mut lookup = Self::new();

        for (symbol, count) in formula.element_counts() {
            if count == 0 {
                return Err(MolarMassValidationError::ZeroElementCount { symbol });
            }

            let atomic_mass = atomic_mass_by_symbol(&symbol).ok_or_else(|| {
                MolarMassValidationError::MissingAtomicMass {
                    symbol: symbol.clone(),
                }
            })?;
            lookup.insert_atomic_mass(&symbol, atomic_mass)?;
        }

        Ok(lookup)
    }

    /// Inserts a validated atomic mass entry and returns the previous value, if any.
    pub fn insert(&mut self, entry: AtomicMassEntry) -> Option<f64> {
        self.entries
            .insert(entry.symbol().to_owned(), entry.atomic_mass())
    }

    /// Validates and inserts an atomic mass entry.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error when the symbol or atomic mass is invalid.
    pub fn insert_atomic_mass(
        &mut self,
        symbol: &str,
        atomic_mass: f64,
    ) -> Result<Option<f64>, MolarMassValidationError> {
        AtomicMassEntry::new(symbol, atomic_mass).map(|entry| self.insert(entry))
    }

    /// Returns the atomic mass for a symbol.
    #[must_use]
    pub fn atomic_mass(&self, symbol: &str) -> Option<f64> {
        self.entries.get(symbol).copied()
    }

    /// Returns a copy of the stored entry for a symbol.
    #[must_use]
    pub fn entry(&self, symbol: &str) -> Option<AtomicMassEntry> {
        self.entries
            .get_key_value(symbol)
            .map(|(stored_symbol, atomic_mass)| {
                AtomicMassEntry::from_validated(stored_symbol.clone(), *atomic_mass)
            })
    }

    /// Returns true when the lookup contains a symbol.
    #[must_use]
    pub fn contains_symbol(&self, symbol: &str) -> bool {
        self.entries.contains_key(symbol)
    }

    /// Returns the number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the lookup table is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates over stored symbol and atomic-mass pairs in symbol order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> + '_ {
        self.entries
            .iter()
            .map(|(symbol, atomic_mass)| (symbol.as_str(), *atomic_mass))
    }
}

pub(crate) fn validate_symbol(symbol: &str) -> Result<&str, MolarMassValidationError> {
    let symbol = symbol.trim();

    if is_valid_element_symbol(symbol) {
        Ok(symbol)
    } else {
        Err(MolarMassValidationError::InvalidElementSymbol(
            symbol.to_owned(),
        ))
    }
}
