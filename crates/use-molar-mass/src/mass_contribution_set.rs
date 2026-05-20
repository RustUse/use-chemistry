use std::fmt;

use crate::{ElementMassContribution, MolarMass, MolarMassValidationError};

/// Ordered per-element molar-mass contributions.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MassContributionSet {
    contributions: Vec<ElementMassContribution>,
}

impl MassContributionSet {
    /// Creates an empty contribution set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            contributions: Vec::new(),
        }
    }

    /// Creates a contribution set from validated contributions.
    #[must_use]
    pub fn from_contributions(
        contributions: impl IntoIterator<Item = ElementMassContribution>,
    ) -> Self {
        Self {
            contributions: contributions.into_iter().collect(),
        }
    }

    /// Appends a contribution.
    pub fn push(&mut self, contribution: ElementMassContribution) {
        self.contributions.push(contribution);
    }

    /// Returns the contributions as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[ElementMassContribution] {
        &self.contributions
    }

    /// Iterates over contributions in stored order.
    pub fn iter(&self) -> impl Iterator<Item = &ElementMassContribution> {
        self.contributions.iter()
    }

    /// Returns the number of contributions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contributions.len()
    }

    /// Returns true when the contribution set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contributions.is_empty()
    }

    /// Returns the total molar-mass value in grams per mole.
    #[must_use]
    pub fn total_mass_value(&self) -> f64 {
        self.contributions
            .iter()
            .map(ElementMassContribution::total_mass_value)
            .sum()
    }

    /// Returns the total as a molar mass value in grams per mole.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error if the total is not finite and positive.
    pub fn molar_mass(&self) -> Result<MolarMass, MolarMassValidationError> {
        MolarMass::grams_per_mole(self.total_mass_value())
    }
}

impl fmt::Display for MassContributionSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, contribution) in self.contributions.iter().enumerate() {
            if index > 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{contribution}")?;
        }

        Ok(())
    }
}
