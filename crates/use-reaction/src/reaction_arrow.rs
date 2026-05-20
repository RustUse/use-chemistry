use std::fmt;

use crate::ReactionDirection;

/// A reaction arrow style.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReactionArrow {
    /// Forward reaction arrow.
    #[default]
    Forward,
    /// Reverse reaction arrow.
    Reverse,
    /// Reversible reaction arrow.
    Reversible,
    /// Equilibrium reaction arrow.
    Equilibrium,
}

impl ReactionArrow {
    /// Returns the lightweight direction label for this arrow.
    #[must_use]
    pub const fn direction(self) -> ReactionDirection {
        match self {
            Self::Forward => ReactionDirection::Forward,
            Self::Reverse => ReactionDirection::Reverse,
            Self::Reversible => ReactionDirection::Reversible,
            Self::Equilibrium => ReactionDirection::Equilibrium,
        }
    }

    /// Returns `true` for reversible or equilibrium arrows.
    #[must_use]
    pub const fn is_reversible(self) -> bool {
        matches!(self, Self::Reversible | Self::Equilibrium)
    }

    /// Returns `true` for [`Self::Forward`].
    #[must_use]
    pub const fn is_forward(self) -> bool {
        matches!(self, Self::Forward)
    }

    /// Returns `true` for [`Self::Reverse`].
    #[must_use]
    pub const fn is_reverse(self) -> bool {
        matches!(self, Self::Reverse)
    }
}

impl fmt::Display for ReactionArrow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Forward => "->",
            Self::Reverse => "<-",
            Self::Reversible => "<->",
            Self::Equilibrium => "⇌",
        };

        formatter.write_str(value)
    }
}
