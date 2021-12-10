use std::fmt;

use crate::parse::parse;
#[derive(Clone, Debug)]
struct State(Vec<(String, i32)>);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|(k, v)| format!("{} |-> {}", k, v))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl State {
    fn substitute(&self, s: String, i: i32) -> State {
        let State(v) = self;
        let new_vec = v
            .iter()
            .map(|(k, v)| {
                if k == &s {
                    (k.clone(), i)
                } else {
                    (k.clone(), *v)
                }
            })
            .collect();
        State(new_vec)
    }
    fn create_state(v: Vec<String>) -> State {
        State(v.into_iter().map(|x| (x, 0)).collect())
    }
}
#[derive(Clone, Debug)]
pub enum Pgm {
    Program(Vec<String>, Stmt),
}

impl fmt::Display for Pgm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pgm::Program(vars, s) => {
                write!(f, "int {} ; \n{}", vars.join(", "), s.to_string())
            }
        }
    }
}
#[derive(Clone, Debug)]
enum Configuration {
    AExpConf(Box<AExp>, State),
    BExpConf(Box<BExp>, State),
    StmtConf(Box<Stmt>, State),
    PgmConf(Box<Pgm>),
    Dummy, //top level, meaning that it is an unconditional rewrite
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Configuration::AExpConf(a, s) => {
                write!(f, "< {}, {} >", a.to_string(), s.to_string())
            }
            Configuration::BExpConf(a, s) => {
                write!(f, "< {}, {} >", a.to_string(), s.to_string())
            }
            Configuration::StmtConf(a, s) => {
                write!(f, "< {}, {} >", a.to_string(), s.to_string())
            }
            Configuration::PgmConf(a) => {
                write!(f, "< {} >", a.to_string())
            }
            Configuration::Dummy => {
                write!(f, "Error this shouldn't be here")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AExp {
    Plus(Box<AExp>, Box<AExp>),
    Divide(Box<AExp>, Box<AExp>),
    Id(String),
    Int(i32),
}

impl fmt::Display for AExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AExp::Plus(a1, a2) => {
                write!(f, "{} + {}", a1.to_string(), a2.to_string())
            }
            AExp::Divide(a1, a2) => {
                write!(f, "{} + {}", a1.to_string(), a2.to_string())
            }
            AExp::Id(s) => {
                write!(f, "{}", s.to_string())
            }
            AExp::Int(s) => {
                write!(f, "{}", s.to_string())
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum BExp {
    LessThanEq(Box<AExp>, Box<AExp>),
    Negation(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Bool(bool),
}

impl fmt::Display for BExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BExp::LessThanEq(a, s) => {
                write!(f, "{} <= {}", a.to_string(), s.to_string())
            }
            BExp::Negation(a) => {
                write!(f, "!({})", a.to_string())
            }
            BExp::And(a, s) => {
                write!(f, "{} && {}", a.to_string(), s.to_string())
            }
            BExp::Bool(a) => {
                write!(f, "{}", a)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    StmtBlock(Box<Block>),
    Assign(String, Box<AExp>),
    Sequence(Box<Stmt>, Box<Stmt>),
    IfThenElse(Box<BExp>, Box<Block>, Box<Block>),
    While(Box<BExp>, Box<Block>),
}

#[derive(Clone, Debug)]
pub enum Block {
    EmptyBlock,
    BlockStmt(Box<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::StmtBlock(x) => {
                write!(f, "{}", x.to_string())
            }
            Stmt::Assign(s, a) => {
                write!(f, "{} = {} ; ", s.to_string(), a.to_string())
            }
            Stmt::Sequence(s1, s2) => {
                write!(f, "{}\n{}", s1.to_string(), s2.to_string())
            }
            Stmt::IfThenElse(b, b1, b2) => {
                write!(
                    f,
                    "if {} then\n {} \n else {} \n end",
                    b.to_string(),
                    b1.to_string(),
                    b2.to_string()
                )
            }
            Stmt::While(b, block) => {
                write!(
                    f,
                    "while {} do \n {} \n end",
                    b.to_string(),
                    block.to_string()
                )
            }
        }
    }
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::EmptyBlock => {
                write!(f, "{{}}")
            }
            Block::BlockStmt(x) => {
                write!(f, "{}", x.to_string())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    stack: Vec<Configuration>,
    rules: Vec<Rule>,
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.stack
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n---\n")
        )
    }
}
impl Stack {
    pub fn create_from_string(s: String) -> Option<Stack> {
        let pgm = parse(s)?;
        let s = Stack {
            stack: vec![Configuration::PgmConf(Box::new(pgm))],
            rules: vec![],
        };
        Some(s)
    }
    pub fn new() -> Stack {
        // let variables = vec!["x".to_string(), "y".to_string()];
        // let assign_x = Stmt::Assign("x".to_string(), Box::new(AExp::Int(5)));
        // let assign_y = Stmt::Assign("y".to_string(), Box::new(AExp::Int(7)));
        // let evaluate_x = AExp::Id("x".to_string());
        // let set_y_to_x = Stmt::Assign("y".to_string(), Box::new(evaluate_x));

        // let program = Stmt::Sequence(
        //     assign_x.into(),
        //     Box::new(Stmt::Sequence(Box::new(assign_y), Box::new(set_y_to_x))),
        // );
        // Stack {
        //     stack: vec![Configuration::PgmConf(Box::new(Pgm::Program(
        //         variables, program,
        //     )))],
        //     rules: vec![],
        // }
        let variables = vec!["x".to_string(), "y".to_string()];
        let assign_x = Stmt::Assign("x".to_string(), Box::new(AExp::Int(5)));
        let assign_y = Stmt::Assign("y".to_string(), Box::new(AExp::Int(7)));
        let evaluate_x = AExp::Id("x".to_string());
        let evaluate_y = AExp::Id("y".to_string());
        let add_to_x = AExp::Plus(Box::new(evaluate_x.clone()), Box::new(AExp::Int(1)));
        let add_to_x2 = Stmt::Assign("x".to_string(), Box::new(add_to_x));
        let less_xy = BExp::LessThanEq(Box::new(evaluate_x), Box::new(evaluate_y));
        let while_xy = Stmt::While(
            Box::new(less_xy),
            Box::new(Block::BlockStmt(Box::new(add_to_x2))),
        );

        let program = Stmt::Sequence(
            assign_x.into(),
            Box::new(Stmt::Sequence(Box::new(assign_y), Box::new(while_xy))),
        );
        Stack {
            stack: vec![Configuration::PgmConf(Box::new(Pgm::Program(
                variables, program,
            )))],
            rules: vec![],
        }
    }
    pub fn can_apply_rule(&self, rule: Rule) -> bool {
        let last = self.stack.last().expect("oops");
        let next_configuration = rule.get_next_configuration(last.clone());
        println!("{:?}", next_configuration);
        match next_configuration {
            Some(Configuration::Dummy) => {
                let mut top_conf = Configuration::Dummy;
                    let bottom_conf = last;
                    println!("{:?}", self);
                    match rule.reduce_down(bottom_conf.clone(), top_conf) {
                        None => {
                            false
                        }
                        Some(x) => true
                    }
            }
            Some(conf) => {
                true
            }
            None => {
                false
            }
        }
    }
    // true means a sucess apply, false failed to apply rule
    pub fn applyRule(&mut self, rule: Rule) -> bool {
        println!("{:?}", self);
        self.rules.push(rule.clone());
        println!("{:?}", self);
        let last = self.stack.last().expect("oops");
        let next_configuration = rule.get_next_configuration(last.clone());
        println!("{:?}", next_configuration);
        match next_configuration {
            Some(Configuration::Dummy) => {
                let mut top_conf = Configuration::Dummy;
                while !self.rules.is_empty() {
                    let rule = self.rules.pop().expect("failed to pop rule stack");
                    let bottom_conf = self.stack.pop().expect("failed to pop stack");
                    println!("{:?}", self);
                    match rule.reduce_down(bottom_conf.clone(), top_conf) {
                        None => {
                            self.stack.push(bottom_conf); //restore the stack
                            self.rules.pop(); // remove bad rule
                            return false;
                        }
                        Some(x) => top_conf = x,
                    }

                    println!("{:?}", self);
                }
                self.stack.push(top_conf);
                println!("{:?}", self);
            }
            Some(conf) => {
                self.stack.push(conf);
            }
            None => {
                self.rules.pop();
                return false;
            }
        }
        true
    }
    pub fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
            self.rules.pop();
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Rule {
    //  crl o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .
    RewriteVariableLookup,

    //  crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
    RewritePlusLeft,

    //  crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
    RewritePlusRight,

    //   rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .
    RewritePlus,

    //  crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
    RewriteDivideLeft,
    //  crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
    RewriteDivideRight,
    // crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .
    RewriteDivide,
    // crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
    RewriteLessThanLeft,
    // crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
    RewriteLessThanRight,
    // rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .
    RewriteLessThan,
    // crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .
    RewriteNegate,
    // rl o < ! true,Sigma > => < false,Sigma > .
    RewriteNegateTrue,

    // rl o < ! false,Sigma > => < true,Sigma > .
    RewriteNegateFalse,
    //   rl o < {S},Sigma > => < S,Sigma > .
    RewriteBlockStatement,

    // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."
    RewriteAssignmentArith,
    // Enum::Fourth => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),
    RewriteAssignmentInt,

    // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
    RewriteSequence,
    // "rl o < {} S2,Sigma > => < S2,Sigma > ."
    RewriteEmptyBlock,

    // crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .
    RewriteConditional,
    // rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .
    RewriteConditionalTrue,
    // rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .
    RewriteConditionalFalse,
    // rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .
    RewriteLoop,

    // Enum::Fifth => " o < int Xl ; S > => < S,(Xl |-> 0) > .".to_string(),
    RewriteTop,
    // Enum::Sixth => "None selected".to_string(),
    NoOp,
}

impl Rule {
    pub fn list_of_rules() -> Vec<Rule> {
        vec![
            Rule::RewriteVariableLookup,
            Rule::RewritePlusLeft,
            Rule::RewritePlusRight,
            Rule::RewritePlus,
            Rule::RewriteDivideLeft,
            Rule::RewriteDivideRight,
            Rule::RewriteDivide,
            Rule::RewriteLessThanLeft,
            Rule::RewriteLessThanRight,
            Rule::RewriteLessThan,
            Rule::RewriteNegate,
            Rule::RewriteNegateTrue,
            Rule::RewriteNegateFalse,
            Rule::RewriteBlockStatement,
            Rule::RewriteAssignmentArith,
            Rule::RewriteAssignmentInt,
            Rule::RewriteSequence,
            Rule::RewriteEmptyBlock,
            Rule::RewriteConditional,
            Rule::RewriteConditionalTrue,
            Rule::RewriteConditionalFalse,
            Rule::RewriteLoop,
            Rule::RewriteTop,
        ]
    }
    pub fn get_description(&self) -> String {
        match self {
             Rule::RewriteVariableLookup => "crl o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .".to_string(),
             Rule::RewritePlusLeft => "crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
             Rule::RewritePlusRight => "crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
             Rule::RewritePlus => " rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .".to_string(),
             Rule::RewriteDivideLeft => "crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
             Rule::RewriteDivideRight => "crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
             Rule::RewriteDivide => "crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .".to_string(),
             Rule::RewriteLessThanLeft => "crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
             Rule::RewriteLessThanRight => "crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
             Rule::RewriteLessThan => "rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .".to_string(),
             Rule::RewriteNegate => "crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .".to_string(),
             Rule::RewriteNegateTrue => "rl o < ! true,Sigma > => < false,Sigma > .".to_string(),
             Rule::RewriteNegateFalse => "rl o < ! false,Sigma > => < true,Sigma > .".to_string(),
             Rule::RewriteBlockStatement => "rl o < {S},Sigma > => < S,Sigma > .".to_string(),
             Rule::RewriteAssignmentArith => "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .".to_string(),
             Rule::RewriteAssignmentInt => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),
             Rule::RewriteSequence => "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > .".to_string(),
             Rule::RewriteEmptyBlock => "rl o < {} S2,Sigma > => < S2,Sigma > .".to_string(),
             Rule::RewriteConditional => "crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .".to_string(),
             Rule::RewriteConditionalTrue => "rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .".to_string(),
             Rule::RewriteConditionalFalse => "rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .".to_string(),

             Rule::RewriteLoop => "rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .".to_string(),
             Rule::RewriteTop  => "rl o < int Xl ; S > => < S,(Xl |-> 0) > .".to_string(),
             Rule::NoOp  => "This was not supposed to be available".to_string(),
        }
    }

    fn get_next_configuration(&self, conf: Configuration) -> Option<Configuration> {
        let ret = match self {
            Rule::RewritePlus => Configuration::Dummy,
            Rule::RewritePlusLeft => {
                //crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .

                match conf {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Plus(a1, _a2) => Configuration::AExpConf(a1, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteVariableLookup => {
                // o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .

                match conf {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Id(x) => {
                            let State(v) = sigma;
                            match v.iter().find(|(k, _v)| k == &x) {
                                Some((_k, _v)) => Configuration::Dummy,
                                _ => return None,
                            }
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewritePlusRight => {
                // crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .

                match conf {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Plus(_a1, a2) => Configuration::AExpConf(a2, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteDivide => {
                // Configuration::
                // crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .
                Configuration::Dummy
            }
            Rule::RewriteDivideLeft => {
                // crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma >
                match conf {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Divide(a1, _a2) => Configuration::AExpConf(a1, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteDivideRight => {
                // crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
                match conf {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Divide(_a1, a2) => Configuration::AExpConf(a2, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            // crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
            Rule::RewriteLessThanLeft => match conf {
                Configuration::BExpConf(x, sigma) => match *x {
                    BExp::LessThanEq(a1, _a2) => Configuration::AExpConf(a1, sigma),
                    _ => return None,
                },
                _ => return None,
            },
            // crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
            Rule::RewriteLessThanRight => {
                // match conf {
                //     Configuration::BExpConf(x, sigma) =>
                //     match *x {
                //         BExp::LessThanEq(a1, a2) => Configuration::AExpConf(a2, sigma),
                //         _ => return None
                //     }
                //     _ => return None
                // }
                Configuration::Dummy
            }
            // rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .
            Rule::RewriteLessThan => Configuration::Dummy,
            // crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .
            Rule::RewriteNegate => match conf {
                Configuration::BExpConf(x, sigma) => match *x {
                    BExp::Negation(b) => Configuration::BExpConf(b, sigma),
                    _ => return None,
                },
                _ => return None,
            },
            // rl o < ! true,Sigma > => < false,Sigma > .
            Rule::RewriteNegateTrue => Configuration::Dummy,
            // rl o < ! false,Sigma > => < true,Sigma > .
            Rule::RewriteNegateFalse => Configuration::Dummy,
            //   rl o < {S},Sigma > => < S,Sigma > .
            Rule::RewriteBlockStatement => Configuration::Dummy,
            Rule::RewriteEmptyBlock => {
                //
                Configuration::Dummy
            }
            Rule::RewriteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                match conf {
                    Configuration::StmtConf(x, sigma) => match *x {
                        Stmt::Sequence(s1, _) => Configuration::StmtConf(s1, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."
                match conf {
                    Configuration::StmtConf(x, sigma) => match *x {
                        Stmt::Assign(_x, a) => Configuration::AExpConf(a, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteAssignmentInt => {
                // Configuration::
                // Enum::Fourth => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined ."
                Configuration::Dummy
            }
            Rule::RewriteTop => {
                // Configuration::

                Configuration::Dummy
            }
            Rule::RewriteConditional => {
                // crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .
                match conf {
                    Configuration::StmtConf(s, sigm) => match *s {
                        Stmt::IfThenElse(b_ptr, _s1_ptr, _s2_ptr) => {
                            Configuration::BExpConf(b_ptr, sigm)
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            // rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .
            Rule::RewriteConditionalTrue => Configuration::Dummy,
            // rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .
            Rule::RewriteConditionalFalse => Configuration::Dummy,
            // rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .
            Rule::RewriteLoop => Configuration::Dummy,
            Rule::NoOp => return None,
        };
        Some(ret)
    }

    // bottom is the configuration below the line,
    // top is the configuration above the line, and we want to return the new top
    fn reduce_down(&self, bottom: Configuration, top: Configuration) -> Option<Configuration> {
        let x = match self {
            Rule::RewriteVariableLookup => {
                // o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .

                match bottom {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Id(x) => {
                            let State(v) = sigma.clone();
                            match v.iter().find(|(k, _v)| k == &x) {
                                Some((_k, v)) => {
                                    Configuration::AExpConf(Box::new(AExp::Int(*v)), sigma)
                                }
                                _ => return None,
                            }
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewritePlusLeft => {
                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Plus(box1, _box2) => {
                            Configuration::AExpConf(Box::new(AExp::Plus(box1, new_arith)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewritePlusRight => {
                // crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .

                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Plus(_box1, box2) => {
                            Configuration::AExpConf(Box::new(AExp::Plus(box2, new_arith)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewritePlus => match bottom {
                Configuration::AExpConf(x, sigma) => match *x {
                    AExp::Plus(box1, box2) => match *box1 {
                        AExp::Int(n1) => match *box2 {
                            AExp::Int(n2) => {
                                Configuration::AExpConf(Box::new(AExp::Int(n1 + n2)), sigma)
                            }
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },
            //  crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
            Rule::RewriteDivideLeft => {
                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Divide(box1, _box2) => {
                            Configuration::AExpConf(Box::new(AExp::Divide(box1, new_arith)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            //  crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
            Rule::RewriteDivideRight => {
                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::AExpConf(x, sigma) => match *x {
                        AExp::Divide(_box1, box2) => {
                            Configuration::AExpConf(Box::new(AExp::Divide(box2, new_arith)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteDivide => match bottom {
                Configuration::AExpConf(x, sigma) => match *x.clone() {
                    AExp::Divide(box1, box2) => match *box1 {
                        AExp::Int(n1) => match *box2 {
                            AExp::Int(0) => Configuration::AExpConf(x, sigma),
                            AExp::Int(n2) => {
                                Configuration::AExpConf(Box::new(AExp::Int(n1 / n2)), sigma)
                            }
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },

            Rule::RewriteLessThanLeft => {
                // crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::BExpConf(x, sigma) => match *x {
                        BExp::LessThanEq(_box1, box2) => Configuration::BExpConf(
                            Box::new(BExp::LessThanEq(new_arith, box2)),
                            sigma,
                        ),
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteLessThanRight => {
                let new_arith = match top {
                    Configuration::AExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::BExpConf(x, sigma) => match *x {
                        BExp::LessThanEq(box1, _box2) => Configuration::BExpConf(
                            Box::new(BExp::LessThanEq(box1, new_arith)),
                            sigma,
                        ),
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteLessThan => match bottom {
                Configuration::BExpConf(_x, _sigma) => match *_x {
                    BExp::LessThanEq(box1, box2) => match *box1 {
                        AExp::Int(i1) => match *box2 {
                            AExp::Int(i2) => {
                                if i2 > i1 {
                                    Configuration::BExpConf(Box::new(BExp::Bool(true)), _sigma)
                                } else {
                                    Configuration::BExpConf(Box::new(BExp::Bool(false)), _sigma)
                                }
                            }
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },

            // crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .
            Rule::RewriteNegate => {
                let new_bool = match top {
                    Configuration::BExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::BExpConf(x, sigma) => match *x {
                        BExp::Negation(_box1) => Configuration::BExpConf(new_bool, sigma),
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            // rl o < ! true,Sigma > => < false,Sigma > .
            Rule::RewriteNegateTrue => match bottom {
                Configuration::BExpConf(x, sigma) => match *x {
                    BExp::Negation(box1) => match *box1 {
                        BExp::Bool(true) => {
                            Configuration::BExpConf(Box::new(BExp::Bool(false)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },

            // rl o < ! false,Sigma > => < true,Sigma > .
            Rule::RewriteNegateFalse => match bottom {
                Configuration::BExpConf(x, sigma) => match *x {
                    BExp::Negation(box1) => match *box1 {
                        BExp::Bool(false) => {
                            Configuration::BExpConf(Box::new(BExp::Bool(true)), sigma)
                        }
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },

            Rule::RewriteBlockStatement => {
                // rl o < {S},Sigma > => < S,Sigma > .
                match bottom {
                    Configuration::StmtConf(x, sigma) => match *x {
                        Stmt::StmtBlock(s) => match *s {
                            Block::BlockStmt(s) => Configuration::StmtConf(s, sigma),
                            _ => return None,
                        },

                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."

                match bottom {
                    Configuration::StmtConf(x, _sigma) => match *x {
                        Stmt::Assign(x, _a) => match top {
                            Configuration::AExpConf(a_prime, sigma) => {
                                Configuration::StmtConf(Stmt::Assign(x, a_prime).into(), sigma)
                            }
                            _ => return None,
                        },

                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteAssignmentInt => {
                // Configuration::
                // "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined ."
                match bottom {
                    Configuration::StmtConf(x, sigma) => match *x {
                        Stmt::Assign(x, a) => match *a {
                            AExp::Int(i) => Configuration::StmtConf(
                                Stmt::StmtBlock(Block::EmptyBlock.into()).into(),
                                sigma.substitute(x, i),
                            ),
                            _ => return None,
                        },

                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                match bottom {
                    Configuration::StmtConf(x, _sigma) => match *x {
                        Stmt::Sequence(_s1, s2) => match top {
                            Configuration::StmtConf(s1_prime, sigma_prime) => {
                                Configuration::StmtConf(
                                    Box::new(Stmt::Sequence(s1_prime, s2)),
                                    sigma_prime,
                                )
                            }
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteTop => {
                // Configuration::
                //rl o < int Xl ; S > => < S,(Xl |-> 0) > .

                match bottom {
                    Configuration::PgmConf(p) => match *p {
                        Pgm::Program(xl, s) => {
                            Configuration::StmtConf(s.into(), State::create_state(xl))
                        }
                        _ => return None,
                    },
                    _ => return None,
                }
            }

            Rule::RewriteEmptyBlock => {
                // rl o < {} S2,Sigma > => < S2,Sigma > .
                match bottom {
                    Configuration::StmtConf(s, sigma) => match *s {
                        Stmt::Sequence(s1, s2) => match *s1 {
                            Stmt::StmtBlock(b) => match *b {
                                Block::EmptyBlock => Configuration::StmtConf(s2, sigma),
                                _ => return None,
                            },
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            // crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .
            Rule::RewriteConditional => {
                let new_bool = match top {
                    Configuration::BExpConf(b, _sigma) => b,
                    _ => return None,
                };
                match bottom {
                    Configuration::StmtConf(s, sigm) => match *s {
                        Stmt::IfThenElse(_b_ptr, s1_ptr, s2_ptr) => Configuration::StmtConf(
                            Stmt::IfThenElse(new_bool, s1_ptr, s2_ptr).into(),
                            sigm,
                        ),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            Rule::RewriteConditionalTrue => {
                // rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .
                match bottom {
                    Configuration::StmtConf(s, sigm) => match *s {
                        Stmt::IfThenElse(b_ptr, s1_ptr, _s2_ptr) => match *b_ptr {
                            BExp::Bool(true) => {
                                Configuration::StmtConf(Box::new(Stmt::StmtBlock(s1_ptr)), sigm)
                            }
                            _ => return None,
                        },
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            // rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .
            Rule::RewriteConditionalFalse => match bottom {
                Configuration::StmtConf(s, sigm) => match *s {
                    Stmt::IfThenElse(b_ptr, _s1_ptr, s2_ptr) => match *b_ptr {
                        BExp::Bool(false) => {
                            Configuration::StmtConf(Stmt::StmtBlock(s2_ptr).into(), sigm)
                        }
                        _ => return None,
                    },
                    _ => return None,
                },
                _ => return None,
            },
            // rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .
            Rule::RewriteLoop => match bottom {
                Configuration::StmtConf(s, sigm) => match *s {
                    Stmt::While(b_ptr, s_ptr) => Configuration::StmtConf(
                        Stmt::IfThenElse(
                            b_ptr.clone(),
                            Block::BlockStmt(
                                Stmt::Sequence(
                                    Stmt::StmtBlock(s_ptr.clone()).into(),
                                    Stmt::While(b_ptr, s_ptr).into(),
                                )
                                .into(),
                            )
                            .into(),
                            Block::EmptyBlock.into(),
                        )
                        .into(),
                        sigm,
                    ),
                    _ => return None,
                },
                _ => return None,
            },
            Rule::NoOp => return None,
        };
        Some(x)
    }
}

impl Configuration {
    fn rewrite_top() {}
    fn rewrite_conditional() {}
}

/*

start

int x; x = 5 + 5;

user presses apply button
< x = 5 + 5, (x -> 0) >


user selectes   crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .

< 5 + 5, (x -> 0) >
-----  crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = 5 + 5 , (x -> 0) >

user selects    rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .

< x = 10, (x -> 0) >


*/
/*

start

int x; x = (1 + 1) + (1 + 1);

user selects   rl o < int Xl ; S > => < S,(Xl |-> 0) > .
< x = (1 + 1) + (1 + 1), (x -> 0) >


user selectes   crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .

< (1 + 1) + (1 + 1), (x -> 0), (x -> 0) >
-----  crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = (1 + 1) + (1 + 1) , (x -> 0) >

user selects     crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .

< (1 + 1), (x -> 0), (x -> 0) >
----crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .
< (1 + 1) + (1 + 1), (x -> 0), (x -> 0) >
-----  crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = (1 + 1) + (1 + 1) , (x -> 0) >

user selects   rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .
< x = (2) + (1 + 1) , (x -> 0) >


user selectes   crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< 2 + (1 + 1) , (x -> 0) >
--- crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = (2) + (1 + 1) , (x -> 0) >


user selects  crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
< (1 + 1) , (x -> 0) >
---
< 2 + (1 + 1) , (x -> 0) >
--- crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = (2) + (1 + 1) , (x -> 0) >


user selects  rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .
< x = (2) + (2) , (x -> 0) >


user selectes   crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< 2 + 2, (x -> 0) >
----crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .
< x = (2) + (2) , (x -> 0) >

user selects  rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .
< x = 4 , (x -> 0) >

*/
