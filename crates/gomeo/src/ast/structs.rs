#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionalDecl>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionalDecl {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: crate::types::Type,
    pub body: Block,
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub name: String,
    pub arg_type: crate::types::Type,
}

#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(String),
    Integer32Literal(String),
    VoidLiteral(),
    VariableLiteral(String),
    Call {
        callee: String,
        arguments: Vec<Expression>,
    },
}
