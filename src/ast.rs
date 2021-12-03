
enum State {
    Lookup(Box<State>, String),
    Update(Box<State>, u32, String),
}

enum Pgm {
    Program(Vec<String>, Stmt),
}

enum Configuration {
    AExpConf(AExp, State),
    BExpConf(BExp, State),
    StmtConf(Stmt, State),
    PgmConf(Pgm),
}

enum AExp {
    Plus(Box<AExp>, Box<AExp>),
    Divide(Box<AExp>, Box<AExp>),
    Id(String),
}

enum BExp {
    LessThanEq(Box<AExp>, Box<AExp>),
    Negation(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Bool(bool),
}

enum Stmt {
    StmtBlock(Block),
    Assign(String, AExp),
    Sequence(Box<Stmt>, Box<Stmt>),
    IfThenElse(BExp, Block, Block),
    While(BExp, Block),
}

enum Block {
    EmptyBlock,
    BlockStmt(Box<Stmt>),
}


struct Stack {
    stack: Vec<Configuration>,
    rules: Vec<Rules>,
}

impl Stack {
    fn applyRule(&mut self, rule: Rules) {
        //self.stack.

    }
}

enum Rules {
    RewritePlusLeft,
    RewritePlusRight,
    RewritePlus, 

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