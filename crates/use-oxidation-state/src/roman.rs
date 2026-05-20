use std::fmt;

use crate::{OxidationMagnitude, OxidationState};

const ROMAN_NUMERALS: [&str; 8] = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII"];

/// A Roman numeral representation of a common oxidation-state magnitude.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RomanOxidationState {
    magnitude: OxidationMagnitude,
}

impl RomanOxidationState {
    /// Creates a Roman oxidation-state label from a nonzero magnitude.
    #[must_use]
    pub const fn new(magnitude: OxidationMagnitude) -> Option<Self> {
        if magnitude.is_zero() {
            None
        } else {
            Some(Self { magnitude })
        }
    }

    /// Creates a Roman oxidation-state label from an oxidation state.
    #[must_use]
    pub const fn from_state(state: OxidationState) -> Option<Self> {
        Self::new(state.magnitude())
    }

    /// Returns the magnitude represented by this Roman label.
    #[must_use]
    pub const fn magnitude(self) -> OxidationMagnitude {
        self.magnitude
    }

    /// Returns the Roman numeral text.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        roman_numeral(self.magnitude.get()).unwrap_or("")
    }
}

impl fmt::Display for RomanOxidationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Returns the Roman numeral for a supported oxidation-state magnitude.
#[must_use]
pub const fn roman_numeral(magnitude: u8) -> Option<&'static str> {
    match magnitude {
        1..=8 => Some(ROMAN_NUMERALS[(magnitude - 1) as usize]),
        _ => None,
    }
}
