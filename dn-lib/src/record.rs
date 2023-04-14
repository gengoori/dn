use crate::formula::{Formula,TokenizationError};
use crate::justif::{Jusitification, ReadError as JusitifReadError};

#[derive(Debug)]
pub enum RecordError {
    /// A field is missing
    MissingField,
    /// The id field is invalid
    InvalidId,
    /// The ctxt field is invalid    
    InvalidCtxt,
    /// There are too many fields
    TooMuch,
    /// Error parsing the formula
    InvalidFormula(TokenizationError),

    /// Error parsing the justification
    InvalidJustif(JusitifReadError),
}

#[derive(Debug)]
pub struct Record {
    pub id: usize,
    pub ctxt: Vec<usize>,
    pub stmt: Statement,
    pub justif: Jusitification,
}

impl Record {
    /// Reads a record
    pub fn read_record(input: &str) -> Result<Self, RecordError> {
        let mut input = input.split(';');
        let id = Self::read_id(input.next().ok_or(RecordError::MissingField)?)?;
        let ctxt = Self::read_ctxt(input.next().ok_or(RecordError::MissingField)?)?;
        let stmt = Self::read_stmt(input.next().ok_or(RecordError::MissingField)?)?;
        let justif = Self::read_justif(input.next().ok_or(RecordError::MissingField)?)?;
        match input.next() {
            Some(_) => Err(RecordError::TooMuch),
            None => Ok(Self {
                id,
                ctxt,
                stmt,
                justif,
            }),
        }
    }

    /// Reads the id of the record
    fn read_id(input: &str) -> Result<usize, RecordError> {
        input.parse::<usize>().map_err(|_| RecordError::InvalidId)
    }

    /// Reads the context of the record
    fn read_ctxt(input: &str) -> Result<Vec<usize>, RecordError> {
        input
            .split(|c: char| c == ',')
            .filter(|slc| ! slc.is_empty())
            .map(|slc: &str| slc.parse::<usize>().map_err(|_| RecordError::InvalidCtxt))
            .collect()
    }

    /// Reads a statement
    fn read_stmt(input: &str) -> Result<Statement, RecordError> {
        match input.strip_prefix("Donc ") {
            Some(input) => {
                let f = Formula::read(input).map_err(|e| RecordError::InvalidFormula(e))?;
                Ok(Statement::Donc(f))
            }
            None => match input.strip_prefix("Supposons ") {
                Some(input) => {
                    let f = Formula::read(input).map_err(|e| RecordError::InvalidFormula(e))?;
                    Ok(Statement::Supposons(f))
                },
                None => {
                    let f = Formula::read(input).map_err(|e| RecordError::InvalidFormula(e))?;
                    Ok(Statement::Simple(f))
                }
            }
            
        }
    }

    /// Reads the justification
    fn read_justif(input: &str) -> Result<Jusitification, RecordError> {
        Jusitification::read(input).map_err(|err| RecordError::InvalidJustif(err))
    }


}

#[derive(Debug)]
pub enum Statement {
    Supposons(Formula),
    Donc(Formula),
    Simple(Formula),
}

impl Statement {
    pub fn get_formula(&self) -> &Formula {
        match self {
            Statement::Supposons(formula) => formula,
            Statement::Donc(formula) => formula,
            Statement::Simple(formula) => formula,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_simple_stmt() {
        let input = "1;2,3;A;EImpl 3 4";
        let r = Record::read_record(input);
        assert!(r.is_ok());
    }

    #[test]
    fn simple_supposons_stmt() {
        let input = "1;2,3;Supposons A;EImpl 3 4";
        let r = Record::read_record(input);
        assert!(r.is_ok());
    }

    #[test]
    fn simple_donc_stmt() {
        let input = "1;2,3;Donc A;EImpl 3 4";
        let r = Record::read_record(input);
        assert!(r.is_ok());
    }

    #[test]
    fn too_much_to_read_v1() {
        let input = "1;2,3;Donc A;EImpl 3 4 ddddddd";
        let r = Record::read_record(input);
        assert!(r.is_err())
    }

    #[test]
    fn too_much_to_read_v2() {
        let input = "1;2,3;Donc A;EImpl 3 4;ddddddd";
        let r = Record::read_record(input);
        assert!(r.is_err())
    }
}