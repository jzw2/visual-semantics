
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum Pgm {
    Program(Vec<String>, Stmt),
}

#[derive(Clone, Debug)]
enum Configuration {
    AExpConf(Box<AExp>, State),
    BExpConf(Box<BExp>, State),
    StmtConf(Box<Stmt>, State),
    PgmConf(Box<Pgm>),
    Dummy, //top level, meaning that it is an unconditional rewrite
}

#[derive(Clone, Debug)]
enum AExp {
    Plus(Box<AExp>, Box<AExp>),
    Divide(Box<AExp>, Box<AExp>),
    Id(String),
    Int(u32),
}

#[derive(Clone, Debug)]
enum BExp {
    LessThanEq(Box<AExp>, Box<AExp>),
    Negation(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Bool(bool),
}

#[derive(Clone, Debug)]
enum Stmt {
    StmtBlock(Box<Block>),
    Assign(String, Box<AExp>),
    Sequence(Box<Stmt>, Box<Stmt>),
    IfThenElse(BExp, Block, Block),
    While(BExp, Block),
}

#[derive(Clone, Debug)]
enum Block {
    EmptyBlock,
    BlockStmt(Box<Stmt>),
}


#[derive(Clone, Debug)]
pub struct Stack {
    stack: Vec<Configuration>,
    rules: Vec<Rule>,
}

impl Stack {
    pub fn new() -> Stack {
    Stack { stack: vec![Configuration::PgmConf(
        Box::new(Pgm::Program(vec!["x".to_string()], Stmt::Assign("x".to_string(), Box::new(AExp::Int(5)))))
        )], rules: vec![] }
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
            Some (Configuration::Dummy) => {
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

  

    // "rl o < {} S2,Sigma > => < S2,Sigma > ."
    RewriteEmptyBlock,


    // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
    RewriteSequence,
    // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."
    RewriteAssignmentArith,
    // Enum::Fourth => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),
    RewriteAssignmentInt,
    // Enum::Fifth => " o < int Xl ; S > => < S,(Xl |-> 0) > .".to_string(),
    RewriteTop,
    // Enum::Sixth => "None selected".to_string(),
    
    NoOp,

}


impl Rule {
    fn get_next_configuration(&self, conf: Configuration) -> Option<Configuration> {
        let ret = match self {
            Rule::RewritePlus => {
                return None
            }
            Rule::RewritePlusLeft => {
                 //crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .

                 match conf {
                    Configuration::AExpConf(x, sigma) => 
                    match *x {
                       AExp::Plus(a1, _a2) => Configuration::AExpConf(a1, sigma),
                       _ => return None
                    }
                    _ => return None
                 }

            }
            Rule::RewriteVariableLookup => {
                // o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .
                
                match conf {
                    Configuration::AExpConf(x, sigma) => 
                      match *x {
                        AExp::Id(x) => {
                            let State(v) = sigma;
                            match v.iter().find(|(k, v)| k == &x) {
                                Some((k, v)) => Configuration::Dummy, 
                                _ => return None,
                            }
                        }
                        _ => return None
                      },
                    _ => return None
                }
            }
            Rule::RewritePlusRight => {
                // crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
                
                match conf {
                    Configuration::AExpConf(x, sigma) => 
                    match *x {
                       AExp::Plus(a1, a2) => Configuration::AExpConf(a2, sigma),
                       _ => return None
                    }
                    _ => return None
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
                    Configuration::AExpConf(x, sigma) => 
                    match *x {
                       AExp::Divide(a1, _a2) => Configuration::AExpConf(a1, sigma),
                       _ => return None
                    }
                    _ => return None
                 }
            }
            Rule::RewriteDivideRight => {
                // crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .
                match conf {
                    Configuration::AExpConf(x, sigma) =>
                    match *x {
                        AExp::Divide(a1, a2) => Configuration::AExpConf(a2, sigma),
                        _ => return None
                    }
                    _ => return None
                }
            }
            Rule::RewriteEmptyBlock => {
                // 
                return None;
            }
            Rule::RewriteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                 match conf {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Sequence(s1, _) => Configuration::StmtConf(s1, sigma),
                       _ => return None,
                    }
                    _ => return None,
                 }
            }
            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."
                 match conf {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                       Stmt::Assign(_x, a) => Configuration::AExpConf(a, sigma),
                       _ => return None,
                    }
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
            _ => return None,
        };
        Some(ret)
    }

    fn reduce_down(&self, bottom: Configuration, top: Configuration) -> Option<Configuration> {
        
        let x = match self {


            Rule::RewriteVariableLookup => {
                // o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .
                
                match bottom {
                    Configuration::AExpConf(x, sigma) => 
                      match *x {
                        AExp::Id(x) => {
                            let State(v) = sigma.clone();
                            match v.iter().find(|(k, v)| k == &x) {
                                Some((k, v)) => Configuration::AExpConf(Box::new(AExp::Int(*v)), sigma),
                                _ => return None,
                            }
                        }
                        _ => return None
                      },
                    _ => return None
                }
            }
            Rule::RewriteSequence => {
                // "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > ."
                 match bottom {
                    Configuration::StmtConf(x, _sigma) => 
                    match *x {
                       Stmt::Sequence(_s1, s2) => 
                       match top {
                        Configuration::StmtConf(s1_prime, sigma_prime) => Configuration::StmtConf(Box::new(Stmt::Sequence(s1_prime, s2)), sigma_prime),
                        _ => return None,
                       }
                       _ => return None,
                    }
                    _ => return None,
                 }
            }

            Rule::RewriteAssignmentArith => {
                // Configuration::
                // "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > ."

                 match bottom {
                    Configuration::StmtConf(x, _sigma) => 
                    match *x {
                       Stmt::Assign(x, _a) => 
                       match top {
                            Configuration::AExpConf(a_prime, sigma) => Configuration::StmtConf(Stmt::Assign(x, a_prime).into(), sigma),
                            _ => return None,
                       }
                       
                       _ => return None,
                    }
                    _ => return None,
                 }
            }
            Rule::RewriteAssignmentInt => {
                // Configuration::
                // "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined ."
                match bottom {
                    Configuration::StmtConf(x, sigma) => 
                    match *x {
                      Stmt::Assign(x, a) => 
                      match *a {
                          AExp::Int(i) => Configuration::StmtConf(Stmt::StmtBlock(Block::EmptyBlock.into()).into(), sigma.substitute(x, i)), 
                          _ => return None,
                      }
                        
                       _ => return None,
                    }
                    _ => return None,
                 }

            }
            Rule::RewriteTop => {
                // Configuration::
                //rl o < int Xl ; S > => < S,(Xl |-> 0) > .

                match bottom {
                    Configuration::PgmConf(p) => {
                        match *p {
                            Pgm::Program(xl, s) => Configuration::StmtConf(s.into(), State::create_state(xl)),
                            _ => return None,
                        }
                    }
                    _ => return None,
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
                                            _ => return None,
                                        }
                                    },
                                    _ => return None,
                                }
                            },
                            _ => return None,
                        }   
                    }
                    _ => return None,
                }
            }
            _ => return None,
        };
        Some(x)
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