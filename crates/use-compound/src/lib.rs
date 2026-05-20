#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Chemical compound identity primitives.

mod compound;
mod compound_formula;
mod compound_identifier;
mod compound_kind;
mod compound_name;
mod error;
mod registry;

pub use compound::Compound;
pub use compound_formula::{CompoundFormula, EmpiricalFormula, MolecularFormula};
pub use compound_identifier::CompoundIdentifier;
pub use compound_kind::CompoundKind;
pub use compound_name::{CommonName, CompoundName, SystematicName};
pub use error::CompoundValidationError;
pub use registry::CompoundRegistry;

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;

    use super::{
        CommonName, Compound, CompoundFormula, CompoundIdentifier, CompoundKind, CompoundName,
        CompoundRegistry, CompoundValidationError, EmpiricalFormula, MolecularFormula,
        SystematicName,
    };

    fn formula(input: &str) -> ChemicalFormula {
        ChemicalFormula::parse(input).expect("test formula should parse")
    }

    #[test]
    fn creates_simple_compound() {
        let water = Compound::new("water", formula("H2O")).expect("compound should be valid");

        assert_eq!(water.name().as_str(), "water");
        assert_eq!(water.formula().to_string(), "H2O");
        assert!(water.common_name().is_none());
        assert!(water.systematic_name().is_none());
        assert!(water.kinds().is_empty());
        assert!(water.identifiers().is_empty());
    }

    #[test]
    fn validates_compound_names() {
        assert_eq!(
            CompoundName::new("   "),
            Err(CompoundValidationError::EmptyName)
        );
        assert_eq!(
            Compound::new("", formula("H2O")).map(|compound| compound.name().to_string()),
            Err(CompoundValidationError::EmptyName)
        );
        assert_eq!(
            CompoundName::new(" water ").map(|name| name.to_string()),
            Ok(String::from("water"))
        );
    }

    #[test]
    fn handles_common_and_systematic_names() {
        let calcium_hydroxide = Compound::new("calcium hydroxide", formula("Ca(OH)2"))
            .expect("compound should be valid")
            .try_with_common_name("slaked lime")
            .expect("common name should be valid")
            .try_with_systematic_name("calcium dihydroxide")
            .expect("systematic name should be valid");

        assert_eq!(
            calcium_hydroxide.common_name().map(CommonName::as_str),
            Some("slaked lime")
        );
        assert_eq!(
            calcium_hydroxide
                .systematic_name()
                .map(SystematicName::as_str),
            Some("calcium dihydroxide")
        );
        assert_eq!(
            CommonName::new(""),
            Err(CompoundValidationError::EmptyCommonName)
        );
        assert_eq!(
            SystematicName::new("  "),
            Err(CompoundValidationError::EmptySystematicName)
        );
    }

    #[test]
    fn wraps_empirical_and_molecular_formulas() {
        let empirical = EmpiricalFormula::new(formula("CH2O"));
        let molecular = MolecularFormula::new(formula("C6H12O6"));
        let glucose = Compound::new("glucose", formula("C6H12O6"))
            .expect("compound should be valid")
            .with_empirical_formula(empirical.clone())
            .with_molecular_formula(molecular.clone());

        assert_eq!(empirical.to_string(), "CH2O");
        assert_eq!(molecular.to_string(), "C6H12O6");
        assert_eq!(
            glucose.empirical_formula().map(ToString::to_string),
            Some(String::from("CH2O"))
        );
        assert_eq!(
            glucose.molecular_formula().map(ToString::to_string),
            Some(String::from("C6H12O6"))
        );
        assert_eq!(
            CompoundFormula::new(formula("H2O"))
                .as_formula()
                .to_string(),
            "H2O"
        );
    }

    #[test]
    fn assigns_compound_kinds() {
        let sodium_chloride = Compound::new("sodium chloride", formula("NaCl"))
            .expect("compound should be valid")
            .with_kind(CompoundKind::Ionic)
            .with_kind(CompoundKind::Salt)
            .with_kind(CompoundKind::Salt);

        assert_eq!(
            sodium_chloride.kinds(),
            &[CompoundKind::Ionic, CompoundKind::Salt]
        );
        assert_eq!(CompoundKind::Coordination.to_string(), "coordination");
    }

    #[test]
    fn handles_compound_identifiers() {
        let cas = CompoundIdentifier::cas_number("7732-18-5").expect("CAS should be valid");
        let custom = CompoundIdentifier::custom("local", "water")
            .expect("custom identifier should be valid");
        let water = Compound::new("water", formula("H2O"))
            .expect("compound should be valid")
            .try_with_identifier(cas.clone())
            .expect("identifier should be valid")
            .try_with_identifier(custom.clone())
            .expect("identifier should be valid");

        assert_eq!(cas.registry(), &CompoundRegistry::CasNumber);
        assert_eq!(cas.value(), "7732-18-5");
        assert_eq!(custom.registry().to_string(), "local");
        assert_eq!(custom.value(), "water");
        assert_eq!(water.identifiers(), &[cas, custom]);
    }

    #[test]
    fn displays_compounds() {
        let carbon_dioxide =
            Compound::new("carbon dioxide", formula("CO2")).expect("compound should be valid");

        assert_eq!(carbon_dioxide.to_string(), "carbon dioxide (CO2)");
    }

    #[test]
    fn rejects_invalid_identifier_values() {
        assert_eq!(
            CompoundIdentifier::pub_chem_cid(""),
            Err(CompoundValidationError::EmptyIdentifierValue)
        );
        assert_eq!(
            CompoundIdentifier::custom("", "value"),
            Err(CompoundValidationError::EmptyIdentifierNamespace)
        );
        assert_eq!(
            CompoundIdentifier::custom("local", "  "),
            Err(CompoundValidationError::EmptyIdentifierValue)
        );
    }
}
