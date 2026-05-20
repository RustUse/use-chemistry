use std::fmt;

/// A formal molecule charge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoleculeCharge(i16);

impl MoleculeCharge {
    /// Neutral charge.
    pub const NEUTRAL: Self = Self(0);

    /// Creates a molecule charge.
    #[must_use]
    pub const fn new(charge: i16) -> Self {
        Self(charge)
    }

    /// Returns the signed charge value.
    #[must_use]
    pub const fn get(self) -> i16 {
        self.0
    }

    /// Returns `true` when the charge is neutral.
    #[must_use]
    pub const fn is_neutral(self) -> bool {
        self.0 == 0
    }
}

impl Default for MoleculeCharge {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

impl fmt::Display for MoleculeCharge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.cmp(&0) {
            std::cmp::Ordering::Greater => write!(formatter, "+{}", self.0),
            std::cmp::Ordering::Equal => formatter.write_str("0"),
            std::cmp::Ordering::Less => write!(formatter, "{}", self.0),
        }
    }
}
