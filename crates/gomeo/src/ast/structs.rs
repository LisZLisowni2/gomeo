pub struct Program {
    pub functions: Vec<FunctionalDecl>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub struct FunctionalDecl {
    pub name: String,
    pub return_type: crate::types::Type,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(String),
    Integer32Literal(String),
    VoidLiteral(),
    Call {
        callee: String,
        arguments: Vec<Expression>,
    },
}
