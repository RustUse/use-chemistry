use std::fmt;

/// A lightweight reaction classification label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReactionKind {
    /// Synthesis reaction label.
    Synthesis,
    /// Decomposition reaction label.
    Decomposition,
    /// Single-replacement reaction label.
    SingleReplacement,
    /// Double-replacement reaction label.
    DoubleReplacement,
    /// Combustion reaction label.
    Combustion,
    /// Acid-base reaction label.
    AcidBase,
    /// Redox reaction label.
    Redox,
    /// Precipitation reaction label.
    Precipitation,
    /// Neutralization reaction label.
    Neutralization,
    /// Polymerization reaction label.
    Polymerization,
    /// Unknown or intentionally unspecified classification.
    Unknown,
}

impl fmt::Display for ReactionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Synthesis => "synthesis",
            Self::Decomposition => "decomposition",
            Self::SingleReplacement => "single replacement",
            Self::DoubleReplacement => "double replacement",
            Self::Combustion => "combustion",
            Self::AcidBase => "acid-base",
            Self::Redox => "redox",
            Self::Precipitation => "precipitation",
            Self::Neutralization => "neutralization",
            Self::Polymerization => "polymerization",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}
