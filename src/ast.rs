
#[derive(Clone)]
struct State(Vec<(String, u32)>);

impl State {
    fn substitute(&self, s:String , i: u32) -> State {
        let State(v) = self;
        let new_vec = v.iter().map(|(k, v)| {
            if k == &s {
                (k.clone(), i)
            } else {
                (k.clone(), *v)
            }
        }).collect();
        State(new_vec)
    }
    fn create_state(v: Vec<String>) -> State {
        State(v.into_iter().map(|x| (x, 0)).collect())
    }
}

#[derive(Clone)]
enum Pgm {
    Program(Vec<String>, Stmt),
}

#[derive(Clone)]
enum Configuration {
    AExpConf(Box<AExp>, State),
    BExpConf(Box<BExp>, State),
    StmtConf(Box<Stmt>, State),
    PgmConf(Box<Pgm>),
    Dummy,
}

#[derive(Clone)]
enum AExp {
    Plus(Box<AExp>, Box<AExp>),
    Divide(Box<AExp>, Box<AExp>),
    Id(String),
    Int(u32),
}

#[derive(Clone)]
enum BExp {
    LessThanEq(Box<AExp>, Box<AExp>),
    Negation(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Bool(bool),
}

#[derive(Clone)]
enum Stmt {
    StmtBlock(Box<Block>),
    Assign(String, Box<AExp>),
    Sequence(Box<Stmt>, Box<Stmt>),
    IfThenElse(BExp, Block, Block),
    While(BExp, Block),
}

#[derive(Clone)]
enum Block {
    EmptyBlock,
    BlockStmt(Box<Stmt>),
}


#[derive(Clone)]
struct Stack {
    stack: Vec<Configuration>,
    rules: Vec<Rule>,
}

impl Stack {
    fn applyRule(&mut self, rule: Rule) {
        self.rules.push(rule.clone());
        let last = self.stack.last().expect("oops");
        let next_configuration = rule.getNextConfiguartion(last.clone());
        match next_configuration {
            Some(conf) => {

                self.stack.push(conf);
            }
            None => {
                let mut top_conf = Configuration::Dummy;
                while !self.rules.is_empty() {
                    let rule = self.rules.pop().expect("failed to pop rule stack");
                    let bottom_conf = self.stack.pop().expect("failed to pop stack");
                    rule.reduce_down(bottom_conf.clone(), top_conf);
                    top_conf = bottom_conf;
                }
            }
        }
    }
}

#[derive(Clone)]
enum Rule {
    RewritePlusLeft,
    RewritePlusRight,
    RewritePlus, 

    // "rl o < {} S2,Sigma > => < S2,Sigma > ."
    RewriteEmptyBlock,


    // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
    RerwiteSequence,
    // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."
    RewriteAssignmentArith,
    // Enum::Fourth => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),
    RewriteAssigmentInt,
    // Enum::Fifth => " o < int Xl ; S > => < S,(Xl |-> 0) > .".to_string(),
    RewriteTop,
    // Enum::Sixth => "None selected".to_string(),
    NoOp,

}


impl Rule {
    fn getNextConfiguartion(&self, conf: Configuration) -> Option<Configuration> {
        let ret = match self {
            Rule::RewritePlus => {
                todo!()
            }
            Rule::RewritePlusLeft => {
                 //crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .

                 match conf {
                    Configuration::AExpConf(x, sigma) => 
                    match *x {
                       AExp::Plus(a1, a2) => Configuration::AExpConf(a1, sigma),
                       _ => todo!()
                    }
                    _ => todo!()
                 }

            }
            Rule::RewritePlusRight => {
                // Configuration::
                todo!()
            }
            Rule::RewriteEmptyBlock => {
                // 
                return None;


            }
            Rule::RerwiteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                 match conf {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Sequence(s1, _) => Configuration::StmtConf(s1, sigma),
                       _ => todo!()
                    }
                    _ => todo!()
                 }
            }
            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."

                 match conf {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Assign(x, a) => Configuration::AExpConf(a, sigma),
                       _ => todo!()
                    }
                    _ => todo!()
                 }
            }
            Rule::RewriteAssigmentInt => {
                // Configuration::
                // Enum::Fourth => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),
                return None;

            }
            Rule::RewriteTop => {
                // Configuration::

                return None;
            }
            _ => todo!()
        };
        Some(ret)
    }

    fn reduce_down(&self, bottom: Configuration, top: Configuration) -> Configuration {
        let x = match self {
            Rule::RerwiteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                 match bottom {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Sequence(s1, s2) => 
                       match top {
                        Configuration::StmtConf(s1_prime, sigma_prime) => Configuration::StmtConf(Box::new(Stmt::Sequence(s1_prime, s2)), sigma_prime),
                        _ => todo!()
                       }
                       _ => todo!()
                    }
                    _ => todo!()
                 }
            }

            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."

                 match bottom {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Assign(x, a) => 
                       match top {
                            Configuration::AExpConf(a_prime, sigma) => Configuration::StmtConf(Stmt::Assign(x, a_prime).into(), sigma),
                            _ => todo!()
                       }
                       
                       _ => todo!()
                    }
                    _ => todo!()
                 }
            }
            Rule::RewriteAssigmentInt => {
                // Configuration::
                // "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined ."
                match bottom {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                      Stmt::Assign(x, a) => 
                      match *a {
                          AExp::Int(i) => Configuration::StmtConf(Stmt::StmtBlock(Block::EmptyBlock.into()).into(), sigma.substitute(x, i)), 
                          _ => todo!()
                      }
                        
                       _ => todo!()
                    }
                    _ => todo!()
                 }

            }
            Rule::RewriteTop => {
                // Configuration::
                //rl o < int Xl ; S > => < S,(Xl |-> 0) > .

                match bottom {
                    Configuration::PgmConf(p) => {
                        match *p {
                            Pgm::Program(xl, s) => Configuration::StmtConf(s.into(), State::create_state(xl)),
                            _ => todo!()
                        }
                    }
                    _ => todo!()
                }

            }

            Rule::RewriteEmptyBlock => {
                // rl o < {} S2,Sigma > => < S2,Sigma > .
                match bottom {
                    Configuration::StmtConf(s, sigma) =>  {
                        match *s {
                            Stmt::Sequence(s1, s2) => {
                                match *s1 {
                                    Stmt::StmtBlock(b) => {
                                        match *b {
                                            Block::EmptyBlock => Configuration::StmtConf(s2, sigma),
                                            _ => todo!()
                                        }
                                    },
                                    _ => todo!()
                                }
                            },
                            _ => todo!()
                        }   
                    }
                    _ => todo!()
                }
            }
            _ => todo!()
        };
        todo!()
    }
}


impl Configuration {
    fn rewrite_top() {
        
    }
    fn rewrite_conditional() {
        
    }

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