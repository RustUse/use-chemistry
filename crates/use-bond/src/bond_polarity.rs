use std::fmt;

/// A lightweight bond polarity label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BondPolarity {
    /// Nonpolar bond.
    Nonpolar,
    /// Polar bond.
    Polar,
    /// Ionic bond polarity.
    Ionic,
    /// Unknown or intentionally unspecified bond polarity.
    Unknown,
}

impl fmt::Display for BondPolarity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Nonpolar => "nonpolar",
            Self::Polar => "polar",
            Self::Ionic => "ionic",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
