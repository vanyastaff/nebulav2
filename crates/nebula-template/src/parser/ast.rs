#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Template(Vec<TemplateElement>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateElement {
    Text(String),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    String(String),
    Number(f64),
    Boolean(bool),
    Null,

    // Data access
    DataAccess {
        source: DataSource,
        path: Vec<String>,
    },

    // Function calls and pipelines
    Pipeline {
        input: Box<Expr>,
        functions: Vec<FunctionCall>,
    },

    Function {
        name: String,
        args: Vec<Expr>,
    },

    // Control flow
    IfElse {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Option<Box<Expr>>,
    },

    ForEach {
        iterator: String,
        iterable: Box<Expr>,
        body: Box<Expr>,
        separator: Option<String>,
    },

    // Operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        operator: UnaryOp,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}