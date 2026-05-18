#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Simple electron shell distribution helpers.

mod data;

use data::SHELLS;
use use_element::element_by_atomic_number;

/// Returns the static shell distribution for an atomic number.
///
/// # Examples
///
/// ```rust
/// use use_electron_shell::electron_shells;
///
/// assert_eq!(electron_shells(1), Some(vec![1]));
/// assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
/// ```
#[must_use]
pub fn electron_shells(atomic_number: u8) -> Option<Vec<u8>> {
    atomic_number
        .checked_sub(1)
        .and_then(|index| SHELLS.get(usize::from(index)))
        .map(|shells| shells.to_vec())
}

/// Returns the number of occupied shells in the static distribution.
#[must_use]
pub fn shell_count(atomic_number: u8) -> Option<usize> {
    electron_shells(atomic_number).map(|shells| shells.len())
}

/// Returns the electron count in the outermost occupied shell.
#[must_use]
pub fn outer_shell_electrons(atomic_number: u8) -> Option<u8> {
    electron_shells(atomic_number).and_then(|shells| shells.last().copied())
}

/// Returns a conservative main-group valence electron count.
///
/// Transition metals, lanthanides, and actinides return `None` because their
/// practical valence behavior is not well represented by a single introductory value.
#[must_use]
pub fn valence_electrons_main_group(atomic_number: u8) -> Option<u8> {
    element_by_atomic_number(atomic_number).and_then(|element| match element.group {
        Some(group @ 1..=2) => Some(group),
        Some(group @ 13..=18) => {
            if element.atomic_number == 2 {
                Some(2)
            } else {
                Some(group - 10)
            }
        },
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        electron_shells, outer_shell_electrons, shell_count, valence_electrons_main_group,
    };

    #[test]
    fn exposes_expected_shell_distributions() {
        assert_eq!(electron_shells(1), Some(vec![1]));
        assert_eq!(electron_shells(2), Some(vec![2]));
        assert_eq!(electron_shells(6), Some(vec![2, 4]));
        assert_eq!(electron_shells(10), Some(vec![2, 8]));
        assert_eq!(electron_shells(11), Some(vec![2, 8, 1]));
        assert_eq!(electron_shells(20), Some(vec![2, 8, 8, 2]));
        assert_eq!(electron_shells(0), None);
        assert_eq!(electron_shells(119), None);
    }

    #[test]
    fn exposes_shell_counts_and_outer_shell_counts() {
        assert_eq!(shell_count(1), Some(1));
        assert_eq!(shell_count(11), Some(3));
        assert_eq!(shell_count(20), Some(4));
        assert_eq!(outer_shell_electrons(1), Some(1));
        assert_eq!(outer_shell_electrons(8), Some(6));
        assert_eq!(outer_shell_electrons(10), Some(8));
        assert_eq!(outer_shell_electrons(11), Some(1));
        assert_eq!(outer_shell_electrons(0), None);
    }

    #[test]
    fn exposes_conservative_main_group_valence() {
        assert_eq!(valence_electrons_main_group(1), Some(1));
        assert_eq!(valence_electrons_main_group(2), Some(2));
        assert_eq!(valence_electrons_main_group(6), Some(4));
        assert_eq!(valence_electrons_main_group(11), Some(1));
        assert_eq!(valence_electrons_main_group(17), Some(7));
        assert_eq!(valence_electrons_main_group(18), Some(8));
        assert_eq!(valence_electrons_main_group(26), None);
        assert_eq!(valence_electrons_main_group(57), None);
        assert_eq!(valence_electrons_main_group(92), None);
    }
}
