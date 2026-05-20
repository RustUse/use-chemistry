use std::fmt;

/// A lightweight bond strength label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BondStrength {
    /// Weak bond or interaction.
    Weak,
    /// Moderate bond or interaction.
    Moderate,
    /// Strong bond or interaction.
    Strong,
    /// Very strong bond or interaction.
    VeryStrong,
    /// Unknown or intentionally unspecified bond strength.
    Unknown,
}

impl fmt::Display for BondStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Weak => "weak",
            Self::Moderate => "moderate",
            Self::Strong => "strong",
            Self::VeryStrong => "very strong",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
