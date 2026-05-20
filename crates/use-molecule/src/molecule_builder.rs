use use_chemical_formula::ChemicalFormula;

use crate::{
    AtomConnection, MolecularAtom, Molecule, MoleculeCharge, MoleculeKind, MoleculeValidationError,
};

/// Builder for assembling a molecule with optional explicit atom data.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MoleculeBuilder {
    name: String,
    formula: Option<ChemicalFormula>,
    atoms: Vec<MolecularAtom>,
    connections: Vec<AtomConnection>,
    charge: MoleculeCharge,
    kinds: Vec<MoleculeKind>,
}

impl MoleculeBuilder {
    /// Creates a molecule builder.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            formula: None,
            atoms: Vec::new(),
            connections: Vec::new(),
            charge: MoleculeCharge::NEUTRAL,
            kinds: Vec::new(),
        }
    }

    /// Sets the molecule formula.
    #[must_use]
    pub fn formula(mut self, formula: ChemicalFormula) -> Self {
        self.formula = Some(formula);
        self
    }

    /// Adds an explicit atom.
    #[must_use]
    pub fn atom(mut self, atom: MolecularAtom) -> Self {
        self.atoms.push(atom);
        self
    }

    /// Adds an atom connection to validate during build.
    #[must_use]
    pub fn connection(mut self, connection: AtomConnection) -> Self {
        self.connections.push(connection);
        self
    }

    /// Sets the formal molecule charge.
    #[must_use]
    pub const fn charge(mut self, charge: MoleculeCharge) -> Self {
        self.charge = charge;
        self
    }

    /// Adds a kind label if it is not already present.
    #[must_use]
    pub fn kind(mut self, kind: MoleculeKind) -> Self {
        if !self.kinds.contains(&kind) {
            self.kinds.push(kind);
        }
        self
    }

    /// Builds the molecule.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyName`] for an empty name,
    /// [`MoleculeValidationError::MissingFormula`] if no formula was assigned, or
    /// [`MoleculeValidationError::InvalidConnectionIndex`] when a connection references an atom
    /// index outside the explicit atom list.
    pub fn build(self) -> Result<Molecule, MoleculeValidationError> {
        let Some(formula) = self.formula else {
            return Err(MoleculeValidationError::MissingFormula);
        };

        let mut molecule = Molecule::new(&self.name, formula)?.with_charge(self.charge);
        for kind in self.kinds {
            molecule = molecule.with_kind(kind);
        }
        for atom in self.atoms {
            molecule = molecule.with_atom(atom);
        }
        for connection in self.connections {
            molecule = molecule.try_with_connection(connection)?;
        }

        Ok(molecule)
    }
}
