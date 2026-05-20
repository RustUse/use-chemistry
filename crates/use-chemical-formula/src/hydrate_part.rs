use std::collections::BTreeMap;
use std::fmt;

use crate::{FormulaMultiplier, FormulaPart};

/// A dot-separated hydrate formula part.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HydratePart {
    multiplier: FormulaMultiplier,
    part: FormulaPart,
}

impl HydratePart {
    /// Creates a hydrate formula part.
    #[must_use]
    pub const fn new(multiplier: FormulaMultiplier, part: FormulaPart) -> Self {
        Self { multiplier, part }
    }

    /// Returns the hydrate multiplier.
    #[must_use]
    pub const fn multiplier(&self) -> FormulaMultiplier {
        self.multiplier
    }

    /// Returns the hydrate formula part.
    #[must_use]
    pub const fn part(&self) -> &FormulaPart {
        &self.part
    }

    pub(crate) fn add_counts(&self, counts: &mut BTreeMap<String, u64>) {
        self.part
            .add_counts(counts, u64::from(self.multiplier.get()));
    }
}

impl fmt::Display for HydratePart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.multiplier.is_one() {
            write!(formatter, "{}", self.multiplier)?;
        }
        write!(formatter, "{}", self.part)
    }
}
