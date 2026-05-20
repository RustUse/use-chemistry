# Changelog

## Unreleased

### Added

- Added `release-plz` configuration and follow-up release workflows for the
  `use-chemistry` workspace.
- Added a manual publish workflow and maintainer release guidance for the
  initial chemistry crate publish wave.
- Added `use-chemical-formula` for structural chemical formula primitives,
  lightweight parsing, display, and expanded element counts.
- Added `use-stoichiometry` for formula-backed stoichiometric coefficients,
  terms, mole ratios, reaction-side entries, reagent labels, yield helpers,
  validation errors, and umbrella reexports through `use-chemistry`.
- Added `use-reaction` for formula-backed chemical reaction equations,
  reactants, products, arrows, conditions, catalysts, solvents, classification
  labels, validation errors, and umbrella reexports through `use-chemistry`.
- Added `use-molar-mass` for validated molar-mass values, atomic-mass lookup
  entries, formula molar-mass calculations, element contribution totals, and
  umbrella reexports through `use-chemistry`.
- Added `use-bond` for chemical bond identity primitives, bond kind/order
  labels, endpoint references, polarity and strength labels, optional length,
  and lightweight descriptors.
- Added `use-oxidation-state` for oxidation-state value primitives, bounded
  magnitudes, Roman numeral labels, element/formula assignments, assignment
  sets, validation errors, and umbrella reexports through `use-chemistry`.
- Added `use-ion` for formula-backed charged atom and charged group
  primitives, nonzero ion charges, cation/anion helpers, monatomic/polyatomic
  wrappers, optional names, and oxidation-state labels.
- Added `use-compound` for named chemical compound identity primitives,
  lightweight classification labels, optional registry identifiers, and formula
  wrappers backed by `use-chemical-formula`.
- Added `use-molecule` for discrete molecule identity primitives, optional
  explicit atom lists, simple atom connections, formal charge, and molecule
  classification labels backed by `use-chemical-formula`.
- Added `use-isotope` for chemistry-facing isotope identity, count, and
  notation helpers, plus umbrella reexports through `use-chemistry`.
