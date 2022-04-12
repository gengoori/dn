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

#[derive(Clone, Copy)]
enum Operators {
    Not,
    Or,
    And,
    Implies,
    RLImplies,
    Equiv,
}

enum ParseStackItem {
    Operator(Operators),
    Parenthesis,
}

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

pub enum TokenizationError {
    InvalidCharacter(usize, char),
    UnmatchedClosingParenthesis,
    UnmatchedOpeningParenthesis,
    InputIsTooLong,
    AFormulaIsMissing,
    TooManyFormulas,
    InternalError,
}

pub enum Regime {
    Stop,
    Eat,
    ForceEat,
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
        for l in lexemes {
            Self::read_lexeme(l, &mut regime, &mut state, &mut stack, &mut formulas)?;
        }
        todo!()
    }

    fn read_lexeme(
        l: Lexemes,
        regime: &mut Regime,
        state: &mut ParseState,
        stack: &mut Vec<ParseStackItem>,
        formulas: &mut Vec<Formula>,
    ) -> Result<(), TokenizationError> {
        assert!(if let Regime::Stop = *regime {
            true
        } else {
            false
        });
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
                match stack.pop() {
                    Some(ParseStackItem::Parenthesis) => *regime = Regime::ForceEat,
                    _ => return Err(TokenizationError::UnmatchedClosingParenthesis),
                };
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
                        ParseState::Normal => {}
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InternalError)
                        }
                        ParseState::Normal => match Operators::cmp(&left, &right) {
                            Priority::Less => *regime = Regime::Stop,
                            Priority::More => {
                                let len = stack.len();
                                stack.swap(len - 2, len - 1);
                                match stack.pop() {
                                    Some(ParseStackItem::Operator(o)) => {
                                        Self::build_from_operator(o, formulas)?
                                    }
                                    _ => return Err(TokenizationError::InternalError),
                                };
                            }
                        },
                    },
                },
                Regime::ForceEat => match Self::get_last_two_ops_from_stack(stack) {
                    //TODO
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
                formulas.push(Formula::Or(Box::new(left), Box::new(right)));
            }
            Operators::RLImplies => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Or(Box::new(left), Box::new(right)));
            }
            Operators::Equiv => {
                let right = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                let left = formulas
                    .pop()
                    .ok_or_else(|| TokenizationError::AFormulaIsMissing)?;
                formulas.push(Formula::Or(Box::new(left), Box::new(right)));
            }
        };
        Ok(())
    }
}

//// a^b^c
//// 12345
//// 1. formula={a} stack{}
//// 2. formula={a} stack{^}
//// 3. formula={a,b} stack=^
//// 4. formula={a,b} stack=v

use std::str::ParseBoolError;
