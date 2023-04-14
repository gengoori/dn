#[derive(Clone, Debug, PartialEq)]
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
    Constant,
    Operator,
}

enum Priority {
    Less,
    More,
}

impl Operators {
    /// # Priorities
    /// ¬,∧,∨,⇒,⇐,⇔
    ///
    /// # Contraintes
    ///
    /// Let a be binary operator, u a unary operator and z either a binary or a unary operator.
    /// cmp should verify this:
    /// - cmp(z,u)=Less
    /// - cmp(u,a)=More
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

#[derive(Debug,PartialEq)]
pub enum TokenizationError {
    InvalidCharacter(usize, char),
    UnmatchedClosingParenthesis,
    UnmatchedOpeningParenthesis,
    InputIsTooLong,
    AFormulaIsMissing,
    TooManyFormulas,
    InternalError(usize),
    EmptyParenthesis,
    InvalidSubFormula,
    OperatorWithoutRightHandside,
    BinaryOperatorWithoutRightHandside,
}

#[derive(Debug, PartialEq)]
enum Regime {
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
        *regime = Regime::Eat;
        match l {
            Lexemes::OpeningParenthesis => {
                stack.push(ParseStackItem::Parenthesis);
                *state = ParseState::BeginingANewGroup
            }
            Lexemes::Variable(v) => {
                formulas.push(Formula::Variable(v));
                *state = ParseState::Constant
            }
            Lexemes::Top => {
                formulas.push(Formula::Top);
                *regime = Regime::Eat;
                *state = ParseState::Constant
            }
            Lexemes::Bottom => {
                formulas.push(Formula::Bottom);
                *state = ParseState::Constant
            }
            Lexemes::ClosingParenthesis => {
                *regime = Regime::ForceEat;
                // L'état ne change pas
            }
            Lexemes::Or => {
                stack.push(ParseStackItem::Operator(Operators::Or));
                *state = ParseState::Operator
            }
            Lexemes::And => {
                stack.push(ParseStackItem::Operator(Operators::And));
                *state = ParseState::Operator
            }
            Lexemes::Not => {
                stack.push(ParseStackItem::Operator(Operators::Not));
                *state = ParseState::Operator
            }
            Lexemes::Implies => {
                stack.push(ParseStackItem::Operator(Operators::Implies));
                *state = ParseState::Operator
            }
            Lexemes::RLImplies => {
                stack.push(ParseStackItem::Operator(Operators::RLImplies));
                *state = ParseState::Operator
            }
            Lexemes::Equiv => {
                stack.push(ParseStackItem::Operator(Operators::Equiv));
                *state = ParseState::Operator
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
                        }
                        ParseState::Operator => {
                            return Err(TokenizationError::InternalError(0));
                        }
                        ParseState::Constant => {
                            *regime = Regime::Stop;
                        }
                    },
                    ZOT::One(o) => match state {
                        ParseState::BeginingANewGroup => {
                            if o.is_unary() {
                                *regime = Regime::Stop;
                            } else {
                                return Err(TokenizationError::TooManyFormulas);
                            }
                        }
                        ParseState::Operator => {
                            *regime = Regime::Stop;
                        }
                        ParseState::Constant => {
                            // On est dans la situation ?{?Oa
                            if o.is_unary() {
                                // O s'applique nécessairement à a:
                                debug_assert_eq!(Some(ParseStackItem::Operator(o)), stack.pop());
                                Self::build_from_operator(o, formulas)?;
                            }
                            *regime = Regime::Stop;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            // On est dans la situation {?L?R? avec un seul lexème après la parenthèse...
                            // Sûrement une erreur dans l'assignation de l'état.
                            return Err(TokenizationError::InternalError(1));
                        }
                        ParseState::Constant => match Operators::cmp(&left, &right) {
                            Priority::Less => {
                                // On est dans la situation ?L?Ra. On mange ?Ra.
                                debug_assert_eq!(
                                    Some(ParseStackItem::Operator(right)),
                                    stack.pop()
                                );
                                Self::build_from_operator(right, formulas)?;
                            }
                            Priority::More => {
                                debug_assert!(!right.is_unary());
                                // Comme right n'est pas unaire, on est dans la situation:
                                // ? L a R b
                                // En enlevant b des formules et R de la pile
                                debug_assert_eq!(
                                    Some(ParseStackItem::Operator(right)),
                                    stack.pop()
                                );
                                let rightest =
                                    formulas.pop().ok_or(TokenizationError::InternalError(2))?;
                                // On se retrouve dans la situation ? L a, et on fait:
                                debug_assert_eq!(Some(ParseStackItem::Operator(left)), stack.pop());
                                Self::build_from_operator(left, formulas)?;
                                // On remet l'opérateur de droite et la formule
                                formulas.push(rightest);
                                stack.push(ParseStackItem::Operator(right));
                            }
                        },
                        ParseState::Operator => match Operators::cmp(&left, &right) {
                            Priority::Less => {
                                // Situation: ? L [? R
                                *regime = Regime::Stop;
                            }
                            Priority::More => {
                                if right.is_unary() {
                                    // Situation LR
                                    *regime = Regime::Stop; //TODO: ne devrait pas arriver !
                                } else {
                                    debug_assert!(!right.is_unary());
                                    // Si right n'est pas unaire, on est dans la situation:
                                    // ? L a R
                                    // En enlevant R de la pile
                                    debug_assert_eq!(
                                        Some(ParseStackItem::Operator(right)),
                                        stack.pop()
                                    );
                                    // On se retrouve dans la situation ? L a, et on fait:
                                    debug_assert_eq!(
                                        Some(ParseStackItem::Operator(left)),
                                        stack.pop()
                                    );
                                    Self::build_from_operator(left, formulas)?;
                                    // On remet l'opérateur de droite
                                    stack.push(ParseStackItem::Operator(right));
                                    *regime = Regime::Eat; // Peut-être stop
                                }
                            }
                        },
                    },
                },
                Regime::ForceEat => match Self::get_last_two_ops_from_stack(stack) {
                    ZOT::Zero => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::EmptyParenthesis);
                        }
                        ParseState::Operator => {
                            // On est dans la situation ?(?o) or une opération à forcément une
                            // opérande à gauche.
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => {
                            // On est dans la situation ?(a) et on passe à ?a
                            debug_assert_eq!(Some(ParseStackItem::Parenthesis), stack.pop());
                            *regime = Regime::Eat;
                        }
                    },
                    ZOT::One(op) => match state {
                        ParseState::BeginingANewGroup | ParseState::Operator => {
                            // On est dans le cas ?(o) ou ?(?o) or une opération à forcément une
                            // opérande à gauche.
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => {
                            // On est dans le cas ?oa
                            assert_eq!(Some(ParseStackItem::Operator(op)), stack.pop());
                            assert_eq!(Some(ParseStackItem::Parenthesis), stack.pop());
                            Self::build_from_operator(op, formulas)?;
                            *regime = Regime::Eat;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            // On est dans la situation (?L?R) avec un seul lexème après la parenthèse...
                            // Sûrement une erreur dans l'assignation de l'état.
                            return Err(TokenizationError::InternalError(3));
                        }
                        ParseState::Operator => {
                            // On est dans le cas ?(?L?R) or R à forcément une
                            // opérande à gauche.
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => {
                            match Operators::cmp(&left, &right) {
                                Priority::Less => {
                                    // On est dans la situation ?(?L?Ra). On mange ?Ra.
                                    assert_eq!(Some(ParseStackItem::Operator(right)), stack.pop());
                                    Self::build_from_operator(right, formulas)?;
                                    *regime = Regime::Eat;
                                }
                                Priority::More => {
                                    // right n'est pas unaire
                                    debug_assert!(!right.is_unary());
                                    // Donc on est dans le cas ?(?LaRb)
                                    // En enlevant R de la pile et b des formules
                                    debug_assert_eq!(
                                        Some(ParseStackItem::Operator(right)),
                                        stack.pop()
                                    );
                                    let rightest = formulas
                                        .pop()
                                        .ok_or(TokenizationError::AFormulaIsMissing)?;
                                    // On se retrouve dans la situation ? L a, et on fait:
                                    debug_assert_eq!(
                                        Some(ParseStackItem::Operator(left)),
                                        stack.pop()
                                    );
                                    Self::build_from_operator(left, formulas)?;
                                    // On remet l'opérateur de droite et la formule
                                    stack.push(ParseStackItem::Operator(right));
                                    formulas.push(rightest);
                                    *regime = Regime::ForceEat; // On doit tout manger
                                                                // state ne change pas.
                                }
                            }
                        }
                    },
                },
                Regime::EndEat => match Self::get_last_two_ops_from_stack(stack) {
                    ZOT::Zero => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::EmptyParenthesis)
                        }
                        ParseState::Operator => {
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => {
                            *regime = Regime::Stop;
                        }
                    },
                    ZOT::One(op) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InvalidSubFormula)
                        }
                        ParseState::Operator => {
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => {
                            assert_eq!(Some(ParseStackItem::Operator(op)), stack.pop());
                            Self::build_from_operator(op, formulas)?;
                            *regime = Regime::EndEat;
                        }
                    },
                    ZOT::Two(left, right) => match state {
                        ParseState::BeginingANewGroup => {
                            return Err(TokenizationError::InternalError(4))
                        }
                        ParseState::Operator => {
                            return Err(TokenizationError::OperatorWithoutRightHandside);
                        }
                        ParseState::Constant => match Operators::cmp(&left, &right) {
                            Priority::Less => {
                                assert_eq!(Some(ParseStackItem::Operator(right)), stack.pop());
                                Self::build_from_operator(right, formulas)?;
                                *regime = Regime::EndEat;
                            }
                            Priority::More => {
                                // right n'est pas unaire
                                debug_assert!(!right.is_unary());
                                // Donc on est dans le cas ?LaRb
                                // En enlevant R de la pile et b des formules
                                debug_assert_eq!(
                                    Some(ParseStackItem::Operator(right)),
                                    stack.pop()
                                );
                                let rightest =
                                    formulas.pop().ok_or(TokenizationError::AFormulaIsMissing)?;
                                // On se retrouve dans la situation ?La, et on fait:
                                debug_assert_eq!(Some(ParseStackItem::Operator(left)), stack.pop());
                                Self::build_from_operator(left, formulas)?;
                                // On remet l'opérateur de droite et la formule
                                stack.push(ParseStackItem::Operator(right));
                                formulas.push(rightest);
                                *regime = Regime::EndEat; // On doit tout manger
                                                          // state ne change pas.
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
mod tests;