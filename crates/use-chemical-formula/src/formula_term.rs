use std::collections::BTreeMap;
use std::fmt;

use crate::{ElementCount, ElementSymbol, FormulaGroup};

/// A formula term: either an element with a count or a parenthesized group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaTerm {
    /// Element term such as `H2` or `Na`.
    Element {
        /// The element symbol.
        symbol: ElementSymbol,
        /// The element count.
        count: ElementCount,
    },
    /// Parenthesized group term.
    Group(FormulaGroup),
}

impl FormulaTerm {
    /// Creates an element term.
    #[must_use]
    pub const fn element(symbol: ElementSymbol, count: ElementCount) -> Self {
        Self::Element { symbol, count }
    }

    /// Creates a group term.
    #[must_use]
    pub const fn group(group: FormulaGroup) -> Self {
        Self::Group(group)
    }

    pub(crate) fn add_counts(&self, counts: &mut BTreeMap<String, u64>, multiplier: u64) {
        match self {
            Self::Element { symbol, count } => {
                let amount = u64::from(count.get()).saturating_mul(multiplier);
                let entry = counts.entry(symbol.as_str().to_owned()).or_insert(0);
                *entry = entry.saturating_add(amount);
            },
            Self::Group(group) => group.add_counts(counts, multiplier),
        }
    }
}

impl fmt::Display for FormulaTerm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element { symbol, count } => {
                write!(formatter, "{symbol}")?;
                if !count.is_one() {
                    write!(formatter, "{count}")?;
                }
                Ok(())
            },
            Self::Group(group) => write!(formatter, "{group}"),
        }
    }
}
