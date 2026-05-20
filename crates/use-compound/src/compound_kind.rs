use std::fmt;

/// A lightweight compound classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompoundKind {
    /// A molecular compound.
    Molecular,
    /// An ionic compound.
    Ionic,
    /// An organic compound.
    Organic,
    /// An inorganic compound.
    Inorganic,
    /// An acid.
    Acid,
    /// A base.
    Base,
    /// A salt.
    Salt,
    /// A hydrate.
    Hydrate,
    /// A coordination compound.
    Coordination,
    /// Unknown or intentionally unspecified classification.
    Unknown,
}

impl fmt::Display for CompoundKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Molecular => "molecular",
            Self::Ionic => "ionic",
            Self::Organic => "organic",
            Self::Inorganic => "inorganic",
            Self::Acid => "acid",
            Self::Base => "base",
            Self::Salt => "salt",
            Self::Hydrate => "hydrate",
            Self::Coordination => "coordination",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
