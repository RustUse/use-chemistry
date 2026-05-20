use crate::{
    ChemicalFormula, ElementCount, ElementSymbol, FormulaGroup, FormulaMultiplier,
    FormulaParseError, FormulaPart, FormulaTerm, HydratePart,
};

pub(crate) fn parse_formula(input: &str) -> Result<ChemicalFormula, FormulaParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(FormulaParseError::EmptyFormula);
    }

    let mut parser = Parser::new(trimmed);
    let main_part = parser.parse_part(false)?;
    let mut hydrate_parts = Vec::new();

    while let Some(separator) = parser.peek() {
        if !is_hydrate_separator(separator) {
            return Err(FormulaParseError::UnexpectedCharacter(separator));
        }

        parser.advance();
        if parser.peek().is_none() {
            return Err(FormulaParseError::TrailingSeparator);
        }

        let multiplier = parser.parse_optional_multiplier()?.unwrap_or_default();
        let part = parser.parse_part(false)?;
        hydrate_parts.push(HydratePart::new(multiplier, part));
    }

    Ok(ChemicalFormula::new(main_part, hydrate_parts))
}

struct Parser {
    characters: Vec<char>,
    index: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            characters: input.chars().collect(),
            index: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.index).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.index += 1;
        Some(character)
    }

    fn parse_part(&mut self, stop_at_group_close: bool) -> Result<FormulaPart, FormulaParseError> {
        let mut terms = Vec::new();

        while let Some(character) = self.peek() {
            if is_hydrate_separator(character) {
                break;
            }

            match character {
                ')' if stop_at_group_close => break,
                ')' => return Err(FormulaParseError::UnmatchedCloseGroup),
                '(' => terms.push(FormulaTerm::group(self.parse_group()?)),
                character if character.is_ascii_uppercase() => {
                    terms.push(self.parse_element_term()?);
                },
                character if character.is_ascii_lowercase() => {
                    return Err(FormulaParseError::InvalidSymbol(character.to_string()));
                },
                character => return Err(FormulaParseError::UnexpectedCharacter(character)),
            }
        }

        FormulaPart::new(terms).map_err(Into::into)
    }

    fn parse_group(&mut self) -> Result<FormulaGroup, FormulaParseError> {
        self.advance();

        if self.peek().is_none() {
            return Err(FormulaParseError::UnmatchedOpenGroup);
        }

        if self.peek() == Some(')') {
            return Err(FormulaParseError::EmptyGroup);
        }

        let part = self.parse_part(true)?;

        match self.peek() {
            Some(')') => {
                self.advance();
            },
            None => return Err(FormulaParseError::UnmatchedOpenGroup),
            Some(character) => return Err(FormulaParseError::UnexpectedCharacter(character)),
        }

        let multiplier = self.parse_optional_multiplier()?.unwrap_or_default();
        FormulaGroup::new(part.terms().to_vec(), multiplier).map_err(Into::into)
    }

    fn parse_element_term(&mut self) -> Result<FormulaTerm, FormulaParseError> {
        let symbol = self.parse_symbol()?;
        let count = self.parse_optional_count()?.unwrap_or_default();
        Ok(FormulaTerm::element(symbol, count))
    }

    fn parse_symbol(&mut self) -> Result<ElementSymbol, FormulaParseError> {
        let Some(first) = self.advance() else {
            return Err(FormulaParseError::UnexpectedEnd);
        };

        if !first.is_ascii_uppercase() {
            return Err(FormulaParseError::InvalidSymbol(first.to_string()));
        }

        let mut symbol = first.to_string();
        if let Some(second) = self.peek()
            && second.is_ascii_lowercase()
        {
            symbol.push(second);
            self.advance();
        }

        ElementSymbol::new(&symbol).map_err(Into::into)
    }

    fn parse_optional_count(&mut self) -> Result<Option<ElementCount>, FormulaParseError> {
        let Some(number) = self.parse_optional_number()? else {
            return Ok(None);
        };

        Ok(Some(ElementCount::new(number)?))
    }

    fn parse_optional_multiplier(
        &mut self,
    ) -> Result<Option<FormulaMultiplier>, FormulaParseError> {
        let Some(number) = self.parse_optional_number()? else {
            return Ok(None);
        };

        Ok(Some(FormulaMultiplier::new(number)?))
    }

    fn parse_optional_number(&mut self) -> Result<Option<u32>, FormulaParseError> {
        let mut number = String::new();

        while let Some(character) = self.peek() {
            if character.is_ascii_digit() {
                number.push(character);
                self.advance();
            } else {
                break;
            }
        }

        if number.is_empty() {
            return Ok(None);
        }

        number
            .parse::<u32>()
            .map(Some)
            .map_err(|_| FormulaParseError::InvalidNumber(number))
    }
}

fn is_hydrate_separator(character: char) -> bool {
    matches!(character, '.' | '·')
}
