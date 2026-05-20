use std::fmt;

use crate::atomic_mass_entry::validate_atomic_mass;
use crate::atomic_mass_lookup::validate_symbol;
use crate::{MolarMass, MolarMassValidationError};

/// A per-element contribution to a formula molar mass.
#[derive(Clone, Debug, PartialEq)]
pub struct ElementMassContribution {
    symbol: String,
    atomic_mass: f64,
    count: u32,
}

impl ElementMassContribution {
    /// Creates an element mass contribution.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error when the symbol, atomic mass, count,
    /// or calculated contribution is invalid.
    pub fn new(
        symbol: &str,
        atomic_mass: f64,
        count: u32,
    ) -> Result<Self, MolarMassValidationError> {
        let symbol = validate_symbol(symbol)?;
        validate_atomic_mass(symbol, atomic_mass)?;

        if count == 0 {
            return Err(MolarMassValidationError::ZeroElementCount {
                symbol: symbol.to_owned(),
            });
        }

        let contribution = atomic_mass * f64::from(count);
        MolarMass::grams_per_mole(contribution)?;

        Ok(Self {
            symbol: symbol.to_owned(),
            atomic_mass,
            count,
        })
    }

    /// Returns the element symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the atomic mass in grams per mole.
    #[must_use]
    pub const fn atomic_mass(&self) -> f64 {
        self.atomic_mass
    }

    /// Returns the element count in the formula.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.count
    }

    /// Returns the contribution value in grams per mole.
    #[must_use]
    pub fn total_mass_value(&self) -> f64 {
        self.atomic_mass * f64::from(self.count)
    }

    /// Returns the contribution as a molar mass value in grams per mole.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error if the calculated contribution is invalid.
    pub fn total_molar_mass(&self) -> Result<MolarMass, MolarMassValidationError> {
        MolarMass::grams_per_mole(self.total_mass_value())
    }
}

impl fmt::Display for ElementMassContribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} × {} = {} g/mol",
            self.symbol,
            self.count,
            self.atomic_mass,
            self.total_mass_value()
        )
    }
}
