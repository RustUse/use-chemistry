#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Chemistry-facing isotope identity and notation helpers.

pub mod prelude;

use use_element::{Element, element_by_atomic_number, element_by_symbol};

/// Small validated isotope identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Isotope {
    atomic_number: u8,
    mass_number: u16,
}

impl Isotope {
    /// Creates an isotope identity from an atomic number and mass number.
    ///
    /// Validation is structural: the atomic number must be in the element range, and the
    /// mass number must be at least the atomic number. This does not imply that the isotope
    /// is naturally occurring, stable, or experimentally known.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_isotope::Isotope;
    ///
    /// let carbon_12 = Isotope::new(6, 12).unwrap();
    ///
    /// assert_eq!(carbon_12.atomic_number(), 6);
    /// assert_eq!(carbon_12.mass_number(), 12);
    /// ```
    #[must_use]
    pub const fn new(atomic_number: u8, mass_number: u16) -> Option<Self> {
        if is_valid_isotope_numbers(atomic_number, mass_number) {
            Some(Self {
                atomic_number,
                mass_number,
            })
        } else {
            None
        }
    }

    /// Creates an isotope identity from an element symbol and mass number.
    #[must_use]
    pub fn from_symbol(symbol: &str, mass_number: u16) -> Option<Self> {
        isotope_by_symbol(symbol, mass_number)
    }

    /// Returns the isotope atomic number.
    #[must_use]
    pub const fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    /// Returns the isotope mass number.
    #[must_use]
    pub const fn mass_number(&self) -> u16 {
        self.mass_number
    }

    /// Returns the proton count.
    #[must_use]
    pub const fn proton_count(&self) -> u8 {
        self.atomic_number
    }

    /// Returns the neutron count.
    #[must_use]
    pub const fn neutron_count(&self) -> u16 {
        self.mass_number - self.atomic_number as u16
    }

    /// Returns the nucleon count.
    #[must_use]
    pub const fn nucleon_count(&self) -> u16 {
        self.mass_number
    }

    /// Looks up the isotope element metadata.
    #[must_use]
    pub fn element(&self) -> Option<Element> {
        element_by_atomic_number(self.atomic_number)
    }

    /// Looks up the isotope element symbol.
    #[must_use]
    pub fn element_symbol(&self) -> Option<&'static str> {
        self.element().map(|element| element.symbol)
    }

    /// Looks up the isotope element name.
    #[must_use]
    pub fn element_name(&self) -> Option<&'static str> {
        self.element().map(|element| element.name)
    }

    /// Formats the isotope with ASCII hyphen notation, such as `C-12`.
    #[must_use]
    pub fn hyphen_notation(&self) -> Option<String> {
        self.element_symbol()
            .map(|symbol| format!("{symbol}-{}", self.mass_number))
    }
}

/// Creates an isotope identity from an atomic number and mass number.
#[must_use]
pub const fn isotope(atomic_number: u8, mass_number: u16) -> Option<Isotope> {
    Isotope::new(atomic_number, mass_number)
}

/// Creates an isotope identity from an element symbol and mass number.
///
/// Symbol lookup trims surrounding whitespace and compares symbols with ASCII
/// case-insensitive matching through `use-element`.
#[must_use]
pub fn isotope_by_symbol(symbol: &str, mass_number: u16) -> Option<Isotope> {
    element_by_symbol(symbol).and_then(|element| Isotope::new(element.atomic_number, mass_number))
}

/// Returns `true` when the isotope numbers are structurally valid.
///
/// This only checks that `atomic_number` is between 1 and 118 inclusive and that
/// `mass_number >= atomic_number`. It does not imply that the isotope is known,
/// stable, naturally occurring, or abundant.
#[must_use]
pub const fn is_valid_isotope_numbers(atomic_number: u8, mass_number: u16) -> bool {
    matches!(atomic_number, 1..=118) && mass_number >= atomic_number as u16
}

/// Returns the proton count for structurally valid isotope numbers.
#[must_use]
pub const fn isotope_proton_count(atomic_number: u8, mass_number: u16) -> Option<u8> {
    if is_valid_isotope_numbers(atomic_number, mass_number) {
        Some(atomic_number)
    } else {
        None
    }
}

/// Returns the neutron count for structurally valid isotope numbers.
#[must_use]
pub const fn isotope_neutron_count(atomic_number: u8, mass_number: u16) -> Option<u16> {
    if is_valid_isotope_numbers(atomic_number, mass_number) {
        Some(mass_number - atomic_number as u16)
    } else {
        None
    }
}

/// Returns the nucleon count for structurally valid isotope numbers.
#[must_use]
pub const fn isotope_nucleon_count(atomic_number: u8, mass_number: u16) -> Option<u16> {
    if is_valid_isotope_numbers(atomic_number, mass_number) {
        Some(mass_number)
    } else {
        None
    }
}

/// Formats structurally valid isotope numbers as ASCII hyphen notation.
#[must_use]
pub fn hyphen_notation(atomic_number: u8, mass_number: u16) -> Option<String> {
    Isotope::new(atomic_number, mass_number).and_then(|isotope| isotope.hyphen_notation())
}

/// Named alias for ASCII isotope notation such as `C-12`.
#[must_use]
pub fn isotope_symbol(atomic_number: u8, mass_number: u16) -> Option<String> {
    hyphen_notation(atomic_number, mass_number)
}

#[cfg(test)]
mod tests {
    use super::{
        Isotope, hyphen_notation, is_valid_isotope_numbers, isotope, isotope_by_symbol,
        isotope_neutron_count, isotope_nucleon_count, isotope_proton_count, isotope_symbol,
    };

    #[test]
    fn validates_structural_isotope_numbers() {
        assert!(is_valid_isotope_numbers(1, 1));
        assert!(is_valid_isotope_numbers(6, 12));
        assert!(is_valid_isotope_numbers(6, 14));
        assert!(is_valid_isotope_numbers(8, 16));
        assert!(is_valid_isotope_numbers(92, 235));
        assert!(is_valid_isotope_numbers(92, 238));

        assert!(!is_valid_isotope_numbers(0, 1));
        assert!(!is_valid_isotope_numbers(119, 294));
        assert!(!is_valid_isotope_numbers(1, 0));
        assert!(!is_valid_isotope_numbers(6, 5));
    }

    #[test]
    fn constructs_isotopes_and_exposes_counts() {
        let Some(carbon_12) = Isotope::new(6, 12) else {
            panic!("expected carbon-12 isotope");
        };

        assert_eq!(carbon_12.atomic_number(), 6);
        assert_eq!(carbon_12.mass_number(), 12);
        assert_eq!(carbon_12.proton_count(), 6);
        assert_eq!(carbon_12.neutron_count(), 6);
        assert_eq!(carbon_12.nucleon_count(), 12);
        assert_eq!(carbon_12.element_symbol(), Some("C"));
        assert_eq!(carbon_12.element_name(), Some("Carbon"));
        assert_eq!(carbon_12.hyphen_notation(), Some(String::from("C-12")));
        assert_eq!(
            isotope(92, 235).map(|value| value.neutron_count()),
            Some(143)
        );
        assert_eq!(Isotope::new(2, 1), None);
    }

    #[test]
    fn resolves_symbols_case_insensitively() {
        let Some(carbon_14) = isotope_by_symbol(" c ", 14) else {
            panic!("expected carbon-14 isotope");
        };

        assert_eq!(carbon_14.atomic_number(), 6);
        assert_eq!(carbon_14.mass_number(), 14);
        assert_eq!(carbon_14.neutron_count(), 8);
        assert_eq!(carbon_14.hyphen_notation(), Some(String::from("C-14")));

        assert_eq!(
            Isotope::from_symbol("U", 238).map(|value| value.neutron_count()),
            Some(146)
        );
        assert_eq!(isotope_by_symbol("bad", 12), None);
        assert_eq!(isotope_by_symbol("", 12), None);
        assert_eq!(isotope_by_symbol("H", 0), None);
    }

    #[test]
    fn helper_functions_validate_inputs() {
        assert_eq!(isotope_proton_count(6, 12), Some(6));
        assert_eq!(isotope_neutron_count(6, 12), Some(6));
        assert_eq!(isotope_neutron_count(6, 14), Some(8));
        assert_eq!(isotope_nucleon_count(8, 16), Some(16));
        assert_eq!(hyphen_notation(8, 16), Some(String::from("O-16")));
        assert_eq!(isotope_symbol(92, 235), Some(String::from("U-235")));

        assert_eq!(isotope_proton_count(0, 1), None);
        assert_eq!(isotope_neutron_count(119, 294), None);
        assert_eq!(isotope_nucleon_count(6, 5), None);
        assert_eq!(hyphen_notation(0, 1), None);
        assert_eq!(isotope_symbol(119, 294), None);
    }
}
