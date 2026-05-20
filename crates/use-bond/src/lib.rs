#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Chemical bond primitives.

mod bond;
mod bond_descriptor;
mod bond_endpoint;
mod bond_kind;
mod bond_length;
mod bond_order;
mod bond_participant;
mod bond_polarity;
mod bond_strength;
mod error;

pub use bond::Bond;
pub use bond_descriptor::BondDescriptor;
pub use bond_endpoint::BondEndpoint;
pub use bond_kind::BondKind;
pub use bond_length::BondLength;
pub use bond_order::{BondOrder, FractionalBondOrder};
pub use bond_participant::BondParticipant;
pub use bond_polarity::BondPolarity;
pub use bond_strength::BondStrength;
pub use error::BondValidationError;

#[cfg(test)]
mod tests {
    use super::{
        Bond, BondDescriptor, BondEndpoint, BondKind, BondLength, BondOrder, BondParticipant,
        BondPolarity, BondStrength, BondValidationError, FractionalBondOrder,
    };

    fn endpoint(label: &str) -> BondEndpoint {
        BondEndpoint::new(label).expect("endpoint should be valid")
    }

    #[test]
    fn creates_simple_covalent_bond() {
        let bond = Bond::new(BondKind::Covalent).with_order(BondOrder::Single);

        assert_eq!(bond.kind(), BondKind::Covalent);
        assert_eq!(bond.order(), Some(BondOrder::Single));
        assert!(bond.endpoints().is_empty());
    }

    #[test]
    fn creates_common_bond_orders() {
        assert_eq!(BondOrder::Single.to_string(), "single");
        assert_eq!(BondOrder::Double.to_string(), "double");
        assert_eq!(BondOrder::Triple.to_string(), "triple");
        assert_eq!(BondOrder::Aromatic.to_string(), "aromatic");
        assert_eq!(
            BondOrder::Fractional(
                FractionalBondOrder::new(3, 2).expect("fractional order should be valid")
            )
            .to_string(),
            "3/2"
        );
    }

    #[test]
    fn creates_ionic_and_hydrogen_bonds() {
        let ionic = Bond::new(BondKind::Ionic).with_polarity(BondPolarity::Ionic);
        let hydrogen = Bond::new(BondKind::Hydrogen).with_strength(BondStrength::Weak);

        assert_eq!(ionic.kind(), BondKind::Ionic);
        assert_eq!(ionic.polarity(), Some(BondPolarity::Ionic));
        assert_eq!(hydrogen.kind(), BondKind::Hydrogen);
        assert_eq!(hydrogen.strength(), Some(BondStrength::Weak));
    }

    #[test]
    fn validates_endpoint_labels() {
        assert_eq!(
            BondEndpoint::new(" O ").map(|endpoint| endpoint.to_string()),
            Ok(String::from("O"))
        );
        assert_eq!(
            BondEndpoint::new("  "),
            Err(BondValidationError::EmptyEndpointLabel)
        );
    }

    #[test]
    fn creates_endpoint_pair_bonds() {
        let bond = Bond::between(endpoint("O"), endpoint("H"), BondKind::Covalent)
            .with_order(BondOrder::Single);

        assert_eq!(bond.endpoints().len(), 2);
        assert_eq!(bond.endpoints()[0].as_str(), "O");
        assert_eq!(bond.endpoints()[1].as_str(), "H");
        assert_eq!(bond.to_string(), "O-H covalent bond (single)");
    }

    #[test]
    fn assigns_polarity_independently_from_kind() {
        let bond = Bond::new(BondKind::Covalent).with_polarity(BondPolarity::Polar);

        assert_eq!(bond.kind(), BondKind::Covalent);
        assert_eq!(bond.polarity(), Some(BondPolarity::Polar));
        assert_eq!(BondPolarity::Nonpolar.to_string(), "nonpolar");
    }

    #[test]
    fn assigns_strength_independently_from_kind() {
        let bond = Bond::new(BondKind::VanDerWaals).with_strength(BondStrength::Weak);

        assert_eq!(bond.kind(), BondKind::VanDerWaals);
        assert_eq!(bond.strength(), Some(BondStrength::Weak));
        assert_eq!(BondStrength::VeryStrong.to_string(), "very strong");
    }

    #[test]
    fn represents_optional_bond_length() {
        let length = BondLength::new(96.0, "pm").expect("length should be valid");
        let bond = Bond::new(BondKind::Covalent).with_length(length.clone());

        assert_eq!(bond.length(), Some(&length));
        assert_eq!(length.value(), 96.0);
        assert_eq!(length.unit(), "pm");
        assert_eq!(length.to_string(), "96 pm");
    }

    #[test]
    fn rejects_invalid_length_values() {
        assert_eq!(
            BondLength::new(f64::NAN, "pm"),
            Err(BondValidationError::NonFiniteBondLength)
        );
        assert_eq!(
            BondLength::new(0.0, "pm"),
            Err(BondValidationError::NonPositiveBondLength)
        );
        assert_eq!(
            BondLength::new(96.0, ""),
            Err(BondValidationError::EmptyLengthUnit)
        );
    }

    #[test]
    fn rejects_invalid_empty_endpoint_labels() {
        assert_eq!(
            BondEndpoint::try_from(""),
            Err(BondValidationError::EmptyEndpointLabel)
        );
    }

    #[test]
    fn formats_bonds_predictably() {
        assert_eq!(Bond::new(BondKind::Covalent).to_string(), "covalent bond");
        assert_eq!(
            Bond::between(endpoint("C"), endpoint("C"), BondKind::Aromatic)
                .with_order(BondOrder::Aromatic)
                .to_string(),
            "C-C aromatic bond (aromatic)"
        );
        assert_eq!(BondKind::LondonDispersion.to_string(), "London dispersion");
    }

    #[test]
    fn validates_fractional_bond_orders() {
        let order = FractionalBondOrder::new(3, 2).expect("fractional order should be valid");

        assert_eq!(order.numerator(), 3);
        assert_eq!(order.denominator(), 2);
        assert_eq!(order.to_string(), "3/2");
        assert_eq!(
            FractionalBondOrder::new(0, 2),
            Err(BondValidationError::ZeroFractionalBondOrderNumerator)
        );
        assert_eq!(
            FractionalBondOrder::new(3, 0),
            Err(BondValidationError::ZeroFractionalBondOrderDenominator)
        );
    }

    #[test]
    fn assigns_descriptors_participants_and_angle_labels() {
        let descriptor = BondDescriptor::new("sigma").expect("descriptor should be valid");
        let participant = BondParticipant::new("ligand").expect("participant should be valid");
        let bond = Bond::new(BondKind::Coordinate)
            .with_descriptor(descriptor.clone())
            .with_descriptor(descriptor.clone())
            .with_participant(participant.clone())
            .try_with_angle_label("donor-metal-acceptor")
            .expect("angle label should be valid");

        assert_eq!(bond.descriptors(), &[descriptor]);
        assert_eq!(bond.participants(), &[participant]);
        assert_eq!(
            bond.angle_label().map(BondDescriptor::as_str),
            Some("donor-metal-acceptor")
        );
        assert_eq!(
            Bond::new(BondKind::Covalent).try_with_angle_label("  "),
            Err(BondValidationError::EmptyAngleLabel)
        );
    }
}
