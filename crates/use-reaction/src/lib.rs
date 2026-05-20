#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Chemical reaction representation primitives.

mod catalyst;
mod chemical_reaction;
mod error;
mod product;
mod reactant;
mod reaction_arrow;
mod reaction_condition;
mod reaction_condition_set;
mod reaction_direction;
mod reaction_equation;
mod reaction_kind;
mod reaction_term;
mod solvent;

pub use catalyst::Catalyst;
pub use chemical_reaction::ChemicalReaction;
pub use error::ReactionValidationError;
pub use product::Product;
pub use reactant::Reactant;
pub use reaction_arrow::ReactionArrow;
pub use reaction_condition::ReactionCondition;
pub use reaction_condition_set::ReactionConditionSet;
pub use reaction_direction::ReactionDirection;
pub use reaction_equation::ReactionEquation;
pub use reaction_kind::ReactionKind;
pub use reaction_term::ReactionTerm;
pub use solvent::Solvent;

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;
    use use_stoichiometry::StoichiometryValidationError;

    use super::{
        Catalyst, ChemicalReaction, Product, Reactant, ReactionArrow, ReactionCondition,
        ReactionConditionSet, ReactionEquation, ReactionKind, ReactionTerm,
        ReactionValidationError, Solvent,
    };

    fn formula(input: &str) -> ChemicalFormula {
        ChemicalFormula::parse(input).expect("test formula should parse")
    }

    fn term(coefficient: u32, input: &str) -> ReactionTerm {
        ReactionTerm::new(formula(input))
            .with_coefficient(coefficient)
            .expect("coefficient should be valid")
    }

    #[test]
    fn creates_simple_synthesis_reaction() {
        let reaction = ChemicalReaction::new()
            .with_reactant(term(2, "H2"))
            .with_reactant(term(1, "O2"))
            .with_product(term(2, "H2O"))
            .with_kind(ReactionKind::Synthesis);

        assert_eq!(reaction.to_string(), "2H2 + O2 -> 2H2O");
        assert_eq!(reaction.kinds(), &[ReactionKind::Synthesis]);
        assert_eq!(reaction.validate(), Ok(()));
    }

    #[test]
    fn creates_decomposition_reaction() {
        let reaction = ChemicalReaction::new()
            .with_reactant(term(1, "CaCO3"))
            .with_product(term(1, "CaO"))
            .with_product(term(1, "CO2"))
            .with_kind(ReactionKind::Decomposition);

        assert_eq!(reaction.to_string(), "CaCO3 -> CaO + CO2");
        assert_eq!(reaction.validate(), Ok(()));
    }

    #[test]
    fn creates_combustion_reaction() {
        let reaction = ChemicalReaction::new()
            .with_reactant(term(1, "CH4"))
            .with_reactant(term(2, "O2"))
            .with_product(term(1, "CO2"))
            .with_product(term(2, "H2O"))
            .with_kind(ReactionKind::Combustion);

        assert_eq!(reaction.to_string(), "CH4 + 2O2 -> CO2 + 2H2O");
        assert_eq!(reaction.kinds(), &[ReactionKind::Combustion]);
    }

    #[test]
    fn creates_reactants_and_products() {
        let reactant = Reactant::new(term(3, "H2"));
        let product = Product::new(term(2, "NH3"));

        assert_eq!(reactant.to_string(), "3H2");
        assert_eq!(product.to_string(), "2NH3");
        assert_eq!(reactant.as_term().coefficient().value(), 3);
        assert_eq!(product.as_term().formula().to_string(), "NH3");
    }

    #[test]
    fn displays_reaction_terms_and_omits_one() {
        let oxygen = term(1, "O2");
        let ammonia = term(2, "NH3");

        assert_eq!(oxygen.to_string(), "O2");
        assert_eq!(ammonia.to_string(), "2NH3");
        assert_eq!(oxygen.coefficient().value(), 1);
        assert!(oxygen.coefficient().is_one());
    }

    #[test]
    fn displays_reaction_arrows() {
        assert_eq!(ReactionArrow::Forward.to_string(), "->");
        assert_eq!(ReactionArrow::Reverse.to_string(), "<-");
        assert_eq!(ReactionArrow::Reversible.to_string(), "<->");
        assert_eq!(ReactionArrow::Equilibrium.to_string(), "⇌");
        assert!(ReactionArrow::Reversible.is_reversible());
        assert!(ReactionArrow::Equilibrium.is_reversible());
    }

    #[test]
    fn stores_catalyst_and_solvent_conditions() {
        let catalyst = Catalyst::new("Pt").expect("catalyst should be valid");
        let solvent = Solvent::new("water").expect("solvent should be valid");
        let conditions = ReactionConditionSet::new()
            .with_condition(ReactionCondition::Catalyst(catalyst.clone()))
            .with_condition(ReactionCondition::Solvent(solvent.clone()));

        assert_eq!(catalyst.to_string(), "Pt");
        assert_eq!(solvent.to_string(), "water");
        assert_eq!(conditions.len(), 2);
        assert_eq!(conditions.to_string(), "catalyst: Pt, solvent: water");
        assert_eq!(conditions.validate(), Ok(()));
    }

    #[test]
    fn stores_heat_and_light_condition_labels() {
        let conditions = ReactionConditionSet::new()
            .with_condition(ReactionCondition::Heat)
            .with_condition(ReactionCondition::Light);

        assert_eq!(ReactionCondition::Heat.to_string(), "heat");
        assert_eq!(ReactionCondition::Light.to_string(), "light");
        assert_eq!(conditions.to_string(), "heat, light");
    }

    #[test]
    fn assigns_reaction_kind_once() {
        let reaction = ChemicalReaction::new()
            .with_kind(ReactionKind::AcidBase)
            .with_kind(ReactionKind::AcidBase)
            .with_kind(ReactionKind::Neutralization);

        assert_eq!(ReactionKind::AcidBase.to_string(), "acid-base");
        assert_eq!(
            reaction.kinds(),
            &[ReactionKind::AcidBase, ReactionKind::Neutralization]
        );
    }

    #[test]
    fn validates_empty_and_incomplete_reactions() {
        assert_eq!(
            ChemicalReaction::new().validate(),
            Err(ReactionValidationError::EmptyReaction)
        );
        assert_eq!(
            ChemicalReaction::new()
                .with_product(term(1, "H2O"))
                .validate(),
            Err(ReactionValidationError::MissingReactants)
        );
        assert_eq!(
            ChemicalReaction::new()
                .with_reactant(term(1, "H2"))
                .validate(),
            Err(ReactionValidationError::MissingProducts)
        );
    }

    #[test]
    fn returns_structured_validation_errors() {
        assert_eq!(
            ReactionTerm::new(formula("H2")).with_coefficient(0),
            Err(ReactionValidationError::InvalidStoichiometry(
                StoichiometryValidationError::ZeroCoefficient
            ))
        );
        assert_eq!(
            Catalyst::new("  "),
            Err(ReactionValidationError::EmptyCatalystLabel)
        );
        assert_eq!(
            ReactionCondition::temperature("  "),
            Err(ReactionValidationError::EmptyTemperatureLabel)
        );
        assert_eq!(
            ReactionCondition::custom("  ", None),
            Err(ReactionValidationError::EmptyConditionLabel)
        );
    }

    #[test]
    fn validates_reaction_equations() {
        let equation = ReactionEquation::new()
            .with_reactant(term(1, "AgNO3"))
            .with_reactant(term(1, "NaCl"))
            .with_product(term(1, "AgCl"))
            .with_product(term(1, "NaNO3"));

        assert_eq!(equation.to_string(), "AgNO3 + NaCl -> AgCl + NaNO3");
        assert_eq!(equation.validate(), Ok(()));
    }
}
