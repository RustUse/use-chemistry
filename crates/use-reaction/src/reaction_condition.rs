use std::fmt;

use crate::{Catalyst, ReactionValidationError, Solvent};

/// A lightweight reaction condition label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReactionCondition {
    /// Temperature descriptor.
    Temperature(String),
    /// Pressure descriptor.
    Pressure(String),
    /// Catalyst descriptor.
    Catalyst(Catalyst),
    /// Solvent descriptor.
    Solvent(Solvent),
    /// Heat label.
    Heat,
    /// Light label.
    Light,
    /// Electrolysis label.
    Electrolysis,
    /// Custom condition label and optional value.
    Custom {
        /// Custom condition label.
        label: String,
        /// Optional custom condition value.
        value: Option<String>,
    },
}

impl ReactionCondition {
    /// Creates a temperature condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyTemperatureLabel`] when `label` is empty after
    /// trimming.
    pub fn temperature(label: &str) -> Result<Self, ReactionValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(ReactionValidationError::EmptyTemperatureLabel)
        } else {
            Ok(Self::Temperature(trimmed.to_owned()))
        }
    }

    /// Creates a pressure condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyPressureLabel`] when `label` is empty after
    /// trimming.
    pub fn pressure(label: &str) -> Result<Self, ReactionValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(ReactionValidationError::EmptyPressureLabel)
        } else {
            Ok(Self::Pressure(trimmed.to_owned()))
        }
    }

    /// Creates a catalyst condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyCatalystLabel`] when `label` is empty after
    /// trimming.
    pub fn catalyst(label: &str) -> Result<Self, ReactionValidationError> {
        Ok(Self::Catalyst(Catalyst::new(label)?))
    }

    /// Creates a solvent condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptySolventLabel`] when `label` is empty after
    /// trimming.
    pub fn solvent(label: &str) -> Result<Self, ReactionValidationError> {
        Ok(Self::Solvent(Solvent::new(label)?))
    }

    /// Creates a custom condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyConditionLabel`] when `label` is empty after
    /// trimming, or [`ReactionValidationError::EmptyConditionValue`] when `value` is present but
    /// empty after trimming.
    pub fn custom(label: &str, value: Option<&str>) -> Result<Self, ReactionValidationError> {
        let label = label.trim();
        if label.is_empty() {
            return Err(ReactionValidationError::EmptyConditionLabel);
        }

        let value = value
            .map(str::trim)
            .map(|trimmed| {
                if trimmed.is_empty() {
                    Err(ReactionValidationError::EmptyConditionValue)
                } else {
                    Ok(trimmed.to_owned())
                }
            })
            .transpose()?;

        Ok(Self::Custom {
            label: label.to_owned(),
            value,
        })
    }

    /// Validates this condition descriptor.
    ///
    /// # Errors
    ///
    /// Returns a [`ReactionValidationError`] when a directly constructed condition contains an
    /// empty label or value.
    pub fn validate(&self) -> Result<(), ReactionValidationError> {
        match self {
            Self::Temperature(label) if label.trim().is_empty() => {
                Err(ReactionValidationError::EmptyTemperatureLabel)
            },
            Self::Pressure(label) if label.trim().is_empty() => {
                Err(ReactionValidationError::EmptyPressureLabel)
            },
            Self::Custom { label, .. } if label.trim().is_empty() => {
                Err(ReactionValidationError::EmptyConditionLabel)
            },
            Self::Custom {
                value: Some(value), ..
            } if value.trim().is_empty() => Err(ReactionValidationError::EmptyConditionValue),
            _ => Ok(()),
        }
    }
}

impl From<Catalyst> for ReactionCondition {
    fn from(catalyst: Catalyst) -> Self {
        Self::Catalyst(catalyst)
    }
}

impl From<Solvent> for ReactionCondition {
    fn from(solvent: Solvent) -> Self {
        Self::Solvent(solvent)
    }
}

impl fmt::Display for ReactionCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Temperature(label) => write!(formatter, "temperature: {label}"),
            Self::Pressure(label) => write!(formatter, "pressure: {label}"),
            Self::Catalyst(catalyst) => write!(formatter, "catalyst: {catalyst}"),
            Self::Solvent(solvent) => write!(formatter, "solvent: {solvent}"),
            Self::Heat => formatter.write_str("heat"),
            Self::Light => formatter.write_str("light"),
            Self::Electrolysis => formatter.write_str("electrolysis"),
            Self::Custom { label, value } => match value {
                Some(value) => write!(formatter, "{label}: {value}"),
                None => formatter.write_str(label),
            },
        }
    }
}
