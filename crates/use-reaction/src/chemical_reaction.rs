use std::fmt;

use crate::{
    Product, Reactant, ReactionArrow, ReactionCondition, ReactionConditionSet, ReactionEquation,
    ReactionKind, ReactionValidationError,
};

/// A chemical reaction with an equation, optional conditions, and classification labels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChemicalReaction {
    equation: ReactionEquation,
    conditions: ReactionConditionSet,
    kinds: Vec<ReactionKind>,
}

impl ChemicalReaction {
    /// Creates an empty reaction builder/value.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            equation: ReactionEquation::new(),
            conditions: ReactionConditionSet::new(),
            kinds: Vec::new(),
        }
    }

    /// Adds a reactant and returns the updated reaction.
    #[must_use]
    pub fn with_reactant<T>(mut self, reactant: T) -> Self
    where
        T: Into<Reactant>,
    {
        self.equation = self.equation.with_reactant(reactant);
        self
    }

    /// Adds a product and returns the updated reaction.
    #[must_use]
    pub fn with_product<T>(mut self, product: T) -> Self
    where
        T: Into<Product>,
    {
        self.equation = self.equation.with_product(product);
        self
    }

    /// Sets the reaction arrow.
    #[must_use]
    pub fn with_arrow(mut self, arrow: ReactionArrow) -> Self {
        self.equation = self.equation.with_arrow(arrow);
        self
    }

    /// Adds a condition and returns the updated reaction.
    #[must_use]
    pub fn with_condition<T>(mut self, condition: T) -> Self
    where
        T: Into<ReactionCondition>,
    {
        self.conditions.push(condition);
        self
    }

    /// Adds a kind label if it is not already present.
    #[must_use]
    pub fn with_kind(mut self, kind: ReactionKind) -> Self {
        if !self.kinds.contains(&kind) {
            self.kinds.push(kind);
        }
        self
    }

    /// Returns the reaction equation.
    #[must_use]
    pub const fn equation(&self) -> &ReactionEquation {
        &self.equation
    }

    /// Returns reactants in insertion order.
    #[must_use]
    pub fn reactants(&self) -> &[Reactant] {
        self.equation.reactants()
    }

    /// Returns products in insertion order.
    #[must_use]
    pub fn products(&self) -> &[Product] {
        self.equation.products()
    }

    /// Returns the reaction arrow.
    #[must_use]
    pub const fn arrow(&self) -> ReactionArrow {
        self.equation.arrow()
    }

    /// Returns the reaction conditions.
    #[must_use]
    pub const fn conditions(&self) -> &ReactionConditionSet {
        &self.conditions
    }

    /// Returns reaction kind labels in insertion order.
    #[must_use]
    pub fn kinds(&self) -> &[ReactionKind] {
        &self.kinds
    }

    /// Validates the equation and condition descriptors.
    ///
    /// # Errors
    ///
    /// Returns a [`ReactionValidationError`] when the equation is incomplete or a condition label
    /// is empty.
    pub fn validate(&self) -> Result<(), ReactionValidationError> {
        self.equation.validate()?;
        self.conditions.validate()
    }
}

impl Default for ChemicalReaction {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ChemicalReaction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.equation)
    }
}
