use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn dummy() -> Self {
        let pos = Position { line: 0, column: 0, offset: 0 };
        Span { start: pos, end: pos }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Yo,
    YoMamaSoFat,       // println
    YoMamaSoDumb,      // panic
    YoMamaSoUgly,      // unsafe
    MamaMain,
    MamaSaid,          // if
    MamaLied,          // else
    MamaKeepsSaying,   // while
    DidYourMom,        // return
    DoingYourMom,      // call
    YoDaddy,           // struct (future)

    // Quantum types
    YoMamaInTwoPlaces, // superposition keyword
    IntAll,            // int_all
    RealAll,           // real_all
    RationalAll,       // rational_all
    IrrationalAll,     // irrational_all
    ImaginaryAll,      // imaginary_all

    // New quantum / exotic types
    Heisenberg,        // heisenberg(...)
    QuaternionAll,     // quaternion_all
    TransfiniteAll,    // transfinite_all
    QuantumSort,       // quantum_sort(...)
    EntangledWith,     // entangled_with

    // New control flow
    MamaSaidMaybe,     // Schrodinger's if
    MamaForgot,        // goto
    YoDaddyIssues,     // try
    MamaCaught,        // catch

    // New print / assert
    YoMamaSoLoud,      // uppercase print
    YoMamaSoParanoid,  // random assertion

    // Lazy
    YoMamaSoLazy,      // lazy evaluation

    // Meta / macros
    YoMamaSoMeta,      // macro definition / rewrite_self

    // Literals
    Identifier(String),
    StringLit(String),
    Number(i64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Pipe,  // | for quantum superposition

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn dummy(kind: TokenKind) -> Self {
        Self { kind, span: Span::dummy() }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "Identifier({})", s),
            TokenKind::StringLit(s)  => write!(f, "String(\"{}\")", s),
            TokenKind::Number(n)     => write!(f, "Number({})", n),
            _ => write!(f, "{:?}", self),
        }
    }
}
