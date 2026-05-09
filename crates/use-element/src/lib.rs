#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Basic chemical element primitives and static lookup helpers.

mod data;

use data::ELEMENTS;

/// Small copyable element metadata suitable for direct lookup helpers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Element {
    pub atomic_number: u8,
    pub symbol: &'static str,
    pub name: &'static str,
    pub atomic_mass: f64,
    pub period: u8,
    pub group: Option<u8>,
}

impl Element {
    /// Creates an element metadata value.
    #[must_use]
    pub const fn new(
        atomic_number: u8,
        symbol: &'static str,
        name: &'static str,
        atomic_mass: f64,
        period: u8,
        group: Option<u8>,
    ) -> Self {
        Self {
            atomic_number,
            symbol,
            name,
            atomic_mass,
            period,
            group,
        }
    }
}

/// Returns the full static element table in atomic-number order.
#[must_use]
pub fn all_elements() -> &'static [Element] {
    &ELEMENTS
}

/// Looks up an element by symbol using ASCII case-insensitive matching.
///
/// Leading and trailing whitespace are ignored.
///
/// # Examples
///
/// ```rust
/// use use_element::element_by_symbol;
///
/// let sodium = element_by_symbol(" na ").unwrap();
///
/// assert_eq!(sodium.atomic_number, 11);
/// assert_eq!(sodium.symbol, "Na");
/// ```
#[must_use]
pub fn element_by_symbol(symbol: &str) -> Option<Element> {
    let normalized = symbol.trim();

    if normalized.is_empty() {
        return None;
    }

    ELEMENTS
        .iter()
        .copied()
        .find(|element| element.symbol.eq_ignore_ascii_case(normalized))
}

/// Looks up an element by atomic number.
///
/// # Examples
///
/// ```rust
/// use use_element::element_by_atomic_number;
///
/// let oxygen = element_by_atomic_number(8).unwrap();
///
/// assert_eq!(oxygen.symbol, "O");
/// assert_eq!(oxygen.name, "Oxygen");
/// ```
#[must_use]
pub fn element_by_atomic_number(atomic_number: u8) -> Option<Element> {
    atomic_number
        .checked_sub(1)
        .and_then(|index| ELEMENTS.get(usize::from(index)))
        .copied()
}

/// Returns the element name for a symbol lookup.
///
/// # Examples
///
/// ```rust
/// use use_element::element_name;
///
/// assert_eq!(element_name("Au"), Some("Gold"));
/// assert_eq!(element_name("??"), None);
/// ```
#[must_use]
pub fn element_name(symbol: &str) -> Option<&'static str> {
    element_by_symbol(symbol).map(|element| element.name)
}

/// Returns the element symbol for an atomic number.
///
/// # Examples
///
/// ```rust
/// use use_element::element_symbol;
///
/// assert_eq!(element_symbol(79), Some("Au"));
/// assert_eq!(element_symbol(0), None);
/// ```
#[must_use]
pub fn element_symbol(atomic_number: u8) -> Option<&'static str> {
    element_by_atomic_number(atomic_number).map(|element| element.symbol)
}

#[cfg(test)]
mod tests {
    use super::{
        all_elements, element_by_atomic_number, element_by_symbol, element_name, element_symbol,
    };

    #[test]
    fn exposes_all_known_elements() {
        assert_eq!(all_elements().len(), 118);
        assert_eq!(
            all_elements().first().map(|element| element.symbol),
            Some("H")
        );
        assert_eq!(
            all_elements().last().map(|element| element.symbol),
            Some("Og")
        );
    }

    #[test]
    fn looks_up_expected_symbols_case_insensitively() {
        assert_eq!(
            element_by_symbol("H").map(|element| element.atomic_number),
            Some(1)
        );
        assert_eq!(
            element_by_symbol("c").map(|element| element.name),
            Some("Carbon")
        );
        assert_eq!(
            element_by_symbol("O").map(|element| element.atomic_number),
            Some(8)
        );
        assert_eq!(
            element_by_symbol(" na ").map(|element| element.atomic_number),
            Some(11)
        );
        assert_eq!(
            element_by_symbol("Fe").map(|element| element.name),
            Some("Iron")
        );
        assert_eq!(
            element_by_symbol("Au").map(|element| element.atomic_number),
            Some(79)
        );
        assert_eq!(
            element_by_symbol("U").map(|element| element.name),
            Some("Uranium")
        );
        assert_eq!(
            element_by_symbol("Og").map(|element| element.atomic_number),
            Some(118)
        );
        assert_eq!(element_by_symbol("Xx"), None);
        assert_eq!(element_by_symbol("   "), None);
    }

    #[test]
    fn looks_up_expected_atomic_numbers() {
        assert_eq!(
            element_by_atomic_number(1).map(|element| element.symbol),
            Some("H")
        );
        assert_eq!(
            element_by_atomic_number(6).map(|element| element.name),
            Some("Carbon")
        );
        assert_eq!(
            element_by_atomic_number(8).map(|element| element.symbol),
            Some("O")
        );
        assert_eq!(
            element_by_atomic_number(11).map(|element| element.symbol),
            Some("Na")
        );
        assert_eq!(
            element_by_atomic_number(26).map(|element| element.name),
            Some("Iron")
        );
        assert_eq!(
            element_by_atomic_number(79).map(|element| element.name),
            Some("Gold")
        );
        assert_eq!(
            element_by_atomic_number(92).map(|element| element.name),
            Some("Uranium")
        );
        assert_eq!(
            element_by_atomic_number(118).map(|element| element.name),
            Some("Oganesson")
        );
        assert_eq!(element_by_atomic_number(0), None);
        assert_eq!(element_by_atomic_number(119), None);
    }

    #[test]
    fn exposes_name_and_symbol_helpers() {
        assert_eq!(element_name("H"), Some("Hydrogen"));
        assert_eq!(element_name("na"), Some("Sodium"));
        assert_eq!(element_name("invalid"), None);
        assert_eq!(element_symbol(8), Some("O"));
        assert_eq!(element_symbol(79), Some("Au"));
        assert_eq!(element_symbol(119), None);
    }

    #[test]
    fn stores_expected_period_and_group_metadata() {
        let hydrogen = element_by_symbol("H").expect("hydrogen should exist");
        let carbon = element_by_symbol("C").expect("carbon should exist");
        let iron = element_by_symbol("Fe").expect("iron should exist");
        let uranium = element_by_symbol("U").expect("uranium should exist");
        let oganesson = element_by_symbol("Og").expect("oganesson should exist");

        assert_eq!((hydrogen.period, hydrogen.group), (1, Some(1)));
        assert_eq!((carbon.period, carbon.group), (2, Some(14)));
        assert_eq!((iron.period, iron.group), (4, Some(8)));
        assert_eq!((uranium.period, uranium.group), (7, None));
        assert_eq!((oganesson.period, oganesson.group), (7, Some(18)));
    }
}
