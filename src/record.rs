use crate::formula::Formula;
use crate::justif::Jusitification;

pub struct Record {
    nb: usize,
    ctxt: Vec<usize>,
    stmt: Statement,
    justif: Jusitification,
}

pub enum Statement {
    Supposons(Formula),
    Donc(Formula),
    Simple(Formula),
}