use std::fmt;

use crate::{
    BondDescriptor, BondEndpoint, BondKind, BondLength, BondOrder, BondParticipant, BondPolarity,
    BondStrength, BondValidationError,
};

/// A chemical bond identity with optional structural descriptors.
#[derive(Clone, Debug, PartialEq)]
pub struct Bond {
    kind: BondKind,
    order: Option<BondOrder>,
    endpoints: Vec<BondEndpoint>,
    participants: Vec<BondParticipant>,
    polarity: Option<BondPolarity>,
    strength: Option<BondStrength>,
    length: Option<BondLength>,
    angle_label: Option<BondDescriptor>,
    descriptors: Vec<BondDescriptor>,
}

impl Bond {
    /// Creates a bond with a kind label.
    #[must_use]
    pub fn new(kind: BondKind) -> Self {
        Self {
            kind,
            order: None,
            endpoints: Vec::new(),
            participants: Vec::new(),
            polarity: None,
            strength: None,
            length: None,
            angle_label: None,
            descriptors: Vec::new(),
        }
    }

    /// Creates a bond between two validated endpoint references.
    #[must_use]
    pub fn between(endpoint_a: BondEndpoint, endpoint_b: BondEndpoint, kind: BondKind) -> Self {
        Self::new(kind)
            .with_endpoint(endpoint_a)
            .with_endpoint(endpoint_b)
    }

    /// Returns the bond kind.
    #[must_use]
    pub const fn kind(&self) -> BondKind {
        self.kind
    }

    /// Returns the optional bond order.
    #[must_use]
    pub const fn order(&self) -> Option<BondOrder> {
        self.order
    }

    /// Returns endpoint references in insertion order.
    #[must_use]
    pub fn endpoints(&self) -> &[BondEndpoint] {
        &self.endpoints
    }

    /// Returns participant references in insertion order.
    #[must_use]
    pub fn participants(&self) -> &[BondParticipant] {
        &self.participants
    }

    /// Returns the optional polarity label.
    #[must_use]
    pub const fn polarity(&self) -> Option<BondPolarity> {
        self.polarity
    }

    /// Returns the optional strength label.
    #[must_use]
    pub const fn strength(&self) -> Option<BondStrength> {
        self.strength
    }

    /// Returns the optional bond length.
    #[must_use]
    pub const fn length(&self) -> Option<&BondLength> {
        self.length.as_ref()
    }

    /// Returns the optional angle label or reference.
    #[must_use]
    pub const fn angle_label(&self) -> Option<&BondDescriptor> {
        self.angle_label.as_ref()
    }

    /// Returns lightweight descriptors in insertion order.
    #[must_use]
    pub fn descriptors(&self) -> &[BondDescriptor] {
        &self.descriptors
    }

    /// Assigns a bond order.
    #[must_use]
    pub const fn with_order(mut self, order: BondOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Adds an endpoint reference.
    #[must_use]
    pub fn with_endpoint(mut self, endpoint: BondEndpoint) -> Self {
        self.endpoints.push(endpoint);
        self
    }

    /// Adds an endpoint reference after validation.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyEndpointLabel`] when `endpoint` is empty after trimming.
    pub fn try_with_endpoint(self, endpoint: &str) -> Result<Self, BondValidationError> {
        Ok(self.with_endpoint(BondEndpoint::new(endpoint)?))
    }

    /// Adds a participant reference if it is not already present.
    #[must_use]
    pub fn with_participant(mut self, participant: BondParticipant) -> Self {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
        self
    }

    /// Adds a participant reference after validation.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyParticipantLabel`] when `participant` is empty after
    /// trimming.
    pub fn try_with_participant(self, participant: &str) -> Result<Self, BondValidationError> {
        Ok(self.with_participant(BondParticipant::new(participant)?))
    }

    /// Assigns a polarity label.
    #[must_use]
    pub const fn with_polarity(mut self, polarity: BondPolarity) -> Self {
        self.polarity = Some(polarity);
        self
    }

    /// Assigns a strength label.
    #[must_use]
    pub const fn with_strength(mut self, strength: BondStrength) -> Self {
        self.strength = Some(strength);
        self
    }

    /// Assigns a bond length.
    #[must_use]
    pub fn with_length(mut self, length: BondLength) -> Self {
        self.length = Some(length);
        self
    }

    /// Assigns a bond length after validation.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError`] when the length value is not finite, is not positive, or the
    /// unit is empty after trimming.
    pub fn try_with_length(self, value: f64, unit: &str) -> Result<Self, BondValidationError> {
        Ok(self.with_length(BondLength::new(value, unit)?))
    }

    /// Assigns an angle label or reference.
    #[must_use]
    pub fn with_angle_label(mut self, angle_label: BondDescriptor) -> Self {
        self.angle_label = Some(angle_label);
        self
    }

    /// Assigns an angle label or reference after validation.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyAngleLabel`] when `angle_label` is empty after trimming.
    pub fn try_with_angle_label(self, angle_label: &str) -> Result<Self, BondValidationError> {
        let trimmed = angle_label.trim();
        if trimmed.is_empty() {
            return Err(BondValidationError::EmptyAngleLabel);
        }

        Ok(self.with_angle_label(BondDescriptor::new(trimmed)?))
    }

    /// Adds a descriptor if it is not already present.
    #[must_use]
    pub fn with_descriptor(mut self, descriptor: BondDescriptor) -> Self {
        if !self.descriptors.contains(&descriptor) {
            self.descriptors.push(descriptor);
        }
        self
    }

    /// Adds a descriptor after validation.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyDescriptor`] when `descriptor` is empty after trimming.
    pub fn try_with_descriptor(self, descriptor: &str) -> Result<Self, BondValidationError> {
        Ok(self.with_descriptor(BondDescriptor::new(descriptor)?))
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((first, rest)) = self.endpoints.split_first() {
            write!(formatter, "{first}")?;
            for endpoint in rest {
                write!(formatter, "-{endpoint}")?;
            }
            write!(formatter, " {} bond", self.kind)?;
        } else {
            write!(formatter, "{} bond", self.kind)?;
        }

        if let Some(order) = self.order {
            write!(formatter, " ({order})")?;
        }

        Ok(())
    }
}
