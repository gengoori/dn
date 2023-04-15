use std::num::ParseIntError;

use thiserror::Error;

use crate::formula::{Formula, TokenizationError};

#[allow(non_camel_case_types)]
#[derive(Error,Debug, PartialEq)]
pub enum ReadError {
    #[error("empty")]
    InputEmpty,
    #[error("unknown rule")]
    UnknownRule,
    #[error("the input is too large")]
    InputTooLarge,
    #[error("IOrL is missing the position of right formula to be inserted")]
    Missing_In_IOrL_LeftPos,
    #[error("IOrL is provided a position of the right formula to be inserted that is not a valid number: {0}")]
    Invalid_In_IOrL_LeftPos(ParseIntError),
    #[error("IOrL is missing the left formula to be inserted")]
    Missing_In_IOrL_Formula,
    #[error("IOrL is provided a left formula to be inserted that is not a valid formula: {0}")]
    Invalid_In_IOrL_Formula(TokenizationError),
    #[error("IOrR is missing the position of the left formula to be inserted")]
    Missing_In_IOrR_RightPos,
    #[error("IOrR is provided a position of the left formula to be inserted that is not a valid number: {0}")]
    Invalid_In_IOrR_RightPos(ParseIntError),
    #[error("IOrR is missing the right formula to be inserted")]
    Missing_In_IOrR_Formula,
    #[error("IOrR is provided a right formula to be inserted that is not a valid formula: {0}")]
    Invalid_In_IOrR_Formula(TokenizationError),
    #[error("EOr is missing the position of the a->c formula")]
    Missing_In_EOr_A_to_C,
    #[error("EOr is provided a position of the a->c formula that is not a valid number: {0}")]
    Invalid_In_EOr_A_to_C(ParseIntError),
    #[error("EOr is missing the position of the b->c formula")]
    Missing_In_EOr_B_to_C,
    #[error("EOr is provided a position of the b->c formula that is not a valid number: {0}")]
    Invalid_In_EOr_B_to_C(ParseIntError),
    #[error("EOr is missing the position of a∨b formula")]
    Missing_In_EOr_A_or_B,
    #[error("EOr is provided a position of the a∨b formula that is not a valid number: {0}")]
    Invalid_In_EOr_A_or_B(ParseIntError),
    #[error("IAnd is missing the position of the left formula to be inserted")]
    Missing_In_IAnd_Left,
    #[error("IAnd is provided a position of the left formula to be inserted that is not a valid number: {0}")]
    Invalid_In_IAnd_Left(ParseIntError),
    #[error("IAnd is missing the position of the right formula to be inserted")]
    Missing_In_IAnd_Right,
    #[error("IAnd is provided a position of the right formula to be inserted that is not a valid number: {0}")]
    Invalid_In_IAnd_Right(ParseIntError),
    #[error("EAndL is missing the position of the formula to be inserted")]
    Missing_In_EAndL_Reference,
    #[error("EAndL is provided a position of the formula that is not a valid number: {0}")]
    Invalid_In_EAndL_Reference(ParseIntError),
    #[error("EAndR is missing the position of the formula")]
    Missing_In_EAndR_Reference,
    #[error("EAndR is provided a position of the formula that is not a valid number: {0}")]
    Invalid_In_EAndR_Reference(ParseIntError),
    #[error("EImpl is missing the position of the hypothesis formula")]
    Missing_In_EImpl_Hyp,
    #[error("EImpl is provided a position of the hypothesis formula that is not a valid number: {0}")]
    Invalid_In_EImpl_Hyp(ParseIntError),
    #[error("EImpl is missing the position of the implication formula")]
    Missing_In_EImpl_Implication,
    #[error("EImpl is provided a position of the implication formula that is not a valid number: {0}")]
    Invalid_In_EImpl_Implication(ParseIntError),
    #[error("Efq is missing the position of the bottom formula")]
    Missing_In_Efq_Reference,
    #[error("Efq is provided a position of the bottom formula that is not a valid number: {0}")]
    Invalid_In_Efq_Reference(ParseIntError),
    #[error("Raa is missing the position of the formula")]
    Missing_In_Raa_Reference,
    #[error("Raa is provided a position of the formula that is not a valid number: {0}")]
    Invalid_In_Raa_Reference(ParseIntError),
    #[error("Rwrt is missing the position of the formula")]
    Missing_In_Rwrt_Reference,
    #[error("Rwrt is provided a position of the formula that is not a valid number: {0}")]
    Invalid_In_Rwrt_Reference(ParseIntError),
}

#[derive(Debug, PartialEq)]
pub enum Jusitification {
    /// Introduction of Or (new formula on left)
    IOrL(usize, Formula),
    /// Introduction of Or (new formula on right)
    IOrR(usize, Formula),
    /// Elimination of Or
    EOr {
        a_to_c: usize,
        b_to_c: usize,
        a_or_b: usize,
    },
    /// Introduction of And
    IAnd { left: usize, right: usize },
    /// Elimination of And (getting left)
    EAndL(usize),
    /// Elimination of And (getting right)
    EAndR(usize),
    /// New hypothesis
    Hyp,
    /// Introduction of Implies
    IImpl,
    /// Elimination of Implies
    EImpl { hyp: usize, implication: usize },
    /// Ex falso quodlibet
    Efq(usize),
    /// Reductio ad absorbum
    Raa(usize),
    /// Rewrite
    Rwrt(usize)
}

impl Jusitification {
    pub fn read(input: &str) -> Result<Self, ReadError> {
        let mut s = input.split(' ');
        let rule = s.next().ok_or(ReadError::InputEmpty)?;
        let r = match rule {
            "IOrL" => {
                let left_pos = s
                    .next()
                    .ok_or(ReadError::Missing_In_IOrL_LeftPos)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_IOrL_LeftPos(err))?;
                let right_formula =
                    Formula::read(s.next().ok_or(ReadError::Missing_In_IOrL_Formula)?)
                        .map_err(|err| ReadError::Invalid_In_IOrL_Formula(err))?;
                Ok(Self::IOrL(left_pos, right_formula))
            }
            "IOrR" => {
                let right_pos = s
                    .next()
                    .ok_or(ReadError::Missing_In_IOrR_RightPos)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_IOrR_RightPos(err))?;
                let left_formula =
                    Formula::read(s.next().ok_or(ReadError::Missing_In_IOrR_Formula)?)
                        .map_err(|err| ReadError::Invalid_In_IOrR_Formula(err))?;
                Ok(Self::IOrR(right_pos, left_formula))
            }
            "EOr" => {
                let a_to_c = s
                    .next()
                    .ok_or(ReadError::Missing_In_EOr_A_to_C)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EOr_A_to_C(err))?;
                let b_to_c = s
                    .next()
                    .ok_or(ReadError::Missing_In_EOr_B_to_C)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EOr_B_to_C(err))?;
                let a_or_b = s
                    .next()
                    .ok_or(ReadError::Missing_In_EOr_A_or_B)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EOr_A_or_B(err))?;
                Ok(Self::EOr {
                    a_to_c,
                    b_to_c,
                    a_or_b,
                })
            }
            "IAnd" => {
                let left = s
                    .next()
                    .ok_or(ReadError::Missing_In_IAnd_Left)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_IAnd_Left(err))?;
                let right = s
                    .next()
                    .ok_or(ReadError::Missing_In_IAnd_Right)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_IAnd_Right(err))?;
                Ok(Self::IAnd { left, right })
            }
            "EAndL" => {
                let reference = s
                    .next()
                    .ok_or(ReadError::Missing_In_EAndL_Reference)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EAndL_Reference(err))?;
                Ok(Self::EAndL(reference))
            }
            "EAndR" => {
                let reference = s
                    .next()
                    .ok_or(ReadError::Missing_In_EAndR_Reference)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EAndR_Reference(err))?;
                Ok(Self::EAndR(reference))
            }
            "Hyp" => Ok(Self::Hyp),
            "IImpl" => Ok(Self::IImpl),
            "EImpl" => {
                let hyp = s
                    .next()
                    .ok_or(ReadError::Missing_In_EImpl_Hyp)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EImpl_Hyp(err))?;
                let implication = s
                    .next()
                    .ok_or(ReadError::Missing_In_EImpl_Implication)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_EImpl_Implication(err))?;
                Ok(Self::EImpl { hyp, implication })
            }
            "Efq" => {
                let reference = s
                    .next()
                    .ok_or(ReadError::Missing_In_Efq_Reference)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_Efq_Reference(err))?;
                Ok(Self::Efq(reference))
            }
            "Raa" => {
                let reference = s
                    .next()
                    .ok_or(ReadError::Missing_In_Raa_Reference)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_Raa_Reference(err))?;
                Ok(Self::Raa(reference))
            }
            "Rwrt" => {
                let reference = s
                    .next()
                    .ok_or(ReadError::Missing_In_Rwrt_Reference)?
                    .parse::<usize>()
                    .map_err(|err| ReadError::Invalid_In_Rwrt_Reference(err))?;
                Ok(Self::Rwrt(reference))
            }
            _ => Err(ReadError::UnknownRule),
        }?;
        match s.next() {
            Some(_) => Err(ReadError::InputTooLarge),
            None => Ok(r),
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn IOrL_legal() {
        let input = "IOrL 1 ¬x";
        let r = Jusitification::read(input);
        assert_eq!(r, Ok(Jusitification::IOrL(1, Formula::Not(Box::new(Formula::Variable('x'))))));
    }

    #[test]
    fn IOrL_alone() {
        let input = "IOrL";
        let r = Jusitification::read(input);
        assert_eq!(r,Err(ReadError::Missing_In_IOrL_LeftPos));
    }


    #[test]
    fn IOrL_invalid_pos() {
        let input = "IOrL -1";
        let r = Jusitification::read(input);
        matches!(r,Err(ReadError::Invalid_In_IOrL_LeftPos(_)));
    }
}
