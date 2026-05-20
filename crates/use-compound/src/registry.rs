use std::fmt;

/// A compound identifier registry namespace.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompoundRegistry {
    /// CAS Registry Number namespace.
    CasNumber,
    /// PubChem Compound Identifier namespace.
    PubChemCid,
    /// InChI namespace.
    Inchi,
    /// InChIKey namespace.
    InchiKey,
    /// SMILES namespace.
    Smiles,
    /// A custom namespace.
    Custom(String),
}

impl CompoundRegistry {
    /// Returns the registry namespace text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::CasNumber => "CAS",
            Self::PubChemCid => "PubChem CID",
            Self::Inchi => "InChI",
            Self::InchiKey => "InChIKey",
            Self::Smiles => "SMILES",
            Self::Custom(namespace) => namespace.as_str(),
        }
    }
}

impl fmt::Display for CompoundRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
