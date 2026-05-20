use std::fmt;

/// A reaction-side label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReactionSide {
    /// Reactant side.
    Reactant,
    /// Product side.
    Product,
}

impl ReactionSide {
    /// Returns `true` for [`Self::Reactant`].
    #[must_use]
    pub const fn is_reactant(self) -> bool {
        matches!(self, Self::Reactant)
    }

    /// Returns `true` for [`Self::Product`].
    #[must_use]
    pub const fn is_product(self) -> bool {
        matches!(self, Self::Product)
    }
}

impl fmt::Display for ReactionSide {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reactant => formatter.write_str("reactant"),
            Self::Product => formatter.write_str("product"),
        }
    }
}
