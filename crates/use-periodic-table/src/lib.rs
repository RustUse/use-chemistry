#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Periodic-table lookup and conservative classification helpers.

pub use use_element::{all_elements, Element};

use use_element::element_by_atomic_number;

/// Returns all elements in a period.
///
/// # Examples
///
/// ```rust
/// use use_periodic_table::period_elements;
///
/// assert_eq!(period_elements(2).len(), 8);
/// assert!(period_elements(2).iter().any(|element| element.symbol == "C"));
/// ```
#[must_use]
pub fn period_elements(period: u8) -> Vec<Element> {
    all_elements()
        .iter()
        .copied()
        .filter(|element| element.period == period)
        .collect()
}

/// Returns all elements in a group.
///
/// # Examples
///
/// ```rust
/// use use_periodic_table::group_elements;
///
/// let noble_gases = group_elements(18);
///
/// assert!(noble_gases.iter().any(|element| element.symbol == "He"));
/// assert!(noble_gases.iter().any(|element| element.symbol == "Og"));
/// ```
#[must_use]
pub fn group_elements(group: u8) -> Vec<Element> {
    all_elements()
        .iter()
        .copied()
        .filter(|element| element.group == Some(group))
        .collect()
}

/// Returns the period for an atomic number.
#[must_use]
pub fn period_for_atomic_number(atomic_number: u8) -> Option<u8> {
    element_by_atomic_number(atomic_number).map(|element| element.period)
}

/// Returns the group for an atomic number.
#[must_use]
pub fn group_for_atomic_number(atomic_number: u8) -> Option<u8> {
    element_by_atomic_number(atomic_number).and_then(|element| element.group)
}

/// Returns `true` when the atomic number is in the supported range.
#[must_use]
pub const fn is_valid_atomic_number(atomic_number: u8) -> bool {
    matches!(atomic_number, 1..=118)
}

/// Returns `true` for the alkali metals.
#[must_use]
pub const fn is_alkali_metal(atomic_number: u8) -> bool {
    matches!(atomic_number, 3 | 11 | 19 | 37 | 55 | 87)
}

/// Returns `true` for the alkaline earth metals.
#[must_use]
pub const fn is_alkaline_earth_metal(atomic_number: u8) -> bool {
    matches!(atomic_number, 4 | 12 | 20 | 38 | 56 | 88)
}

/// Returns `true` for the halogens.
#[must_use]
pub const fn is_halogen(atomic_number: u8) -> bool {
    matches!(atomic_number, 9 | 17 | 35 | 53 | 85 | 117)
}

/// Returns `true` for the noble gases.
#[must_use]
pub const fn is_noble_gas(atomic_number: u8) -> bool {
    matches!(atomic_number, 2 | 10 | 18 | 36 | 54 | 86 | 118)
}

/// Returns `true` for the lanthanides.
#[must_use]
pub const fn is_lanthanide(atomic_number: u8) -> bool {
    matches!(atomic_number, 57..=71)
}

/// Returns `true` for the actinides.
#[must_use]
pub const fn is_actinide(atomic_number: u8) -> bool {
    matches!(atomic_number, 89..=103)
}

#[cfg(test)]
mod tests {
    use super::{
        all_elements, group_elements, group_for_atomic_number, is_actinide, is_alkali_metal,
        is_alkaline_earth_metal, is_halogen, is_lanthanide, is_noble_gas, is_valid_atomic_number,
        period_elements, period_for_atomic_number,
    };

    #[test]
    fn exposes_expected_period_and_group_filters() {
        assert_eq!(period_elements(2).len(), 8);
        assert!(period_elements(2)
            .iter()
            .any(|element| element.symbol == "C"));
        assert!(period_elements(2)
            .iter()
            .any(|element| element.symbol == "O"));

        assert_eq!(group_elements(18).len(), 7);
        assert!(group_elements(18)
            .iter()
            .any(|element| element.symbol == "He"));
        assert!(group_elements(18)
            .iter()
            .any(|element| element.symbol == "Ne"));
        assert!(group_elements(18)
            .iter()
            .any(|element| element.symbol == "Og"));

        assert!(group_elements(0).is_empty());
        assert!(period_elements(0).is_empty());
        assert_eq!(all_elements().len(), 118);
    }

    #[test]
    fn exposes_period_and_group_for_common_elements() {
        assert_eq!(period_for_atomic_number(1), Some(1));
        assert_eq!(period_for_atomic_number(6), Some(2));
        assert_eq!(period_for_atomic_number(11), Some(3));
        assert_eq!(period_for_atomic_number(26), Some(4));
        assert_eq!(period_for_atomic_number(79), Some(6));
        assert_eq!(period_for_atomic_number(92), Some(7));
        assert_eq!(period_for_atomic_number(118), Some(7));
        assert_eq!(period_for_atomic_number(0), None);
        assert_eq!(period_for_atomic_number(119), None);

        assert_eq!(group_for_atomic_number(1), Some(1));
        assert_eq!(group_for_atomic_number(6), Some(14));
        assert_eq!(group_for_atomic_number(11), Some(1));
        assert_eq!(group_for_atomic_number(26), Some(8));
        assert_eq!(group_for_atomic_number(79), Some(11));
        assert_eq!(group_for_atomic_number(92), None);
        assert_eq!(group_for_atomic_number(118), Some(18));
    }

    #[test]
    fn classifies_conservative_families() {
        assert!(is_valid_atomic_number(118));
        assert!(!is_valid_atomic_number(0));

        assert!(is_alkali_metal(11));
        assert!(!is_alkali_metal(1));
        assert!(is_alkaline_earth_metal(20));
        assert!(is_halogen(9));
        assert!(is_noble_gas(10));
        assert!(is_lanthanide(57));
        assert!(is_lanthanide(71));
        assert!(is_actinide(89));
        assert!(is_actinide(103));
        assert!(!is_halogen(10));
    }
}
