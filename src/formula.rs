#[derive(Debug)]
pub enum Formula {
    Top,
    Bottom,
    Variable(char),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    RLImplies(Box<Formula>, Box<Formula>),
    Equiv(Box<Formula>, Box<Formula>),
}

enum Lexemes {
    Top,
    Bottom,
    Variable(char),
    Not,
    Or,
    And,
    Implies,
    RLImplies,
    Equiv,
    OpeningParenthesis,
    ClosingParenthesis,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operators {
    Not,
    Or,
    And,
    Implies,
    RLImplies,
    Equiv,
}

#[derive(Debug, PartialEq)]
enum ParseStackItem {
    Operator(Operators),
    Parenthesis,
}

#[derive(Debug)]
enum ParseState {
    BeginingANewGroup,
    Normal,
}

enum Priority {
    Less,
    More,
}

impl Operators {
    /// # Priorities
    /// ¬,∧,∨,⇒,⇐,⇔
    fn cmp(left: &Self, right: &Self) -> Priority {
        match (left, right) {
            (Operators::Not, Operators::Not) => Priority::Less,
            (Operators::Not, _) => Priority::More,
            (Operators::And, Operators::Not) => Priority::Less,
            (Operators::And, _) => Priority::More, // And is left-associative
            (Operators::Or, Operators::Not | Operators::And) => Priority::Less,
            (Operators::Or, _) => Priority::More, // Or is left-associative
            (
                Operators::Implies,
                Operators::Not | Operators::And | Operators::Or | Operators::Implies,
            ) => Priority::Less, // Implies is right-associative
            (Operators::Implies, _) => Priority::More,
            (
                Operators::RLImplies,
                Operators::Not | Operators::And | Operators::Or | Operators::Implies,
            ) => Priority::Less,
            (Operators::RLImplies, _) => Priority::More, // RLImplies is left-associative
            (Operators::Equiv, Operators::Equiv) => Priority::More, // Equiv is left-associative
            (Operators::Equiv, _) => Priority::Less,
        }
    }

    fn is_unary(&self) -> bool {
        match self {
            Self::Not => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum TokenizationError {
    InvalidCharacter(usize, char),
    UnmatchedClosingParenthesis,
    UnmatchedOpeningParenthesis,
    InputIsTooLong,
    AFormulaIsMissing,
    TooManyFormulas,
    InternalError,
    EmptyParenthesis,
    InvalidSubFormula,
}

#[derive(Debug, PartialEq)]
pub enum Regime {
    Stop,
    Eat,
    ForceEat,
    EndEat,
}

enum ZOT {
    Zero,
    One(Operators),
    Two(Operators, Operators),
}

impl Formula {
    fn tokenize(input: &str) -> Result<Vec<Lexemes>, TokenizationError> {
        let mut iter = input.chars().enumerate();
        let mut flow = Vec::new();
        while let Some((pos, c)) = iter.next() {
            flow.push(match c {
                '⊤' => Lexemes::Top,
                '⊥' => Lexemes::Bottom,
                'a'..='z' | 'A'..='Z' => Lexemes::Variable(c),
                '¬' => Lexemes::Not,
                '∨' => Lexemes::Or,
                '∧' => Lexemes::And,
                '⇒' => Lexemes::Implies,
                '⇐' => Lexemes::RLImplies,
                '⇔' => Lexemes::Equiv,
                '(' => Lexemes::OpeningParenthesis,
                ')' => Lexemes::ClosingParenthesis,
                _ => return Err(TokenizationError::InvalidCharacter(pos, c)),
            })
        }
        Ok(flow)
    }
    /// Reads a formula from a string
    pub fn read(input: &str) -> Result<Self, TokenizationError> {
        let lexemes: Vec<Lexemes> = Self::tokenize(input)?;
        let mut stack = Vec::new();
        let mut formulas: Vec<Formula> = Vec::new();
        let mut regime = Regime::Stop;
        let mut state = ParseState::BeginingANewGroup;
        eprintln!("Tokenization done !");
        for l in lexemes {
            eprintln!("Got a new lexeme");
            Self::read_lexeme(l, &mut regime, &mut state, &mut stack, &mut formulas)?;
            Self::eat(&mut stack, &mut state, &mut regime, &mut formulas)?;
        }
        regime = Regime::EndEat;
        Self::eat(&mut stack, &mut state, &mut regime, &mut formulas)?;
        return Ok(formulas.pop().unwrap());
    }

    fn read_lexeme(
        l: Lexemes,
        regime: &mut Regime,
        state: &mut ParseState,
        stack: &mut Vec<ParseStackItem>,
        formulas: &mut Vec<Formula>,
    ) -> Result<(), TokenizationError> {
        assert_eq!(Regime::Stop, *regime);
        match l {
            Lexemes::OpeningParenthesis => {
                *state = ParseState::BeginingANewGroup;
                stack.push(ParseStackItem::Parenthesis)
            }
            Lexemes::Variable(v) => {
                formulas.push(Formula::Variable(v));
                *regime = Regime::Eat;
            }
            Lexemes::Top => {
                formulas.push(Formula::Top);
                *regime = Regime::Eat;
            }
            Lexemes::Bottom => {
                formulas.push(Formula::Bottom);
                *regime = Regime::Eat;
            }
            Lexemes::ClosingParenthesis => {
                *regime = Regime::ForceEat;
            }
            Lexemes::Or => {
                stack.push(ParseStackItem::Operator(Operators::Or));
            }
            Lexemes::And => {
                stack.push(ParseStackItem::Operator(Operators::And));
            }
            Lexemes::Not => {
                stack.push(ParseStackItem::Operator(Operators::Not));
            }
            Lexemes::Implies => {
                stack.push(ParseStackItem::Operator(Operators::Implies));
            }
            Lexemes::RLImplies => {
                stack.push(ParseStackItem::Operator(Operators::RLImplies));
            }
            Lexemes::Equiv => {
                stack.push(ParseStackItem::Operator(Operators::Equiv));
            }
        };
        Ok(())
    }

    fn get_last_two_ops_from_stack(stack: &mut Vec<ParseStackItem>) -> ZOT {
        match stack.split_last() {
            None => ZOT::Zero,
            Some((right, tail)) => match tail.split_last() {
                None => match right {
                    ParseStackItem::Parenthesis => ZOT::Zero,
                    ParseStackItem::Operator(o) => ZOT::One(*o),
                },
                Some((left, _)) => match (left, right) {
                    (_, ParseStackItem::Parenthesis) => ZOT::Zero,
                    (ParseStackItem::Parenthesis, ParseStackItem::Operator(o)) => ZOT::One(*o),
                    (ParseStackItem::Operator(left), ParseStackItem::Operator(right)) => {
                        ZOT::Two(*left, *right)
                    }
                },
            },
        }
    }

    fn eat(
        stack: &mut Vec<ParseStackItem>,
        state: &mut ParseState,
        regime: &mut Regime,
        formulas: &mut Vec<Formula>,
    ) -> Result<(), TokenizationError> {
        loop {
            eprintln!("Regime : {:?}", regime);
            eprintln!("State : {:?}", state);
            eprintln!("Stack :");
            for i in stack.iter() {
                eprintln!("   {:?}", i)
            }
            eprintln!("Formula :");
            for i in formulas.iter() {
                eprintln!("   {:?}", i)
            }
            match regime {
                Regime::Stop => break,
                Regime::Eat => match Self::get_last_two_ops_from_stack(stack) {
                    ZOT::Zero => match state {
                        ParseState::BeginingANewGroup => {
                            *regime = Regime::Stop;
                            *state = ParseState::Normal;
                        }
                        ParseState::Normal => {
                            return Err(TokenizationError::TooManyFormulas);
                        }
                    },
                    ZOT::One(o) => match state {
                        ParseState::BeginingANewGroup => {
                            if o.is_unary() {
                                *state = ParseState::Normal;
                                *regime = Regime::Stop;
                            } else {
                                return Err(TokenizationError::TooManyFormulas);
                            }
                        }
                        ParseState::Normal => {
                            *regime = Regime::Stop;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InternalError)
                        }
                        ParseState::Normal => match Operators::cmp(&left, &right) {
                            Priority::Less => *regime = Regime::Stop,
                            Priority::More => {
                                let len: usize = stack.len();
                                stack.swap(len - 2, len - 1);
                                assert_eq!(Some(ParseStackItem::Operator(left)), stack.pop());
                                let rightest_formula = formulas.pop().unwrap();
                                Self::build_from_operator(left, formulas)?;
                                formulas.push(rightest_formula);
                            }
                        },
                    },
                },
                Regime::ForceEat => match Self::get_last_two_ops_from_stack(stack) {
                    ZOT::Zero => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::EmptyParenthesis)
                        }
                        ParseState::Normal => {
                            assert_eq!(stack.pop(), Some(ParseStackItem::Parenthesis));
                            *regime = Regime::Eat;
                        }
                    },
                    ZOT::One(op) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InvalidSubFormula)
                        }
                        ParseState::Normal => {
                            assert_eq!(Some(ParseStackItem::Operator(op)), stack.pop());
                            assert_eq!(Some(ParseStackItem::Parenthesis), stack.pop());
                            Self::build_from_operator(op, formulas)?;
                            *regime = Regime::Eat;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InternalError)
                        }
                        ParseState::Normal => match Operators::cmp(&left, &right) {
                            Priority::Less => {
                                assert_eq!(Some(ParseStackItem::Operator(right)), stack.pop());
                                Self::build_from_operator(right, formulas)?;
                                *regime = Regime::Eat;
                            }
                            Priority::More => {
                                let len: usize = stack.len();
                                stack.swap(len - 2, len - 1);
                                assert_eq!(Some(ParseStackItem::Operator(left)), stack.pop());
                                let rightest_formula = formulas.pop().unwrap();
                                Self::build_from_operator(left, formulas)?;
                                formulas.push(rightest_formula);
                                *regime = Regime::Eat;
                            }
                        },
                    },
                },
                Regime::EndEat => match Self::get_last_two_ops_from_stack(stack) {
                    ZOT::Zero => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::EmptyParenthesis)
                        }
                        ParseState::Normal => {
                            *regime = Regime::Stop;
                        }
                    },
                    ZOT::One(op) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InvalidSubFormula)
                        }
                        ParseState::Normal => {
                            assert_eq!(Some(ParseStackItem::Operator(op)), stack.pop());
                            Self::build_from_operator(op, formulas)?;
                            *regime = Regime::EndEat;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InternalError)
                        }
                        ParseState::Normal => match Operators::cmp(&left, &right) {
                            Priority::Less => {
                                assert_eq!(Some(ParseStackItem::Operator(right)), stack.pop());
                                Self::build_from_operator(right, formulas)?;
                                *regime = Regime::EndEat;
                            }
                            Priority::More => {
                                let len: usize = stack.len();
                                stack.swap(len - 2, len - 1);
                                assert_eq!(Some(ParseStackItem::Operator(left)), stack.pop());
                                let rightest_formula = formulas.pop().unwrap();
                                Self::build_from_operator(left, formulas)?;
                                formulas.push(rightest_formula);
                                *regime = Regime::EndEat;
                            }
                        },
                    },
                },
            }
        }
        Ok(())
    }
    /// Transform `formulas` according to `operator` and the *last* one/two formulae in the stack.
    fn build_from_operator(
        operator: Operators,
        formulas: &mut Vec<Formula>,
    ) -> Result<(), TokenizationError> {
        match operator {
            Operators::Not => {
                let f = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Not(Box::new(f)));
            }
            Operators::And => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::And(Box::new(left), Box::new(right)));
            }
            Operators::Or => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Or(Box::new(left), Box::new(right)));
            }
            Operators::Implies => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Implies(Box::new(left), Box::new(right)));
            }
            Operators::RLImplies => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::RLImplies(Box::new(left), Box::new(right)));
            }
            Operators::Equiv => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Equiv(Box::new(left), Box::new(right)));
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_formula() {
        // ¬,∧,∨,⇒,⇐,⇔
        let input = "c∧(a∨b)";
        dbg!(Formula::read(input).unwrap());
    }
}
