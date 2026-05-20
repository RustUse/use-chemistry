use std::fmt;

/// The sign of an ionic charge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChargeSign {
    /// Positive charge.
    Positive,
    /// Negative charge.
    Negative,
}

impl ChargeSign {
    /// Returns `true` for a positive charge sign.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` for a negative charge sign.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }
}

impl fmt::Display for ChargeSign {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Positive => "+",
            Self::Negative => "-",
        };

        formatter.write_str(value)
    }
}
