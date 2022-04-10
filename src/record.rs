use crate::formula::Formula;
use crate::justif::Jusitification;

pub enum RecordError {
    /// A field is missing
    MissingField,
    /// The id field is invalid
    InvalidId,
    /// The ctxt field is invalid    
    InvalidCtxt,
    /// There are too many fields
    TooMuch,
}

pub struct Record {
    id: usize,
    ctxt: Vec<usize>,
    stmt: Statement,
    justif: Jusitification,
}

impl Record {
    /// Reads a record
    pub fn read_record(input: &str) -> Result<Self, RecordError> {
        let mut input = input.split(|c: char| c == ';');
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
            .map(|slc: &str| slc.parse::<usize>().map_err(|_| RecordError::InvalidCtxt))
            .collect()
    }

    /// Reads a statement
    fn read_stmt(input: &str) -> Result<Statement, RecordError> {
        todo!()
    }

    /// Reads the justification
    fn read_justif(input: &str) -> Result<Jusitification, RecordError> {
        todo!()
    }
}

pub enum Statement {
    Supposons(Formula),
    Donc(Formula),
    Simple(Formula),
}
