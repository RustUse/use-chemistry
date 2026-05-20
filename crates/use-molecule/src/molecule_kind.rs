use std::fmt;

/// A lightweight molecule classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MoleculeKind {
    /// Neutral molecule.
    Neutral,
    /// Molecular ion.
    Ion,
    /// Radical.
    Radical,
    /// Diatomic molecule.
    Diatomic,
    /// Polyatomic molecule.
    Polyatomic,
    /// Organic molecule.
    Organic,
    /// Inorganic molecule.
    Inorganic,
    /// Biomolecule.
    Biomolecule,
    /// Unknown or intentionally unspecified classification.
    Unknown,
}

impl fmt::Display for MoleculeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Neutral => "neutral",
            Self::Ion => "ion",
            Self::Radical => "radical",
            Self::Diatomic => "diatomic",
            Self::Polyatomic => "polyatomic",
            Self::Organic => "organic",
            Self::Inorganic => "inorganic",
            Self::Biomolecule => "biomolecule",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
