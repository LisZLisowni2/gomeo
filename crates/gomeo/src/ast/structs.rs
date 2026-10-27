pub struct Program {
    pub functions: Vec<FunctionalDecl>,
}

pub enum Statement {
    Expression(Expression),
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub struct FunctionalDecl {
    pub name: String,
    pub body: Block,
}

pub enum Expression {
    StringLiteral(String),
    Call {
        callee: String,
        arguments: Vec<Expression>,
    },
}
