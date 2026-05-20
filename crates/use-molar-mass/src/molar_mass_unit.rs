use std::fmt;

/// A display unit for molar mass values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MolarMassUnit {
    /// Grams per mole.
    #[default]
    GramsPerMole,
    /// Kilograms per mole.
    KilogramsPerMole,
}

impl MolarMassUnit {
    /// Returns the abbreviated unit label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GramsPerMole => "g/mol",
            Self::KilogramsPerMole => "kg/mol",
        }
    }
}

impl fmt::Display for MolarMassUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
