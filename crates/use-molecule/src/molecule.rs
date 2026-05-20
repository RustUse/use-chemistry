use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{
    AtomConnection, AtomCount, MolecularAtom, MolecularFormula, MoleculeBuilder, MoleculeCharge,
    MoleculeKind, MoleculeName, MoleculeValidationError,
};

/// A named molecular entity with formula and optional structural primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Molecule {
    name: MoleculeName,
    formula: MolecularFormula,
    atoms: Vec<MolecularAtom>,
    connections: Vec<AtomConnection>,
    charge: MoleculeCharge,
    kinds: Vec<MoleculeKind>,
}

impl Molecule {
    /// Creates a molecule from a name and formula.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn new(name: &str, formula: ChemicalFormula) -> Result<Self, MoleculeValidationError> {
        Ok(Self {
            name: MoleculeName::new(name)?,
            formula: MolecularFormula::new(formula),
            atoms: Vec::new(),
            connections: Vec::new(),
            charge: MoleculeCharge::NEUTRAL,
            kinds: Vec::new(),
        })
    }

    /// Starts a molecule builder.
    #[must_use]
    pub fn builder(name: &str) -> MoleculeBuilder {
        MoleculeBuilder::new(name)
    }

    /// Returns the molecule name.
    #[must_use]
    pub const fn name(&self) -> &MoleculeName {
        &self.name
    }

    /// Returns the molecule formula.
    #[must_use]
    pub fn formula(&self) -> &ChemicalFormula {
        self.formula.as_formula()
    }

    /// Returns the molecule formula wrapper.
    #[must_use]
    pub const fn molecular_formula(&self) -> &MolecularFormula {
        &self.formula
    }

    /// Returns explicit atoms in insertion order.
    #[must_use]
    pub fn atoms(&self) -> &[MolecularAtom] {
        &self.atoms
    }

    /// Returns the number of explicit atoms.
    #[must_use]
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Returns the explicit atom count wrapper.
    #[must_use]
    pub fn atom_count_value(&self) -> AtomCount {
        AtomCount::new(self.atom_count())
    }

    /// Returns explicit atom connections in insertion order.
    #[must_use]
    pub fn connections(&self) -> &[AtomConnection] {
        &self.connections
    }

    /// Returns the formal molecule charge.
    #[must_use]
    pub const fn charge(&self) -> MoleculeCharge {
        self.charge
    }

    /// Returns molecule kind labels in insertion order.
    #[must_use]
    pub fn kinds(&self) -> &[MoleculeKind] {
        &self.kinds
    }

    /// Adds a kind label if it is not already present.
    #[must_use]
    pub fn with_kind(mut self, kind: MoleculeKind) -> Self {
        if !self.kinds.contains(&kind) {
            self.kinds.push(kind);
        }
        self
    }

    /// Sets the formal molecule charge.
    #[must_use]
    pub const fn with_charge(mut self, charge: MoleculeCharge) -> Self {
        self.charge = charge;
        self
    }

    /// Adds an explicit atom.
    #[must_use]
    pub fn with_atom(mut self, atom: MolecularAtom) -> Self {
        self.atoms.push(atom);
        self
    }

    /// Adds an atom connection after validating its indices against the explicit atom list.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::InvalidConnectionIndex`] when the connection references an
    /// atom index outside the explicit atom list.
    pub fn try_with_connection(
        mut self,
        connection: AtomConnection,
    ) -> Result<Self, MoleculeValidationError> {
        let connection = connection.validate_indices(self.atoms.len())?;
        if !self.connections.contains(&connection) {
            self.connections.push(connection);
        }
        Ok(self)
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.formula)
    }
}
