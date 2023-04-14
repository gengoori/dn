
use super::*;

#[test]
fn simple_formula() {
    let input = "c∧(a∨b)";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::And(
            Box::new(Formula::Variable('c')),
            Box::new(Formula::Or(
                Box::new(Formula::Variable('a')),
                Box::new(Formula::Variable('b'))
            ))
        ),
        f
    );
}
#[test]
fn complex_formula() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "¬(a)⇒(a∨b)⇒(¬q∨¬r∧s⇔t⇔d)";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Implies(
            Box::new(Formula::Not(Box::new(Formula::Variable('a')))),
            Box::new(Formula::Implies(
                Box::new(Formula::Or(
                    Box::new(Formula::Variable('a')),
                    Box::new(Formula::Variable('b'))
                )),
                Box::new(Formula::Equiv(
                    Box::new(Formula::Equiv(
                        Box::new(Formula::Or(
                            Box::new(Formula::Not(Box::new(Formula::Variable('q')))),
                            Box::new(Formula::And(
                                Box::new(Formula::Not(Box::new(Formula::Variable('r')))),
                                Box::new(Formula::Variable('s'))
                            ))
                        )),
                        Box::new(Formula::Variable('t'))
                    )),
                    Box::new(Formula::Variable('d')),
                ))
            ))
        ),
        f
    );
}

#[test]
fn single_variable() {
    let input = "a";
    let f = Formula::read(input).unwrap();
    assert_eq!(Formula::Variable('a'), f);
}

#[test]
fn simple_or() {
    let input = "b∨c";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Or(
            Box::new(Formula::Variable('b')),
            Box::new(Formula::Variable('c'))
        ),
        f
    );
}

#[test]
fn simple_and() {
    let input = "d∧e";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::And(
            Box::new(Formula::Variable('d')),
            Box::new(Formula::Variable('e'))
        ),
        f
    );
}

#[test]
fn simple_implies() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "f⇒g";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Implies(
            Box::new(Formula::Variable('f')),
            Box::new(Formula::Variable('g'))
        ),
        f
    );
}

#[test]
fn simple_rlimplies() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "h⇐i";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::RLImplies(
            Box::new(Formula::Variable('h')),
            Box::new(Formula::Variable('i'))
        ),
        f
    );
}

#[test]
fn simple_equiv() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "j⇔k";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Equiv(
            Box::new(Formula::Variable('j')),
            Box::new(Formula::Variable('k'))
        ),
        f
    );
}

#[test]
fn simple_not() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "¬l";
    let f = Formula::read(input).unwrap();
    assert_eq!(Formula::Not(Box::new(Formula::Variable('l'))), f);
}

#[test]
fn implies_right_associativity() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "m⇒n⇒p";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Implies(
            Box::new(Formula::Variable('m')),
            Box::new(Formula::Implies(
                Box::new(Formula::Variable('n')),
                Box::new(Formula::Variable('p'))
            ))
        ),
        f
    );
}

#[test]
fn two_ops_left_is_more() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "a∧b∨c";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Or(
            Box::new(Formula::And(
                Box::new(Formula::Variable('a')),
                Box::new(Formula::Variable('b')),
            )),
            Box::new(Formula::Variable('c')),
        ),
        f
    );
}

#[test]
fn parsing_priorities() {
    // ¬,∧,∨,⇒,⇐,⇔
    let input = "¬q∨¬¬¬r∧s⇔t∧b";
    let f = Formula::read(input).unwrap();
    assert_eq!(
        Formula::Equiv(
            Box::new(Formula::Or(
                Box::new(Formula::Not(Box::new(Formula::Variable('q')))),
                Box::new(Formula::And(
                    Box::new(Formula::Not(Box::new(Formula::Not(Box::new(
                        Formula::Not(Box::new(Formula::Variable('r')))
                    ))))),
                    Box::new(Formula::Variable('s'))
                ))
            )),
            Box::new(Formula::And(
                Box::new(Formula::Variable('t')),
                Box::new(Formula::Variable('b')),
            ))
        ),
        f
    );
}
