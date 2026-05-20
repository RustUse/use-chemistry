use std::fmt;

use crate::{Product, Reactant, ReactionArrow, ReactionValidationError};

/// A chemical reaction equation with reactants, products, and an arrow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactionEquation {
    reactants: Vec<Reactant>,
    products: Vec<Product>,
    arrow: ReactionArrow,
}

impl ReactionEquation {
    /// Creates an empty reaction equation with a forward arrow.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            reactants: Vec::new(),
            products: Vec::new(),
            arrow: ReactionArrow::Forward,
        }
    }

    /// Adds a reactant and returns the updated equation.
    #[must_use]
    pub fn with_reactant<T>(mut self, reactant: T) -> Self
    where
        T: Into<Reactant>,
    {
        self.reactants.push(reactant.into());
        self
    }

    /// Adds a product and returns the updated equation.
    #[must_use]
    pub fn with_product<T>(mut self, product: T) -> Self
    where
        T: Into<Product>,
    {
        self.products.push(product.into());
        self
    }

    /// Sets the reaction arrow.
    #[must_use]
    pub const fn with_arrow(mut self, arrow: ReactionArrow) -> Self {
        self.arrow = arrow;
        self
    }

    /// Returns reactants in insertion order.
    #[must_use]
    pub fn reactants(&self) -> &[Reactant] {
        &self.reactants
    }

    /// Returns products in insertion order.
    #[must_use]
    pub fn products(&self) -> &[Product] {
        &self.products
    }

    /// Returns the reaction arrow.
    #[must_use]
    pub const fn arrow(&self) -> ReactionArrow {
        self.arrow
    }

    /// Validates that the equation has at least one reactant and one product.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyReaction`] when no reaction-side terms are present,
    /// [`ReactionValidationError::MissingReactants`] when no reactants are present, or
    /// [`ReactionValidationError::MissingProducts`] when no products are present.
    pub fn validate(&self) -> Result<(), ReactionValidationError> {
        match (self.reactants.is_empty(), self.products.is_empty()) {
            (true, true) => Err(ReactionValidationError::EmptyReaction),
            (true, false) => Err(ReactionValidationError::MissingReactants),
            (false, true) => Err(ReactionValidationError::MissingProducts),
            (false, false) => Ok(()),
        }
    }
}

impl Default for ReactionEquation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReactionEquation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_side(formatter, &self.reactants)?;
        write!(formatter, " {} ", self.arrow)?;
        write_side(formatter, &self.products)
    }
}

fn write_side<T>(formatter: &mut fmt::Formatter<'_>, terms: &[T]) -> fmt::Result
where
    T: fmt::Display,
{
    for (index, term) in terms.iter().enumerate() {
        if index > 0 {
            formatter.write_str(" + ")?;
        }
        write!(formatter, "{term}")?;
    }

    Ok(())
}
