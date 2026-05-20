#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

//! Molecular identity primitives.

mod atom_connection;
mod atom_index;
mod atom_label;
mod error;
mod molecular_atom;
mod molecular_formula;
mod molecule;
mod molecule_builder;
mod molecule_charge;
mod molecule_kind;
mod molecule_name;

pub use atom_connection::AtomConnection;
pub use atom_index::{AtomCount, AtomIndex};
pub use atom_label::AtomLabel;
pub use error::MoleculeValidationError;
pub use molecular_atom::{MolecularAtom, MolecularAtomId};
pub use molecular_formula::MolecularFormula;
pub use molecule::Molecule;
pub use molecule_builder::MoleculeBuilder;
pub use molecule_charge::MoleculeCharge;
pub use molecule_kind::MoleculeKind;
pub use molecule_name::MoleculeName;

#[cfg(test)]
mod tests {
    use use_chemical_formula::ChemicalFormula;

    use super::{
        AtomConnection, AtomCount, AtomIndex, AtomLabel, MolecularAtom, MolecularAtomId,
        MolecularFormula, Molecule, MoleculeCharge, MoleculeKind, MoleculeName,
        MoleculeValidationError,
    };

    fn formula(input: &str) -> ChemicalFormula {
        ChemicalFormula::parse(input).expect("test formula should parse")
    }

    #[test]
    fn creates_simple_molecule() {
        let water = Molecule::new("water", formula("H2O")).expect("molecule should be valid");

        assert_eq!(water.name().as_str(), "water");
        assert_eq!(water.formula().to_string(), "H2O");
        assert_eq!(water.atom_count(), 0);
        assert!(water.atoms().is_empty());
        assert!(water.connections().is_empty());
    }

    #[test]
    fn validates_molecule_names() {
        assert_eq!(
            MoleculeName::new("   "),
            Err(MoleculeValidationError::EmptyName)
        );
        assert_eq!(
            Molecule::new("", formula("H2O")).map(|molecule| molecule.name().to_string()),
            Err(MoleculeValidationError::EmptyName)
        );
        assert_eq!(
            MoleculeName::new(" water ").map(|name| name.to_string()),
            Ok(String::from("water"))
        );
    }

    #[test]
    fn assigns_and_displays_formula() {
        let methane_formula = MolecularFormula::new(formula("CH4"));
        let methane = Molecule::new("methane", methane_formula.clone().into_formula())
            .expect("molecule should be valid");

        assert_eq!(methane_formula.as_formula().to_string(), "CH4");
        assert_eq!(methane.molecular_formula().to_string(), "CH4");
        assert_eq!(methane.to_string(), "methane (CH4)");
    }

    #[test]
    fn assigns_neutral_kind_and_charge() {
        let oxygen = Molecule::new("oxygen", formula("O2"))
            .expect("molecule should be valid")
            .with_kind(MoleculeKind::Neutral)
            .with_kind(MoleculeKind::Diatomic)
            .with_kind(MoleculeKind::Diatomic)
            .with_charge(MoleculeCharge::NEUTRAL);

        assert_eq!(
            oxygen.kinds(),
            &[MoleculeKind::Neutral, MoleculeKind::Diatomic]
        );
        assert_eq!(oxygen.charge(), MoleculeCharge::NEUTRAL);
        assert!(oxygen.charge().is_neutral());
        assert_eq!(MoleculeCharge::new(1).to_string(), "+1");
        assert_eq!(MoleculeKind::Polyatomic.to_string(), "polyatomic");
    }

    #[test]
    fn supports_explicit_atoms_and_atom_ids() {
        let oxygen_atom = MolecularAtom::new("O")
            .expect("atom label should be valid")
            .try_with_id("oxygen-1")
            .expect("atom id should be valid");
        let hydrogen = MolecularAtom::new("H").expect("atom label should be valid");
        let water = Molecule::new("water", formula("H2O"))
            .expect("molecule should be valid")
            .with_atom(oxygen_atom.clone())
            .with_atom(hydrogen.clone())
            .with_atom(hydrogen);

        assert_eq!(water.atom_count(), 3);
        assert_eq!(water.atom_count_value(), AtomCount::new(3));
        assert_eq!(water.atoms()[0], oxygen_atom);
        assert_eq!(oxygen_atom.label().to_string(), "O");
        assert_eq!(
            oxygen_atom.id().map(MolecularAtomId::as_str),
            Some("oxygen-1")
        );
    }

    #[test]
    fn exposes_atom_indices_and_connections() {
        let connection = AtomConnection::new(AtomIndex::new(0), AtomIndex::new(1), Some(1))
            .expect("connection should be valid");
        let molecule = Molecule::new("hydrogen", formula("H2"))
            .expect("molecule should be valid")
            .with_atom(MolecularAtom::new("H").expect("atom label should be valid"))
            .with_atom(MolecularAtom::new("H").expect("atom label should be valid"))
            .try_with_connection(connection)
            .expect("connection indices should be valid");

        assert_eq!(AtomIndex::new(1).get(), 1);
        assert_eq!(molecule.connections().len(), 1);
        assert_eq!(molecule.connections()[0].from.get(), 0);
        assert_eq!(molecule.connections()[0].to.get(), 1);
        assert_eq!(molecule.connections()[0].order, Some(1));
    }

    #[test]
    fn rejects_invalid_atom_labels_and_ids() {
        assert_eq!(
            AtomLabel::new(""),
            Err(MoleculeValidationError::EmptyAtomLabel)
        );
        assert_eq!(
            MolecularAtom::new("water"),
            Err(MoleculeValidationError::InvalidAtomLabel(String::from(
                "water"
            )))
        );
        assert_eq!(
            MolecularAtomId::new("  "),
            Err(MoleculeValidationError::EmptyAtomId)
        );
    }

    #[test]
    fn rejects_invalid_atom_connections() {
        assert_eq!(
            AtomConnection::new(AtomIndex::new(0), AtomIndex::new(0), Some(1)),
            Err(MoleculeValidationError::SelfConnection { index: 0 })
        );
        assert_eq!(
            AtomConnection::new(AtomIndex::new(0), AtomIndex::new(1), Some(0)),
            Err(MoleculeValidationError::ZeroConnectionOrder)
        );

        let connection = AtomConnection::new(AtomIndex::new(0), AtomIndex::new(2), None)
            .expect("connection should be structurally valid");
        let molecule = Molecule::new("water", formula("H2O"))
            .expect("molecule should be valid")
            .with_atom(MolecularAtom::new("O").expect("atom label should be valid"))
            .with_atom(MolecularAtom::new("H").expect("atom label should be valid"));

        assert_eq!(
            molecule.try_with_connection(connection),
            Err(MoleculeValidationError::InvalidConnectionIndex {
                index: 2,
                atom_count: 2,
            })
        );
    }

    #[test]
    fn builds_molecules_with_builder() {
        let molecule = Molecule::builder("water")
            .formula(formula("H2O"))
            .atom(MolecularAtom::new("O").expect("atom label should be valid"))
            .atom(MolecularAtom::new("H").expect("atom label should be valid"))
            .atom(MolecularAtom::new("H").expect("atom label should be valid"))
            .connection(
                AtomConnection::new(AtomIndex::new(0), AtomIndex::new(1), Some(1))
                    .expect("connection should be valid"),
            )
            .kind(MoleculeKind::Neutral)
            .build()
            .expect("builder should produce a molecule");

        assert_eq!(molecule.atom_count(), 3);
        assert_eq!(molecule.connections().len(), 1);
        assert_eq!(molecule.kinds(), &[MoleculeKind::Neutral]);
        assert_eq!(
            Molecule::builder("missing formula").build(),
            Err(MoleculeValidationError::MissingFormula)
        );
    }
}
