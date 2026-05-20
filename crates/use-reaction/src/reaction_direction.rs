use std::fmt;

/// A lightweight reaction direction label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReactionDirection {
    /// Forward direction.
    Forward,
    /// Reverse direction.
    Reverse,
    /// Reversible direction.
    Reversible,
    /// Equilibrium direction.
    Equilibrium,
}

impl fmt::Display for ReactionDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Forward => "forward",
            Self::Reverse => "reverse",
            Self::Reversible => "reversible",
            Self::Equilibrium => "equilibrium",
        };

        formatter.write_str(value)
    }
}
