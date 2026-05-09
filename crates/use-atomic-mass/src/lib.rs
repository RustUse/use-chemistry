#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Average atomic mass helpers backed by static element data.

use use_element::{element_by_atomic_number, element_by_symbol};

/// Looks up the average atomic mass for an element symbol.
///
/// # Examples
///
/// ```rust
/// use use_atomic_mass::atomic_mass_by_symbol;
///
/// assert!((atomic_mass_by_symbol("C").unwrap() - 12.011).abs() < 0.01);
/// assert!(atomic_mass_by_symbol("??").is_none());
/// ```
#[must_use]
pub fn atomic_mass_by_symbol(symbol: &str) -> Option<f64> {
    element_by_symbol(symbol).map(|element| element.atomic_mass)
}

/// Looks up the average atomic mass for an atomic number.
#[must_use]
pub fn atomic_mass_by_atomic_number(atomic_number: u8) -> Option<f64> {
    element_by_atomic_number(atomic_number).map(|element| element.atomic_mass)
}

/// Named alias for average atomic mass lookup by symbol.
#[must_use]
pub fn average_atomic_mass(symbol: &str) -> Option<f64> {
    atomic_mass_by_symbol(symbol)
}

/// Returns the elemental molar mass in grams per mole.
#[must_use]
pub fn molar_mass_element(symbol: &str) -> Option<f64> {
    atomic_mass_by_symbol(symbol)
}

#[cfg(test)]
mod tests {
    use super::{
        atomic_mass_by_atomic_number, atomic_mass_by_symbol, average_atomic_mass,
        molar_mass_element,
    };

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 0.02, "left={left}, right={right}");
    }

    #[test]
    fn looks_up_expected_atomic_masses_by_symbol() {
        assert_close(atomic_mass_by_symbol("H").unwrap_or_default(), 1.008);
        assert_close(atomic_mass_by_symbol("C").unwrap_or_default(), 12.011);
        assert_close(atomic_mass_by_symbol("O").unwrap_or_default(), 15.999);
        assert_close(atomic_mass_by_symbol("Na").unwrap_or_default(), 22.990);
        assert_close(atomic_mass_by_symbol("Fe").unwrap_or_default(), 55.845);
        assert_close(atomic_mass_by_symbol("Au").unwrap_or_default(), 196.97);
        assert_close(atomic_mass_by_symbol("U").unwrap_or_default(), 238.03);
        assert_close(atomic_mass_by_symbol("Og").unwrap_or_default(), 294.0);
        assert_eq!(atomic_mass_by_symbol("invalid"), None);
    }

    #[test]
    fn looks_up_expected_atomic_masses_by_atomic_number() {
        assert_close(atomic_mass_by_atomic_number(1).unwrap_or_default(), 1.008);
        assert_close(atomic_mass_by_atomic_number(6).unwrap_or_default(), 12.011);
        assert_close(atomic_mass_by_atomic_number(8).unwrap_or_default(), 15.999);
        assert_close(atomic_mass_by_atomic_number(11).unwrap_or_default(), 22.990);
        assert_close(atomic_mass_by_atomic_number(26).unwrap_or_default(), 55.845);
        assert_close(atomic_mass_by_atomic_number(79).unwrap_or_default(), 196.97);
        assert_close(atomic_mass_by_atomic_number(92).unwrap_or_default(), 238.03);
        assert_close(atomic_mass_by_atomic_number(118).unwrap_or_default(), 294.0);
        assert_eq!(atomic_mass_by_atomic_number(0), None);
        assert_eq!(atomic_mass_by_atomic_number(119), None);
    }

    #[test]
    fn exposes_average_and_molar_mass_aliases() {
        assert_close(average_atomic_mass("O").unwrap_or_default(), 15.999);
        assert_close(molar_mass_element("Na").unwrap_or_default(), 22.990);
        assert_eq!(average_atomic_mass("bad"), None);
        assert_eq!(molar_mass_element("bad"), None);
    }
}
