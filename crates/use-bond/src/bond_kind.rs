use std::fmt;

/// A lightweight chemical bond kind label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BondKind {
    /// Covalent bond.
    Covalent,
    /// Ionic bond.
    Ionic,
    /// Metallic bond.
    Metallic,
    /// Coordinate bond.
    Coordinate,
    /// Hydrogen bond.
    Hydrogen,
    /// Aromatic bond.
    Aromatic,
    /// Van der Waals interaction.
    VanDerWaals,
    /// Dipole-dipole interaction.
    DipoleDipole,
    /// London dispersion interaction.
    LondonDispersion,
    /// Unknown or intentionally unspecified bond kind.
    Unknown,
}

impl fmt::Display for BondKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Covalent => "covalent",
            Self::Ionic => "ionic",
            Self::Metallic => "metallic",
            Self::Coordinate => "coordinate",
            Self::Hydrogen => "hydrogen",
            Self::Aromatic => "aromatic",
            Self::VanDerWaals => "van der Waals",
            Self::DipoleDipole => "dipole-dipole",
            Self::LondonDispersion => "London dispersion",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
