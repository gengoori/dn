use crate::{
    formula::Formula,
    justif::Jusitification,
    record::{Record, Statement},
};

#[derive(Debug)]
pub struct ReadError {
    stmt: usize,
    content: crate::record::RecordError,
}

#[derive(Debug)]
pub enum SemanticError {
    /// Internal error. Shouldn't happen
    InternalError,
    /// Id should match the record's position
    IncorrectId,
    /// A Supposons should have the same amount of context as the previous
    /// statement, plus one term.
    SuppCtxtOneMoreThenBefore,
    /// A Supposons should share the same context as the previous statement,
    /// except for the last term.
    SuppCtxtSameAsBefore,
    /// The last term of the context of a Supposons should be the id of the
    /// record.
    SuppCtxtLastIsId,
    /// A Supposons has to justified by a Hyp
    SuppJustIsHyp,

    /// Cannot start with a Donc statement
    DoncNotFirst,
    /// Donc context have one term less than the previous statement.
    DoncCtxtOneLessThenBefore,
    /// Donc context should be the same as that of the previous statement.
    DoncCtxtSameAsBefore,
    /// Donc hypothesis has to be a different statement than the consequence.
    DoncHypDifCons,
    /// Donc hypothesis has to match the linked hypothesis
    DoncHypNotMatching,
    /// Donc consequence has to match the linked consequence
    DoncConsNotMatching,
    /// Donc formula should be an implies.
    DoncFormulaIsImplies,
    /// Donc should have implication introduction justification
    DoncJustifIsIImpl,

    /// Simple shouldn't start a proof
    SimpleIsFirst,
    /// Simple context should be the same as the statement before
    SimpleCtxtSameAsBefore,

    /// IOrL right reference should be lesser than id
    IOrLPosLesser,
    /// IOrL right statement should be compatible
    IOrLIncompatibleCtxt,
    /// IOrL left formula shoud match
    IOrLLeftNotMatching,
    /// IOrL right formula shoud match
    IOrLRightNotMatching,
    /// IOrL formula should be an Or
    IOrLFormulaIsOr,

    /// IOrR right reference should be lesser than id
    IOrRPosLesser,
    /// IOrR right statement should be compatible
    IOrRIncompatibleCtxt,
    /// IOrR left formula shoud match
    IOrRLeftNotMatching,
    /// IOrR right formula shoud match
    IOrRRightNotMatching,
    /// IOrR formula should be an Or
    IOrRFormulaIsOr,

    /// EOr a->c reference should be lesser than id
    EOrA2CPosLesser,
    /// EOr b->c reference should be lesser than id
    EOrB2CPosLesser,
    /// EOr avb reference should be lesser than id
    EOrAOBPosLesser,
    /// EOr a->c statement should be compatible
    EOrA2CIncompatibleCtxt,
    /// EOr b->c statement should be compatible
    EOrB2CIncompatibleCtxt,
    /// EOr a->c statement should be compatible
    EOrAOBIncompatibleCtxt,
    /// EOr left formula shoud match
    EOrLeftNotMatching,
    /// EOr right formula shoud match
    EOrRightNotMatching,
    /// EOr formula should be an Or
    EOrFormulaIsOr,
    /// EOr a formula should be the same in avb and a->c
    EOrAFormulaNotMatching,
    /// EOr b formula should be the same in avb and b->c
    EOrBFormulaNotMatching,
    /// EOr c formula should be the same in b->c and a->c
    EOrCFormulaNotMatchingConsequences,
    /// EOr c formula should be the same in the eliminated conclusion
    EOrCFormulaNotMatchingEliminated,
    /// EOr formulas should be of the form *->*, *->* and *v*
    EOrFormulasNotRightKind,

    /// IAnd left reference should be lesser than id
    IAndLeftPosLesser,
    /// IAnd right reference should be lesser than id
    IAndRightPosLesser,
    /// IAnd left statement should be compatible
    IAndLeftIncompatibleCtxt,
    /// IAnd right statement should be compatible
    IAndRightIncompatibleCtxt,
    /// IAnd left formula shoud match
    IAndLeftNotMatching,
    /// IAnd right formula shoud match
    IAndRightNotMatching,
    /// IAnd formula should be an And
    IAndFormulaIsAnd,

    /// EAnd reference should be lesser than id
    EAndPosLesser,
    /// EAnd statement should be compatible
    EAndIncompatibleCtxt,
    /// EAnd formula shoud match
    EAndNotMatching,
    /// EAndF formula should be an And
    EAndFormulaIsAnd,

    /// Hyp may not justify a simple statement
    HypNotSimple,
    /// IImpl may not justify a simple statement
    IImplNotSimple,

    /// EImpl hyp reference should be lesser than id
    EImplHypPosLesser,
    /// EImpl impl reference should be lesser than id
    EImplImplPosLesser,
    /// EImpl hyp statement should be compatible
    EImplHypIncompatibleCtxt,
    /// EImpl impl statement should be compatible
    EImplImplIncompatibleCtxt,
    /// EImpl hyp formula shoud match
    EImplHypNotMatching,
    /// EImpl impl formula shoud match
    EImplImplNotMatching,
    /// EImpl formula should be an Impl
    EImplFormulaIsImpl,

    /// Efq reference should be lesser than id
    EfqPosLesser,
    /// Efq statement should be compatible
    EfqIncompatibleCtxt,
    /// Efq formula should be a Bot
    EfqFormulaIsBot,

    /// Raa reference should be lesser than id
    RaaPosLesser,
    /// Raa statement should be compatible
    RaaIncompatibleCtxt,
    /// Raa formula should be a NotNot
    RaaFormulaIsNotNot,
    /// Raa formula shoud match
    RaaNotMatching,
}

#[derive(Debug)]
pub enum CheckUpResult {
    NotChecked,
    Valid,
    ValidUntil(usize),
    SemanticErrors {
        first_error: usize,
        errors: Vec<(usize, SemanticError)>,
    },
}

pub struct Proof {
    records: Vec<Record>,
    valid: CheckUpResult,
}

impl Proof {
    /// Reads a proof from a string.
    pub fn read_proof(input: &str) -> Result<Self, ReadError> {
        let mut records: Vec<Record> = Vec::new();
        for (stmt_no, record) in input.split('\n').enumerate() {
            match Record::read_record(record) {
                Ok(r) => records.push(r),
                Err(e) => {
                    return Err(ReadError {
                        stmt: stmt_no,
                        content: e,
                    })
                }
            }
        }
        Ok(Self {
            records,
            valid: CheckUpResult::NotChecked,
        })
    }

    /// Reads a record from input and adds it to the proof.
    pub fn import_record(&mut self, input: &str) -> Result<(), ReadError> {
        match Record::read_record(input) {
            Ok(r) => {
                self.add_record(r);
                Ok(())
            }
            Err(e) => Err(ReadError {
                stmt: 0,
                content: e,
            }),
        }
    }

    /// Adds a record to the proof
    pub fn add_record(&mut self, record: Record) {
        if let CheckUpResult::Valid = self.valid {
            self.valid=CheckUpResult::ValidUntil(self.records.len())
        }
        self.records.push(record);
    }

    /// Checks the proof for record 0..=id. Returns Err if the provided id
    /// is invalid.
    pub fn check_up_to(&mut self, id: usize) -> Result<(),()> {
        if id >= self.records.len() {
            return Err(());
        } else {
            let mut erred = false;
            let mut until: usize = 0;
            let mut errors: Vec<(usize, SemanticError)> = Vec::new();
            for id in 0..=id {
                match self.check_single_record(id) {
                    Ok(()) => (),
                    Err(e) => {
                        if !erred {
                            until = id;
                            erred = true;
                        };
                        errors.push((id, e))
                    }
                }
            }

            self.valid = if erred {
                CheckUpResult::SemanticErrors { first_error: until, errors }
            } else if id + 1 == self.records.len() {
                CheckUpResult::Valid
            } else {
                CheckUpResult::ValidUntil(id+1)
            };
            return Ok(());
        }
    }

    /// Checks the proof for record 0..=id. Returns Err if the provided id
    /// is invalid.
    pub fn check(&mut self) -> Result<(),()> {
        if ! self.records.is_empty() {
            self.check_up_to(self.records.len()-1)
        } else {
            Ok(())
        }
    }

    pub fn state(&self) -> &CheckUpResult{
        &self.valid
    }

    fn check_single_record(&self, id: usize) -> Result<(), SemanticError> {
        let rec = match self.records.get(id) {
            Some(v) => v,
            None => return Err(SemanticError::InternalError),
        };
        // We check the id is the right one.
        if rec.id != id {
            return Err(SemanticError::IncorrectId);
        }
        // We check statement and justification correspond to each other.
        match &rec.stmt {
            Statement::Supposons(_) => {
                // Check context
                if id != 0 {
                    let ante = &self.records[id - 1];
                    if rec.ctxt.len() != ante.ctxt.len() + 1 {
                        return Err(SemanticError::SuppCtxtOneMoreThenBefore);
                    } else if rec.ctxt[0..rec.ctxt.len() - 1] != ante.ctxt {
                        return Err(SemanticError::SuppCtxtSameAsBefore);
                    } else if rec.ctxt[rec.ctxt.len() - 1] != rec.id {
                        return Err(SemanticError::SuppCtxtLastIsId);
                    }
                } else {
                    if rec.ctxt.len() != 1 {
                        return Err(SemanticError::SuppCtxtOneMoreThenBefore);
                    }
                    if rec.ctxt[0] != rec.id {
                        return Err(SemanticError::SuppCtxtLastIsId);
                    }
                };
                match rec.justif {
                    Jusitification::Hyp => Ok(()),
                    _ => Err(SemanticError::SuppJustIsHyp),
                }
            }
            Statement::Donc(conclusion) => {
                match rec.justif {
                    Jusitification::IImpl => {
                        if id == 0 {
                            return Err(SemanticError::DoncNotFirst);
                        }
                        let cons = &self.records[id - 1];
                        if cons.ctxt.len() != rec.ctxt.len() + 1 {
                            return Err(SemanticError::DoncCtxtOneLessThenBefore);
                        }
                        if cons.ctxt[0..rec.ctxt.len()] != rec.ctxt {
                            return Err(SemanticError::DoncCtxtSameAsBefore);
                        }

                        let hyp_pos = cons.ctxt[cons.ctxt.len() - 1];

                        // We ensure the hypothesis was formulated before the current record,
                        // furthermore we want the consequence record to be distinct from the
                        // record.
                        if hyp_pos + 2 > id {
                            return Err(SemanticError::DoncHypDifCons);
                        }
                        let hyp = &self.records[hyp_pos];
                        match conclusion {
                            Formula::Implies(hyp_formula, cons_formula) => {
                                if hyp_formula.as_ref() != hyp.stmt.get_formula() {
                                    Err(SemanticError::DoncHypNotMatching)
                                } else if cons_formula.as_ref() != cons.stmt.get_formula() {
                                    Err(SemanticError::DoncConsNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Err(SemanticError::DoncFormulaIsImplies),
                        }
                    }
                    _ => Err(SemanticError::DoncJustifIsIImpl),
                }
            }
            Statement::Simple(formula) => {
                if id == 0 {
                    return Err(SemanticError::SimpleIsFirst);
                }
                if rec.ctxt != self.records[id - 1].ctxt {
                    return Err(SemanticError::SimpleCtxtSameAsBefore);
                }
                match &rec.justif {
                    Jusitification::IOrL(right_pos, new_left_formula) => {
                        if *right_pos >= id {
                            return Err(SemanticError::IOrLPosLesser);
                        }
                        // This is valid record, since it comes before and was therefore
                        // valited at a previous run of this loop.
                        let right_rec = &self.records[*right_pos];
                        // We check if the record is usable
                        if !check_ctxt_compatibility(&rec.ctxt, &right_rec.ctxt) {
                            return Err(SemanticError::IOrLIncompatibleCtxt);
                        }
                        match formula {
                            Formula::Or(left_formula, right_formula) => {
                                if **left_formula != *new_left_formula {
                                    Err(SemanticError::IOrLLeftNotMatching)
                                } else if **right_formula != *right_rec.stmt.get_formula() {
                                    Err(SemanticError::IOrLRightNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => return Err(SemanticError::IOrLFormulaIsOr),
                        }
                    }
                    Jusitification::IOrR(left_pos, new_right_formula) => {
                        if *left_pos >= id {
                            return Err(SemanticError::IOrRPosLesser);
                        }
                        // This is valid record, since it comes before and was therefore
                        // valited at a previous run of this loop.
                        let left_rec = &self.records[*left_pos];
                        // We check if the record is usable
                        if !check_ctxt_compatibility(&rec.ctxt, &left_rec.ctxt) {
                            return Err(SemanticError::IOrRIncompatibleCtxt);
                        }
                        match formula {
                            Formula::Or(left_formula, right_formula) => {
                                if **right_formula != *new_right_formula {
                                    Err(SemanticError::IOrRRightNotMatching)
                                } else if **left_formula != *left_rec.stmt.get_formula() {
                                    Err(SemanticError::IOrRRightNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Err(SemanticError::IOrRFormulaIsOr),
                        }
                    }
                    Jusitification::EOr {
                        a_to_c,
                        b_to_c,
                        a_or_b,
                    } => {
                        // Checking ids
                        if *a_to_c >= id {
                            return Err(SemanticError::EOrA2CPosLesser);
                        }
                        if *b_to_c >= id {
                            return Err(SemanticError::EOrB2CPosLesser);
                        }
                        if *a_or_b >= id {
                            return Err(SemanticError::EOrAOBPosLesser);
                        }
                        // Since we checked the ids its ok to retrieve the corresponding records.
                        let a_to_c = &self.records[*a_to_c];
                        let b_to_c = &self.records[*b_to_c];
                        let a_or_b = &self.records[*a_or_b];
                        // Checking usability of those records
                        if !check_ctxt_compatibility(&rec.ctxt, &a_to_c.ctxt) {
                            return Err(SemanticError::EOrA2CIncompatibleCtxt);
                        }
                        if !check_ctxt_compatibility(&rec.ctxt, &b_to_c.ctxt) {
                            return Err(SemanticError::EOrB2CIncompatibleCtxt);
                        }
                        if !check_ctxt_compatibility(&rec.ctxt, &a_or_b.ctxt) {
                            return Err(SemanticError::EOrAOBIncompatibleCtxt);
                        }
                        // Checking the constructed formula is correct
                        if let (
                            Formula::Implies(atc_a, atc_c),
                            Formula::Implies(btc_b, btc_c),
                            Formula::Or(aob_a, aob_b),
                        ) = (
                            a_to_c.stmt.get_formula(),
                            b_to_c.stmt.get_formula(),
                            a_or_b.stmt.get_formula(),
                        ) {
                            if aob_a != atc_a {
                                Err(SemanticError::EOrAFormulaNotMatching)
                            } else if aob_b != btc_b {
                                Err(SemanticError::EOrBFormulaNotMatching)
                            } else if atc_c != btc_c {
                                Err(SemanticError::EOrCFormulaNotMatchingConsequences)
                            } else if atc_c.as_ref() != formula {
                                Err(SemanticError::EOrCFormulaNotMatchingEliminated)
                            } else {
                                Ok(())
                            }
                        } else {
                            Err(SemanticError::EOrFormulasNotRightKind)
                        }
                    }
                    Jusitification::IAnd { left, right } => {
                        // Checking ids
                        if *left >= id {
                            return Err(SemanticError::IAndLeftPosLesser);
                        }
                        if *right >= id {
                            return Err(SemanticError::IAndRightPosLesser);
                        }
                        // Since we checked the ids its ok to retrieve the corresponding records.
                        let left = &self.records[*left];
                        let right = &self.records[*right];
                        // Checking usability of left and right
                        if !check_ctxt_compatibility(&rec.ctxt, &left.ctxt) {
                            return Err(SemanticError::IAndLeftIncompatibleCtxt);
                        }
                        if !check_ctxt_compatibility(&rec.ctxt, &right.ctxt) {
                            return Err(SemanticError::IAndRightIncompatibleCtxt);
                        }

                        // Checking the constructed formula is correct
                        match formula {
                            Formula::And(left_formula, right_formula) => {
                                if **left_formula != *left.stmt.get_formula() {
                                    Err(SemanticError::IAndLeftNotMatching)
                                } else if **right_formula != *right.stmt.get_formula() {
                                    Err(SemanticError::IAndRightNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Err(SemanticError::IAndFormulaIsAnd),
                        }
                    }
                    Jusitification::EAndL(and_pos) => {
                        if *and_pos >= id {
                            return Err(SemanticError::EAndPosLesser);
                        }
                        let and = &self.records[*and_pos];
                        if !check_ctxt_compatibility(&rec.ctxt, &and.ctxt) {
                            return Err(SemanticError::EAndIncompatibleCtxt);
                        }
                        match and.stmt.get_formula() {
                            Formula::And(left_formula, _) => {
                                if **left_formula != *formula {
                                    Err(SemanticError::EAndNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Err(SemanticError::EAndFormulaIsAnd),
                        }
                    }
                    Jusitification::EAndR(and_pos) => {
                        if *and_pos >= id {
                            return Err(SemanticError::EAndPosLesser);
                        }
                        let and = &self.records[*and_pos];
                        if !check_ctxt_compatibility(&rec.ctxt, &and.ctxt) {
                            return Err(SemanticError::EAndIncompatibleCtxt);
                        }
                        match and.stmt.get_formula() {
                            Formula::And(_, right_formula) => {
                                if **right_formula != *formula {
                                    Err(SemanticError::EAndNotMatching)
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Err(SemanticError::EAndFormulaIsAnd),
                        }
                    }
                    Jusitification::Hyp => Err(SemanticError::HypNotSimple),
                    Jusitification::IImpl => Err(SemanticError::IImplNotSimple),
                    Jusitification::EImpl { hyp, implication } => {
                        if *hyp >= id {
                            return Err(SemanticError::EImplHypPosLesser);
                        }
                        if *implication >= id {
                            return Err(SemanticError::EImplImplPosLesser);
                        }
                        let hyp = &self.records[*hyp];
                        let implication = &self.records[*implication];
                        if !check_ctxt_compatibility(&rec.ctxt, &hyp.ctxt) {
                            return Err(SemanticError::EImplHypIncompatibleCtxt);
                        }
                        if !check_ctxt_compatibility(&rec.ctxt, &implication.ctxt) {
                            return Err(SemanticError::EImplImplIncompatibleCtxt);
                        }
                        if let Formula::Implies(i_hyp, i_cons) = implication.stmt.get_formula() {
                            if **i_hyp != *hyp.stmt.get_formula() {
                                Err(SemanticError::EImplHypNotMatching)
                            } else if **i_cons != *formula {
                                Err(SemanticError::EImplImplNotMatching)
                            } else {
                                Ok(())
                            }
                        } else {
                            Err(SemanticError::EImplFormulaIsImpl)
                        }
                    }
                    Jusitification::Efq(bot_pos) => {
                        if *bot_pos >= id {
                            return Err(SemanticError::EfqPosLesser);
                        }
                        let bot = &self.records[*bot_pos];
                        if !check_ctxt_compatibility(&rec.ctxt, &bot.ctxt) {
                            return Err(SemanticError::EfqIncompatibleCtxt);
                        }
                        if !matches!(bot.stmt.get_formula(), Formula::Bottom) {
                            return Err(SemanticError::EfqFormulaIsBot);
                        }
                        Ok(())
                    }
                    Jusitification::Raa(nn_pos) => {
                        if *nn_pos >= id {
                            return Err(SemanticError::RaaPosLesser);
                        }
                        let nn = &self.records[*nn_pos];
                        if !check_ctxt_compatibility(&rec.ctxt, &nn.ctxt) {
                            return Err(SemanticError::RaaIncompatibleCtxt);
                        }
                        if let Formula::Not(n_formula) = nn.stmt.get_formula() {
                            if let Formula::Not(nn_formula) = &**n_formula {
                                if **nn_formula != *formula {
                                    Err(SemanticError::RaaNotMatching)
                                } else {
                                    Ok(())
                                }
                            } else {
                                Err(SemanticError::RaaFormulaIsNotNot)
                            }
                        } else {
                            Err(SemanticError::RaaFormulaIsNotNot)
                        }
                    }
                }
            }
        }
    }
 
}

fn check_ctxt_compatibility(current: &[usize], compatible: &[usize]) -> bool {
    if current.len() > compatible.len() {
        false
    } else {
        current == &compatible[0..current.len()]
    }
}
