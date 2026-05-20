use std::fmt;

/// A lightweight ion classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IonKind {
    /// Monatomic ion.
    Monatomic,
    /// Polyatomic ion.
    Polyatomic,
    /// Cation label.
    Cation,
    /// Anion label.
    Anion,
    /// Radical ion label.
    RadicalIon,
    /// Zwitterion label.
    Zwitterion,
    /// Complex ion label.
    ComplexIon,
    /// Unknown or intentionally unspecified classification.
    Unknown,
}

impl fmt::Display for IonKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Monatomic => "monatomic",
            Self::Polyatomic => "polyatomic",
            Self::Cation => "cation",
            Self::Anion => "anion",
            Self::RadicalIon => "radical ion",
            Self::Zwitterion => "zwitterion",
            Self::ComplexIon => "complex ion",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
