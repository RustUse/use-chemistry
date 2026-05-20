use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use crate::{FormulaParseError, FormulaPart, HydratePart, parse};

/// A chemical formula with a main part and optional hydrate parts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChemicalFormula {
    main_part: FormulaPart,
    hydrate_parts: Vec<HydratePart>,
}

impl ChemicalFormula {
    /// Creates a chemical formula from a main part and hydrate parts.
    #[must_use]
    pub fn new(main_part: FormulaPart, hydrate_parts: Vec<HydratePart>) -> Self {
        Self {
            main_part,
            hydrate_parts,
        }
    }

    /// Parses a chemical formula string.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaParseError`] when the input is empty or does not match the supported
    /// lightweight formula grammar.
    pub fn parse(input: &str) -> Result<Self, FormulaParseError> {
        parse::parse_formula(input)
    }

    /// Returns the main formula part.
    #[must_use]
    pub const fn main_part(&self) -> &FormulaPart {
        &self.main_part
    }

    /// Returns hydrate parts after the main formula part.
    #[must_use]
    pub fn hydrate_parts(&self) -> &[HydratePart] {
        &self.hydrate_parts
    }

    /// Returns expanded element counts for the full formula.
    #[must_use]
    pub fn element_counts(&self) -> BTreeMap<String, u64> {
        let mut counts = BTreeMap::new();
        self.main_part.add_counts(&mut counts, 1);
        for hydrate_part in &self.hydrate_parts {
            hydrate_part.add_counts(&mut counts);
        }
        counts
    }
}

impl fmt::Display for ChemicalFormula {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.main_part)?;
        for hydrate_part in &self.hydrate_parts {
            write!(formatter, "·{hydrate_part}")?;
        }
        Ok(())
    }
}

impl FromStr for ChemicalFormula {
    type Err = FormulaParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}
