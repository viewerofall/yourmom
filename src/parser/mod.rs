mod ast;

pub use ast::{Ast, AstNode, BinOp, Expr, Function, MacroDef, Stmt};

use crate::lexer::{Token, TokenKind};
use crate::errors::{
    Result, YourMomError, Diagnostic, Severity,
    joke_for_unexpected_token, joke_for_unclosed_brace, joke_for_bad_function_name,
};
use std::collections::HashSet;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    filename: String,
    source: String,
    quantum_vars: HashSet<String>,
    float_quantum_vars: HashSet<String>,
    rational_quantum_vars: HashSet<String>,
    irrational_quantum_vars: HashSet<String>,
    imaginary_quantum_vars: HashSet<String>,
    heisenberg_vars: HashSet<String>,
    quaternion_vars: HashSet<String>,
    transfinite_vars: HashSet<String>,
    entangled_vars: HashSet<String>,
    lazy_vars: HashSet<String>,
    macros: Vec<MacroDef>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self::with_context(tokens, "<input>".to_string(), String::new())
    }

    pub fn with_context(tokens: Vec<Token>, filename: String, source: String) -> Self {
        Self {
            tokens,
            pos: 0,
            filename,
            source,
            quantum_vars: HashSet::new(),
            float_quantum_vars: HashSet::new(),
            rational_quantum_vars: HashSet::new(),
            irrational_quantum_vars: HashSet::new(),
            imaginary_quantum_vars: HashSet::new(),
            heisenberg_vars: HashSet::new(),
            quaternion_vars: HashSet::new(),
            transfinite_vars: HashSet::new(),
            entangled_vars: HashSet::new(),
            lazy_vars: HashSet::new(),
            macros: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast> {
        let mut functions = Vec::new();

        while !self.is_at_end() {
            match self.peek().kind.clone() {
                TokenKind::YoMamaSoMeta => {
                    let mdef = self.parse_macro_def()?;
                    self.macros.push(mdef);
                }
                TokenKind::Yo => functions.push(self.parse_function()?),
                _ => { self.advance(); }
            }
        }

        Ok(Ast {
            functions,
            quantum_vars: self.quantum_vars.clone(),
            float_quantum_vars: self.float_quantum_vars.clone(),
            rational_quantum_vars: self.rational_quantum_vars.clone(),
            irrational_quantum_vars: self.irrational_quantum_vars.clone(),
            imaginary_quantum_vars: self.imaginary_quantum_vars.clone(),
            heisenberg_vars: self.heisenberg_vars.clone(),
            quaternion_vars: self.quaternion_vars.clone(),
            transfinite_vars: self.transfinite_vars.clone(),
            entangled_vars: self.entangled_vars.clone(),
            lazy_vars: self.lazy_vars.clone(),
            macros: self.macros.clone(),
        })
    }

    // ── helpers: diagnostics ─────────────────────────────────────────────────

    fn make_diagnostic(
        &self,
        line: usize,
        column: usize,
        message: impl Into<String>,
        joke: impl Into<String>,
        help: Option<String>,
        note: Option<String>,
    ) -> YourMomError {
        YourMomError::Diagnostic(Box::new(Diagnostic {
            severity: Severity::Error,
            file: self.filename.clone(),
            line,
            column,
            message: message.into(),
            joke: joke.into(),
            help,
            note,
            source: self.source.clone(),
        }))
    }

    fn diag_unexpected(&self, expected: &str, got: &str, line: usize, col: usize) -> YourMomError {
        self.make_diagnostic(
            line, col,
            format!("expected {}, got {}", expected, got),
            joke_for_unexpected_token(expected, got),
            Some(format!("Try adding {} here", expected)),
            None,
        )
    }

    // ── macro definition (top-level) ─────────────────────────────────────────

    fn parse_macro_def(&mut self) -> Result<MacroDef> {
        self.expect(TokenKind::YoMamaSoMeta)?;

        let name = self.consume_identifier()?;
        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            params.push(self.consume_identifier()?);
            if matches!(self.peek().kind, TokenKind::Comma) { self.advance(); }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Eq)?;

        let mut body_tokens = Vec::new();
        loop {
            match self.peek().kind {
                TokenKind::Eof | TokenKind::YoMamaSoMeta | TokenKind::MamaMain | TokenKind::Yo => break,
                TokenKind::LBrace => break,
                _ => {
                    let t = self.advance();
                    body_tokens.push(format!("{:?}", t.kind));
                    if body_tokens.len() > 50 { break; }
                }
            }
        }

        Ok(MacroDef { name, params, body_tokens })
    }

    // ── function ─────────────────────────────────────────────────────────────

    fn parse_function(&mut self) -> Result<Function> {
        self.expect(TokenKind::Yo)?;
        let name = self.consume_function_name()?;
        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            params.push(self.consume_identifier()?);
            if matches!(self.peek().kind, TokenKind::Comma) { self.advance(); }
        }

        self.expect(TokenKind::RParen)?;
        let lbrace_tok = self.peek().clone();
        self.expect(TokenKind::LBrace)?;

        let mut body = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        if matches!(self.peek().kind, TokenKind::Eof) {
            let t = self.peek().clone();
            return Err(self.make_diagnostic(
                t.span.start.line, t.span.start.column,
                "expected '}', got end of file",
                joke_for_unclosed_brace(),
                Some("Add '}' at the end of the function body".to_string()),
                Some(format!("Opening '{{' was on line {}", lbrace_tok.span.start.line)),
            ));
        }

        self.expect(TokenKind::RBrace)?;
        Ok(Function { name, params, body })
    }

    // ── statements ───────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> Result<Option<Stmt>> {
        match self.peek().kind.clone() {
            TokenKind::Yo               => Ok(Some(self.parse_var_decl()?)),
            TokenKind::YoMamaSoFat      => Ok(Some(self.parse_print()?)),
            TokenKind::MamaSaid         => Ok(Some(self.parse_if()?)),
            TokenKind::MamaSaidMaybe    => Ok(Some(self.parse_schrodinger_if()?)),
            TokenKind::MamaKeepsSaying  => Ok(Some(self.parse_while()?)),
            TokenKind::DidYourMom       => Ok(Some(self.parse_return()?)),
            TokenKind::DoingYourMom     => Ok(Some(self.parse_call_stmt()?)),
            TokenKind::YoMamaSoUgly     => Ok(Some(self.parse_unsafe()?)),
            TokenKind::MamaForgot       => Ok(Some(self.parse_goto()?)),
            TokenKind::YoDaddyIssues    => Ok(Some(self.parse_try_catch()?)),
            TokenKind::YoMamaSoLoud     => Ok(Some(self.parse_print_loud()?)),
            TokenKind::YoMamaSoDumb     => Ok(Some(self.parse_panic()?)),
            TokenKind::YoMamaSoParanoid => Ok(Some(self.parse_paranoid()?)),
            TokenKind::YoMamaSoMeta     => Ok(Some(self.parse_rewrite_self_stmt()?)),
            // Identifier followed by Colon = label
            TokenKind::Identifier(_) if self.peek_ahead_is(1, &TokenKind::Colon) => {
                Ok(Some(self.parse_label()?))
            }
            TokenKind::Identifier(_) if self.peek_ahead_is(1, &TokenKind::Eq) => {
                Ok(Some(self.parse_assignment()?))
            }
            _ => { self.advance(); Ok(None) }
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::Yo)?;
        let name = self.consume_identifier()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expression()?;

        match &value {
            Expr::Superposition(_) | Expr::IntAll => {
                self.quantum_vars.insert(name.clone());
            }
            Expr::RealAll => {
                self.float_quantum_vars.insert(name.clone());
            }
            Expr::RationalAll => {
                self.rational_quantum_vars.insert(name.clone());
            }
            Expr::IrrationalAll => {
                self.irrational_quantum_vars.insert(name.clone());
            }
            Expr::ImaginaryAll => {
                self.imaginary_quantum_vars.insert(name.clone());
            }
            Expr::HeisenbergOf(_) => {
                self.heisenberg_vars.insert(name.clone());
            }
            Expr::QuaternionAll => {
                self.quaternion_vars.insert(name.clone());
            }
            Expr::TransfiniteAll => {
                self.transfinite_vars.insert(name.clone());
            }
            Expr::EntangledWith(_) => {
                self.entangled_vars.insert(name.clone());
            }
            Expr::LazyExpr(_) => {
                self.lazy_vars.insert(name.clone());
            }
            Expr::QuantumSort(_) => {
                self.quantum_vars.insert(name.clone());
            }
            _ => {}
        }

        Ok(Stmt::VarDecl { name, value })
    }

    fn parse_assignment(&mut self) -> Result<Stmt> {
        let name = self.consume_identifier()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Assignment { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoFat)?;
        self.expect(TokenKind::LParen)?;

        if let TokenKind::StringLit(s) = self.peek().kind.clone() {
            self.advance();
            self.expect(TokenKind::RParen)?;
            return Ok(Stmt::PrintStr(s));
        }

        let expr = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        Ok(Stmt::PrintExpr(expr))
    }

    fn parse_print_loud(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoLoud)?;
        self.expect(TokenKind::LParen)?;
        if let TokenKind::StringLit(s) = self.peek().kind.clone() {
            self.advance();
            self.expect(TokenKind::RParen)?;
            return Ok(Stmt::PrintLoud(s));
        }
        let t = self.peek().clone();
        Err(self.make_diagnostic(
            t.span.start.line, t.span.start.column,
            "yo_mama_so_loud expects a string literal argument",
            "Yo mama so loud she couldn't even figure out what to yell",
            Some("Pass a string literal like yo_mama_so_loud(\"HELLO WORLD\")".to_string()),
            None,
        ))
    }

    fn parse_panic(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoDumb)?;
        self.expect(TokenKind::LParen)?;
        if let TokenKind::StringLit(s) = self.peek().kind.clone() {
            self.advance();
            self.expect(TokenKind::RParen)?;
            return Ok(Stmt::Panic(s));
        }
        let expr = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        Ok(Stmt::PanicExpr(expr))
    }

    fn parse_paranoid(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoParanoid)?;
        self.expect(TokenKind::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        Ok(Stmt::Paranoid(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::MamaSaid)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let then_block = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        let mut else_block = Vec::new();
        if matches!(self.peek().kind, TokenKind::MamaLied) {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            else_block = self.parse_block()?;
            self.expect(TokenKind::RBrace)?;
        }

        Ok(Stmt::If { condition, then_block, else_block })
    }

    fn parse_schrodinger_if(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::MamaSaidMaybe)?;
        self.expect(TokenKind::LBrace)?;
        let then_block = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        let mut else_block = Vec::new();
        if matches!(self.peek().kind, TokenKind::MamaLied) {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            else_block = self.parse_block()?;
            self.expect(TokenKind::RBrace)?;
        }

        Ok(Stmt::SchrodingerIf { then_block, else_block })
    }

    fn parse_while(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::MamaKeepsSaying)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_return(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::DidYourMom)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Return(value))
    }

    fn parse_call_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::DoingYourMom)?;
        let name = self.consume_identifier()?;
        self.expect(TokenKind::LParen)?;
        let args = self.parse_args()?;
        self.expect(TokenKind::RParen)?;
        Ok(Stmt::FunctionCall { name, args })
    }

    fn parse_unsafe(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoUgly)?;
        self.expect(TokenKind::LBrace)?;

        let mut asm_code = Vec::new();
        let mut body = Vec::new();

        while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
            if let TokenKind::Identifier(ref s) = self.peek().kind.clone() {
                if s == "__asm__" {
                    self.advance();
                    self.expect(TokenKind::LParen)?;
                    if let TokenKind::StringLit(code) = self.advance().kind {
                        asm_code.push(code);
                    }
                    self.expect(TokenKind::RParen)?;
                    continue;
                }
            }
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(Stmt::UnsafeBlock { asm_code, body })
    }

    fn parse_goto(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::MamaForgot)?;
        let label = self.consume_identifier()?;
        Ok(Stmt::GotoStmt { label })
    }

    fn parse_label(&mut self) -> Result<Stmt> {
        let name = self.consume_identifier()?;
        self.expect(TokenKind::Colon)?;
        Ok(Stmt::LabelStmt { name })
    }

    fn parse_try_catch(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoDaddyIssues)?;
        self.expect(TokenKind::LBrace)?;
        let try_block = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;

        let mut catch_block = Vec::new();
        if matches!(self.peek().kind, TokenKind::MamaCaught) {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            catch_block = self.parse_block()?;
            self.expect(TokenKind::RBrace)?;
        }

        Ok(Stmt::TryCatch { try_block, catch_block })
    }

    fn parse_rewrite_self_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::YoMamaSoMeta)?;
        let name = self.consume_identifier()?;
        if name == "rewrite_self" {
            self.expect(TokenKind::LParen)?;
            if let TokenKind::StringLit(path) = self.peek().kind.clone() {
                self.advance();
                self.expect(TokenKind::RParen)?;
                return Ok(Stmt::RewriteSelf(path));
            }
            let t = self.peek().clone();
            return Err(self.make_diagnostic(
                t.span.start.line, t.span.start.column,
                "rewrite_self expects a string path argument",
                "Yo mama so meta she couldn't even find herself",
                Some("Use rewrite_self(\"my_program.yourmom\")".to_string()),
                None,
            ));
        }
        Ok(Stmt::GotoStmt { label: "__noop__".to_string() })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
            if let Some(s) = self.parse_statement()? {
                stmts.push(s);
            }
        }
        Ok(stmts)
    }

    // ── expressions ──────────────────────────────────────────────────────────

    fn parse_expression(&mut self) -> Result<Expr> {
        let left = self.parse_primary()?;

        if matches!(self.peek().kind, TokenKind::Pipe) {
            let mut values = vec![left];
            while matches!(self.peek().kind, TokenKind::Pipe) {
                self.advance();
                values.push(self.parse_primary()?);
            }
            return Ok(Expr::Superposition(values));
        }

        self.parse_binop(left)
    }

    fn parse_binop(&mut self, mut left: Expr) -> Result<Expr> {
        loop {
            let op = match self.peek().kind {
                TokenKind::Plus    => BinOp::Add,
                TokenKind::Minus   => BinOp::Sub,
                TokenKind::Star    => BinOp::Mul,
                TokenKind::Slash   => BinOp::Div,
                TokenKind::Percent => BinOp::Mod,
                TokenKind::EqEq    => BinOp::Eq,
                TokenKind::NotEq   => BinOp::NotEq,
                TokenKind::Lt      => BinOp::Lt,
                TokenKind::Gt      => BinOp::Gt,
                TokenKind::LtEq    => BinOp::LtEq,
                TokenKind::GtEq    => BinOp::GtEq,
                _                  => break,
            };
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let tok = self.peek().clone();
        match tok.kind.clone() {
            TokenKind::StringLit(s)  => { self.advance(); Ok(Expr::String(s)) }
            TokenKind::Number(n)     => { self.advance(); Ok(Expr::Number(n)) }
            // Unary minus: -expr
            TokenKind::Minus => {
                self.advance();
                let inner = self.parse_primary()?;
                // Fold constant negative numbers
                if let Expr::Number(n) = inner {
                    return Ok(Expr::Number(-n));
                }
                Ok(Expr::BinaryOp {
                    op: BinOp::Sub,
                    left: Box::new(Expr::Number(0)),
                    right: Box::new(inner),
                })
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::DoingYourMom => {
                self.advance();
                let name = self.consume_identifier()?;
                self.expect(TokenKind::LParen)?;
                let args = self.parse_args()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expr::FunctionCall { name, args })
            }
            TokenKind::IntAll        => { self.advance(); Ok(Expr::IntAll) }
            TokenKind::RealAll       => { self.advance(); Ok(Expr::RealAll) }
            TokenKind::RationalAll   => { self.advance(); Ok(Expr::RationalAll) }
            TokenKind::IrrationalAll => { self.advance(); Ok(Expr::IrrationalAll) }
            TokenKind::ImaginaryAll  => { self.advance(); Ok(Expr::ImaginaryAll) }
            TokenKind::QuaternionAll => { self.advance(); Ok(Expr::QuaternionAll) }
            TokenKind::TransfiniteAll => { self.advance(); Ok(Expr::TransfiniteAll) }

            TokenKind::Heisenberg => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let mut values = Vec::new();
                values.push(self.parse_primary()?);
                while matches!(self.peek().kind, TokenKind::Pipe) {
                    self.advance();
                    values.push(self.parse_primary()?);
                }
                self.expect(TokenKind::RParen)?;
                Ok(Expr::HeisenbergOf(values))
            }

            TokenKind::QuantumSort => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let inner = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expr::QuantumSort(Box::new(inner)))
            }

            TokenKind::EntangledWith => {
                self.advance();
                let var_name = self.consume_identifier()?;
                Ok(Expr::EntangledWith(var_name))
            }

            TokenKind::YoMamaSoLazy => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let inner = self.parse_expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expr::LazyExpr(Box::new(inner)))
            }

            TokenKind::Identifier(name) => {
                self.advance();
                if matches!(self.peek().kind, TokenKind::LParen) {
                    let macro_name = name.clone();
                    let macro_exists = self.macros.iter().any(|m| m.name == macro_name);
                    if macro_exists {
                        self.advance();
                        let args = self.parse_args()?;
                        self.expect(TokenKind::RParen)?;
                        return Ok(Expr::FunctionCall { name: macro_name, args });
                    }
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(TokenKind::RParen)?;
                    return Ok(Expr::FunctionCall { name, args });
                }
                Ok(Expr::Variable(name))
            }

            _ => {
                let got = format!("{:?}", tok.kind);
                Err(self.make_diagnostic(
                    tok.span.start.line, tok.span.start.column,
                    format!("unexpected token in expression: {}", got),
                    format!("Yo mama so confused she put a {} where an expression goes", got),
                    Some("Check your syntax — an expression should start with a value, variable, or function call".to_string()),
                    None,
                ))
            }
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            args.push(self.parse_expression()?);
            if matches!(self.peek().kind, TokenKind::Comma) { self.advance(); }
        }
        Ok(args)
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn consume_function_name(&mut self) -> Result<String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::MamaMain      => Ok("mama_main".to_string()),
            TokenKind::Identifier(s) => Ok(s),
            other => {
                let got = format!("{:?}", other);
                Err(self.make_diagnostic(
                    tok.span.start.line, tok.span.start.column,
                    "expected function name after 'yo'",
                    joke_for_bad_function_name(&got),
                    Some("Function names should be identifiers like 'my_func' or 'mama_main'".to_string()),
                    None,
                ))
            }
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Identifier(s) => Ok(s),
            other => {
                let got = format!("{:?}", other);
                Err(self.diag_unexpected("identifier", &got, tok.span.start.line, tok.span.start.column))
            }
        }
    }

    fn peek_ahead_is(&self, offset: usize, kind: &TokenKind) -> bool {
        let idx = self.pos + offset;
        if idx >= self.tokens.len() { return false; }
        std::mem::discriminant(&self.tokens[idx].kind) == std::mem::discriminant(kind)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: TokenKind) -> Result<()> {
        let tok = self.advance();
        if std::mem::discriminant(&tok.kind) != std::mem::discriminant(&expected) {
            let got = format!("{:?}", tok.kind);
            let exp = format!("{:?}", expected);
            return Err(self.diag_unexpected(&exp, &got, tok.span.start.line, tok.span.start.column));
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}
