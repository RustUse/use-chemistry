use std::fmt;
use std::slice;

use crate::OxidationStateAssignment;

/// A small insertion-order collection of oxidation-state assignments.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OxidationStateSet {
    assignments: Vec<OxidationStateAssignment>,
}

impl OxidationStateSet {
    /// Creates an empty oxidation-state set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            assignments: Vec::new(),
        }
    }

    /// Creates a set from assignments, replacing earlier entries with the same label.
    #[must_use]
    pub fn from_assignments(
        assignments: impl IntoIterator<Item = OxidationStateAssignment>,
    ) -> Self {
        let mut set = Self::new();

        for assignment in assignments {
            set.insert(assignment);
        }

        set
    }

    /// Inserts an assignment and returns the previous assignment with the same label, if any.
    pub fn insert(
        &mut self,
        assignment: OxidationStateAssignment,
    ) -> Option<OxidationStateAssignment> {
        if let Some(existing) = self
            .assignments
            .iter_mut()
            .find(|existing| existing.label() == assignment.label())
        {
            return Some(std::mem::replace(existing, assignment));
        }

        self.assignments.push(assignment);
        None
    }

    /// Returns an assignment by label.
    #[must_use]
    pub fn get(&self, label: &str) -> Option<&OxidationStateAssignment> {
        let label = label.trim();
        self.assignments
            .iter()
            .find(|assignment| assignment.label() == label)
    }

    /// Returns `true` when an assignment for `label` exists.
    #[must_use]
    pub fn contains_label(&self, label: &str) -> bool {
        self.get(label).is_some()
    }

    /// Returns the number of assignments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    /// Returns `true` when there are no assignments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    /// Returns an iterator over assignments.
    pub fn iter(&self) -> slice::Iter<'_, OxidationStateAssignment> {
        self.assignments.iter()
    }
}

impl fmt::Display for OxidationStateSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut assignments = self.assignments.iter();

        let Some(first) = assignments.next() else {
            return Ok(());
        };

        write!(formatter, "{first}")?;

        for assignment in assignments {
            write!(formatter, ", {assignment}")?;
        }

        Ok(())
    }
}

impl IntoIterator for OxidationStateSet {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = OxidationStateAssignment;

    fn into_iter(self) -> Self::IntoIter {
        self.assignments.into_iter()
    }
}

impl<'a> IntoIterator for &'a OxidationStateSet {
    type IntoIter = slice::Iter<'a, OxidationStateAssignment>;
    type Item = &'a OxidationStateAssignment;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
