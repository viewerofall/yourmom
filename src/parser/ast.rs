use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: String,
    pub params: Vec<String>,
    pub body_tokens: Vec<String>, // token strings for simple text expansion
}

#[derive(Debug)]
pub struct Ast {
    pub functions: Vec<Function>,
    pub quantum_vars: HashSet<String>,
    pub float_quantum_vars: HashSet<String>,
    pub rational_quantum_vars: HashSet<String>,
    pub irrational_quantum_vars: HashSet<String>,
    pub imaginary_quantum_vars: HashSet<String>,
    pub heisenberg_vars: HashSet<String>,
    pub quaternion_vars: HashSet<String>,
    pub transfinite_vars: HashSet<String>,
    pub entangled_vars: HashSet<String>,
    pub lazy_vars: HashSet<String>,
    pub macros: Vec<MacroDef>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    PrintStr(String),
    PrintExpr(Expr),
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    // Schrodinger's if
    SchrodingerIf {
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Return(Expr),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    UnsafeBlock {
        asm_code: Vec<String>,
        body: Vec<Stmt>,
    },
    // goto / label
    GotoStmt { label: String },
    LabelStmt { name: String },
    // try-catch with setjmp/longjmp
    TryCatch {
        try_block: Vec<Stmt>,
        catch_block: Vec<Stmt>,
    },
    // uppercase print
    PrintLoud(String),
    // random assertion
    Paranoid(Expr),
    // rewrite_self
    RewriteSelf(String),
    // macro definition (top-level, also handled in Ast.macros)
    MacroDef {
        name: String,
        params: Vec<String>,
        body_tokens: Vec<String>,
    },
    // panic / abort with message
    Panic(String),
    PanicExpr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Superposition(Vec<Expr>),
    IntAll,
    RealAll,
    RationalAll,
    IrrationalAll,
    ImaginaryAll,
    HeisenbergOf(Vec<Expr>),         // heisenberg(a | b | c)
    QuaternionAll,                   // quaternion_all
    TransfiniteAll,                  // transfinite_all
    QuantumSort(Box<Expr>),          // quantum_sort(superposition)
    EntangledWith(String),           // entangled_with varname
    LazyExpr(Box<Expr>),             // yo_mama_so_lazy(expr)
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

pub enum AstNode {
    Function(Function),
    Stmt(Stmt),
    Expr(Expr),
}
