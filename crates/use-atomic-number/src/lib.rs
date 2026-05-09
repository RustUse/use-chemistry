#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Atomic-number validation and lookup helpers.

use use_element::{all_elements, element_by_symbol};

/// Returns `true` when the atomic number is between 1 and 118 inclusive.
///
/// # Examples
///
/// ```rust
/// use use_atomic_number::is_valid_atomic_number;
///
/// assert!(is_valid_atomic_number(118));
/// assert!(!is_valid_atomic_number(0));
/// ```
#[must_use]
pub const fn is_valid_atomic_number(value: u8) -> bool {
    matches!(value, 1..=118)
}

/// Looks up an atomic number from a symbol.
#[must_use]
pub fn atomic_number_from_symbol(symbol: &str) -> Option<u8> {
    element_by_symbol(symbol).map(|element| element.atomic_number)
}

/// Looks up an atomic number from an element name using ASCII case-insensitive matching.
#[must_use]
pub fn atomic_number_from_name(name: &str) -> Option<u8> {
    let normalized = name.trim();

    if normalized.is_empty() {
        return None;
    }

    all_elements()
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(normalized))
        .map(|element| element.atomic_number)
}

/// Returns the proton count for a valid atomic number.
#[must_use]
pub fn proton_count(atomic_number: u8) -> Option<u8> {
    is_valid_atomic_number(atomic_number).then_some(atomic_number)
}

/// Returns the electron count for a neutral atom with the given atomic number.
#[must_use]
pub fn electron_count_neutral_atom(atomic_number: u8) -> Option<u8> {
    proton_count(atomic_number)
}

#[cfg(test)]
mod tests {
    use super::{
        atomic_number_from_name, atomic_number_from_symbol, electron_count_neutral_atom,
        is_valid_atomic_number, proton_count,
    };

    #[test]
    fn validates_range() {
        assert!(is_valid_atomic_number(1));
        assert!(is_valid_atomic_number(118));
        assert!(!is_valid_atomic_number(0));
        assert!(!is_valid_atomic_number(119));
    }

    #[test]
    fn looks_up_symbols_and_names() {
        assert_eq!(atomic_number_from_symbol("H"), Some(1));
        assert_eq!(atomic_number_from_symbol("C"), Some(6));
        assert_eq!(atomic_number_from_symbol("O"), Some(8));
        assert_eq!(atomic_number_from_symbol("Na"), Some(11));
        assert_eq!(atomic_number_from_symbol("Fe"), Some(26));
        assert_eq!(atomic_number_from_symbol("Au"), Some(79));
        assert_eq!(atomic_number_from_symbol("bad"), None);

        assert_eq!(atomic_number_from_name("Hydrogen"), Some(1));
        assert_eq!(atomic_number_from_name("carbon"), Some(6));
        assert_eq!(atomic_number_from_name(" oxygen "), Some(8));
        assert_eq!(atomic_number_from_name("Sodium"), Some(11));
        assert_eq!(atomic_number_from_name("Iron"), Some(26));
        assert_eq!(atomic_number_from_name("Gold"), Some(79));
        assert_eq!(atomic_number_from_name("Uranium"), Some(92));
        assert_eq!(atomic_number_from_name("Oganesson"), Some(118));
        assert_eq!(atomic_number_from_name("invalid"), None);
    }

    #[test]
    fn exposes_neutral_atom_counts() {
        assert_eq!(proton_count(6), Some(6));
        assert_eq!(electron_count_neutral_atom(6), Some(6));
        assert_eq!(proton_count(79), Some(79));
        assert_eq!(electron_count_neutral_atom(79), Some(79));
        assert_eq!(proton_count(0), None);
        assert_eq!(electron_count_neutral_atom(119), None);
    }
}
