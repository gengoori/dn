pub enum Formula {
    Top,
    Bottom,
    Variable(char),
    Not(Box<Formula>),
    Or(Box<Formula>,Box<Formula>),
    And(Box<Formula>,Box<Formula>),
    Implies(Box<Formula>,Box<Formula>),
    RLImplies(Box<Formula>,Box<Formula>),
    Equiv(Box<Formula>,Box<Formula>),
}