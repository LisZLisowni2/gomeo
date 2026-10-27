pub struct FunctionSymbol {
    pub name: String,
    pub return_type: TypeSymbol,
    pub arguments: Vec<FunctionArgument>,
}

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeSymbol,
}

pub enum TypeSymbol {
    Integer32,
    String,
    Void,
}
