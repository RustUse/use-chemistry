#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Ion identity and charge primitives.

mod anion;
mod cation;
mod charge_sign;
mod error;
mod ion;
mod ion_charge;
mod ion_formula;
mod ion_kind;
mod ion_name;
mod monatomic_ion;
mod polyatomic_ion;

pub use anion::Anion;
pub use cation::Cation;
pub use charge_sign::ChargeSign;
pub use error::IonValidationError;
pub use ion::Ion;
pub use ion_charge::{ChargeMagnitude, IonCharge};
pub use ion_formula::IonFormula;
pub use ion_kind::IonKind;
pub use ion_name::IonName;
pub use monatomic_ion::MonatomicIon;
pub use polyatomic_ion::PolyatomicIon;

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;

    use super::{
        Anion, Cation, ChargeMagnitude, ChargeSign, Ion, IonCharge, IonFormula, IonKind, IonName,
        IonValidationError, MonatomicIon, PolyatomicIon,
    };

    fn formula(input: &str) -> ChemicalFormula {
        ChemicalFormula::parse(input).expect("test formula should parse")
    }

    fn positive(magnitude: u8) -> IonCharge {
        IonCharge::positive(magnitude).expect("positive charge should be valid")
    }

    fn negative(magnitude: u8) -> IonCharge {
        IonCharge::negative(magnitude).expect("negative charge should be valid")
    }

    #[test]
    fn creates_positive_monatomic_ions() {
        let sodium = Ion::new(formula("Na"), positive(1)).with_kind(IonKind::Monatomic);
        let calcium = Ion::new(formula("Ca"), positive(2));

        assert!(sodium.is_cation());
        assert_eq!(sodium.charge().magnitude(), 1);
        assert_eq!(sodium.to_string(), "Na+");
        assert_eq!(calcium.to_string(), "Ca^2+");
    }

    #[test]
    fn creates_negative_monatomic_ions() {
        let chloride = Ion::new(formula("Cl"), negative(1)).with_kind(IonKind::Monatomic);

        assert!(chloride.is_anion());
        assert_eq!(chloride.charge().sign(), ChargeSign::Negative);
        assert_eq!(chloride.to_string(), "Cl-");
    }

    #[test]
    fn creates_positive_polyatomic_ions() {
        let ammonium = Ion::new(formula("NH4"), positive(1)).with_kind(IonKind::Polyatomic);

        assert!(ammonium.is_cation());
        assert_eq!(ammonium.kinds(), &[IonKind::Polyatomic]);
        assert_eq!(ammonium.to_string(), "NH4+");
    }

    #[test]
    fn creates_negative_polyatomic_ions() {
        let sulfate = Ion::new(formula("SO4"), negative(2)).with_kind(IonKind::Polyatomic);
        let carbonate = Ion::new(formula("CO3"), negative(2));

        assert!(sulfate.is_anion());
        assert_eq!(sulfate.to_string(), "SO4^2-");
        assert_eq!(carbonate.to_string(), "CO3^2-");
    }

    #[test]
    fn validates_charge_magnitude() {
        assert_eq!(
            ChargeMagnitude::new(0),
            Err(IonValidationError::ZeroChargeMagnitude)
        );
        assert_eq!(
            IonCharge::positive(0),
            Err(IonValidationError::ZeroChargeMagnitude)
        );
        assert_eq!(
            IonCharge::negative(0),
            Err(IonValidationError::ZeroChargeMagnitude)
        );
    }

    #[test]
    fn exposes_charge_sign_helpers() {
        assert!(ChargeSign::Positive.is_positive());
        assert!(ChargeSign::Negative.is_negative());
        assert_eq!(ChargeSign::Positive.to_string(), "+");
        assert_eq!(ChargeSign::Negative.to_string(), "-");
        assert_eq!(positive(3).to_string(), "3+");
        assert_eq!(negative(2).to_string(), "2-");
    }

    #[test]
    fn detects_cations_and_anions_from_charge() {
        let sodium = Ion::new(formula("Na"), positive(1));
        let chloride = Ion::new(formula("Cl"), negative(1));

        assert!(sodium.is_cation());
        assert!(!sodium.is_anion());
        assert!(chloride.is_anion());
        assert!(!chloride.is_cation());
    }

    #[test]
    fn exposes_formula_wrappers() {
        let formula = IonFormula::new(formula("NO3"));
        let nitrate = Ion::new(formula.clone().into_formula(), negative(1));

        assert_eq!(nitrate.formula().to_string(), "NO3");
        assert_eq!(nitrate.ion_formula().to_string(), "NO3");
        assert_eq!(nitrate.to_string(), "NO3-");
    }

    #[test]
    fn validates_ion_names() {
        let hydronium = Ion::new(formula("H3O"), positive(1))
            .try_with_name(" hydronium ")
            .expect("ion name should be valid");

        assert_eq!(IonName::new("  "), Err(IonValidationError::EmptyName));
        assert_eq!(hydronium.name().map(IonName::as_str), Some("hydronium"));
        assert_eq!(hydronium.to_string(), "H3O+");
    }

    #[test]
    fn rejects_invalid_empty_names() {
        assert_eq!(
            Ion::new(formula("Na"), positive(1)).try_with_name(""),
            Err(IonValidationError::EmptyName)
        );
    }

    #[test]
    fn wraps_cations_and_anions() {
        let calcium = Cation::new(formula("Ca"), 2).expect("cation should be valid");
        let chloride = Anion::new(formula("Cl"), 1).expect("anion should be valid");

        assert_eq!(calcium.as_ion().to_string(), "Ca^2+");
        assert_eq!(chloride.as_ion().to_string(), "Cl-");
        assert_eq!(
            Cation::from_ion(chloride.as_ion().clone()),
            Err(IonValidationError::ExpectedCation)
        );
        assert_eq!(
            Anion::from_ion(calcium.as_ion().clone()),
            Err(IonValidationError::ExpectedAnion)
        );
    }

    #[test]
    fn wraps_monatomic_and_polyatomic_ions() {
        let sodium =
            MonatomicIon::new(formula("Na"), positive(1)).expect("monatomic ion should be valid");
        let ammonium = PolyatomicIon::new(formula("NH4"), positive(1))
            .expect("polyatomic ion should be valid");

        assert_eq!(sodium.as_ion().kinds(), &[IonKind::Monatomic]);
        assert_eq!(ammonium.as_ion().kinds(), &[IonKind::Polyatomic]);
        assert_eq!(
            MonatomicIon::new(formula("NH4"), positive(1)),
            Err(IonValidationError::ExpectedMonatomicFormula)
        );
        assert_eq!(
            PolyatomicIon::new(formula("Na"), positive(1)),
            Err(IonValidationError::ExpectedPolyatomicFormula)
        );
    }

    #[test]
    fn assigns_oxidation_state_labels() {
        let iron = Ion::new(formula("Fe"), positive(3))
            .try_with_oxidation_state_label("III")
            .expect("oxidation-state label should be valid");

        assert_eq!(iron.oxidation_state_label(), Some("III"));
        assert_eq!(iron.to_string(), "Fe^3+");
        assert_eq!(
            Ion::new(formula("Fe"), positive(2)).try_with_oxidation_state_label(" "),
            Err(IonValidationError::EmptyOxidationStateLabel)
        );
    }
}
