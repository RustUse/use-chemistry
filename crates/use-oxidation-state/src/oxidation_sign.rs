use std::fmt;

/// The sign of an oxidation-state value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OxidationSign {
    /// A positive oxidation state.
    Positive,
    /// A negative oxidation state.
    Negative,
    /// A zero oxidation state.
    Zero,
}

impl OxidationSign {
    /// Returns `true` for [`Self::Positive`].
    #[must_use]
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` for [`Self::Negative`].
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }

    /// Returns `true` for [`Self::Zero`].
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Zero)
    }
}

impl fmt::Display for OxidationSign {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => formatter.write_str("+"),
            Self::Negative => formatter.write_str("-"),
            Self::Zero => formatter.write_str("0"),
        }
    }
}
