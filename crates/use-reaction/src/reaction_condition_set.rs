use std::fmt;

use crate::{ReactionCondition, ReactionValidationError};

/// An ordered collection of reaction conditions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReactionConditionSet {
    conditions: Vec<ReactionCondition>,
}

impl ReactionConditionSet {
    /// Creates an empty condition set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Adds a condition and returns the updated set.
    #[must_use]
    pub fn with_condition<T>(mut self, condition: T) -> Self
    where
        T: Into<ReactionCondition>,
    {
        self.push(condition);
        self
    }

    /// Adds a condition to the set.
    pub fn push<T>(&mut self, condition: T)
    where
        T: Into<ReactionCondition>,
    {
        self.conditions.push(condition.into());
    }

    /// Returns the conditions as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[ReactionCondition] {
        &self.conditions
    }

    /// Iterates over the conditions in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &ReactionCondition> {
        self.conditions.iter()
    }

    /// Returns the number of conditions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.conditions.len()
    }

    /// Returns `true` when the condition set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Validates all conditions in this set.
    ///
    /// # Errors
    ///
    /// Returns a [`ReactionValidationError`] when any condition contains an empty label or value.
    pub fn validate(&self) -> Result<(), ReactionValidationError> {
        for condition in &self.conditions {
            condition.validate()?;
        }

        Ok(())
    }
}

impl fmt::Display for ReactionConditionSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, condition) in self.conditions.iter().enumerate() {
            if index > 0 {
                formatter.write_str(", ")?;
            }
            write!(formatter, "{condition}")?;
        }

        Ok(())
    }
}
