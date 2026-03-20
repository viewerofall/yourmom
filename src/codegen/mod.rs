use std::collections::HashSet;
use crate::parser::{Ast, BinOp, Expr, Function, MacroDef, Stmt};
use crate::errors::{Result, YourMomError};

pub struct Codegen {
    output: String,
    indent: usize,
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
    lazy_counter: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
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
            lazy_counter: 0,
        }
    }

    pub fn generate(&mut self, ast: &Ast) -> Result<String> {
        self.output.push_str("#include \"quantum_runtime.h\"\n\n");

        self.quantum_vars       = ast.quantum_vars.clone();
        self.float_quantum_vars = ast.float_quantum_vars.clone();
        self.rational_quantum_vars    = ast.rational_quantum_vars.clone();
        self.irrational_quantum_vars  = ast.irrational_quantum_vars.clone();
        self.imaginary_quantum_vars   = ast.imaginary_quantum_vars.clone();
        self.heisenberg_vars    = ast.heisenberg_vars.clone();
        self.quaternion_vars    = ast.quaternion_vars.clone();
        self.transfinite_vars   = ast.transfinite_vars.clone();
        self.entangled_vars     = ast.entangled_vars.clone();
        self.lazy_vars          = ast.lazy_vars.clone();
        self.macros             = ast.macros.clone();

        // Emit lazy compute functions before main
        for func in &ast.functions {
            self.emit_lazy_functions_for_body(&func.body)?;
        }

        for func in &ast.functions {
            self.gen_function(func)?;
        }

        Ok(self.output.clone())
    }

    // Pre-pass: emit _lazy_compute_X() for each lazy var
    fn emit_lazy_functions_for_body(&mut self, stmts: &[Stmt]) -> Result<()> {
        for stmt in stmts {
            if let Stmt::VarDecl { name, value: Expr::LazyExpr(inner) } = stmt {
                let expr_str = self.gen_expr(inner)?;
                self.emit(&format!("int _lazy_compute_{}() {{ return {}; }}", name, expr_str));
            }
            // recurse into nested blocks
            match stmt {
                Stmt::If { then_block, else_block, .. } => {
                    self.emit_lazy_functions_for_body(then_block)?;
                    self.emit_lazy_functions_for_body(else_block)?;
                }
                Stmt::SchrodingerIf { then_block, else_block } => {
                    self.emit_lazy_functions_for_body(then_block)?;
                    self.emit_lazy_functions_for_body(else_block)?;
                }
                Stmt::While { body, .. } => {
                    self.emit_lazy_functions_for_body(body)?;
                }
                Stmt::TryCatch { try_block, catch_block } => {
                    self.emit_lazy_functions_for_body(try_block)?;
                    self.emit_lazy_functions_for_body(catch_block)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    // ── function ────────────────────────────────────────────────────────────

    fn gen_function(&mut self, func: &Function) -> Result<()> {
        let func_name = if func.name == "mama_main" {
            "main".to_string()
        } else {
            func.name.clone()
        };

        let params = func.params.iter()
            .map(|p| format!("int {}", p))
            .collect::<Vec<_>>()
            .join(", ");

        self.emit(&format!("int {}({}) {{", func_name, params));
        self.indent += 1;

        if func_name == "main" {
            self.emit("_quantum_init_errors();");
        }

        for stmt in &func.body {
            self.gen_statement(stmt)?;
        }

        if func_name == "main" {
            self.emit("return 0;");
        }

        self.indent -= 1;
        self.emit("}\n");
        Ok(())
    }

    // ── statements ──────────────────────────────────────────────────────────

    fn gen_statement(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::VarDecl { name, value } => {
                self.gen_var_decl(name, value)?;
            }

            Stmt::Assignment { name, value } => {
                if self.lazy_vars.contains(name) {
                    // reassign lazy val
                    let expr = self.gen_expr(value)?;
                    self.emit(&format!("{}.val = {}; {}.computed = 1;", name, expr, name));
                } else {
                    let expr = self.gen_expr(value)?;
                    self.emit(&format!("{} = {};", name, expr));
                }
            }

            Stmt::PrintStr(s) => {
                self.emit(&format!("printf(\"{}\\n\");", s));
            }

            Stmt::PrintExpr(expr) => {
                self.gen_print_expr(expr)?;
            }

            Stmt::PrintLoud(s) => {
                self.emit(&format!("_quantum_shout(\"{}\");", s));
            }

            Stmt::If { condition, then_block, else_block } => {
                let cond = self.gen_expr(condition)?;
                self.emit(&format!("if ({}) {{", cond));
                self.indent += 1;
                for s in then_block { self.gen_statement(s)?; }
                self.indent -= 1;
                if !else_block.is_empty() {
                    self.emit("} else {");
                    self.indent += 1;
                    for s in else_block { self.gen_statement(s)?; }
                    self.indent -= 1;
                }
                self.emit("}");
            }

            Stmt::SchrodingerIf { then_block, else_block } => {
                self.emit("if (rand() % 2) {");
                self.indent += 1;
                for s in then_block { self.gen_statement(s)?; }
                self.indent -= 1;
                self.emit("} else {");
                self.indent += 1;
                for s in else_block { self.gen_statement(s)?; }
                self.indent -= 1;
                self.emit("}");
            }

            Stmt::While { condition, body } => {
                let cond = self.gen_expr(condition)?;
                self.emit(&format!("while ({}) {{", cond));
                self.indent += 1;
                for s in body { self.gen_statement(s)?; }
                self.indent -= 1;
                self.emit("}");
            }

            Stmt::Return(expr) => {
                let expr_str = self.gen_expr(expr)?;
                self.emit(&format!("return {};", expr_str));
            }

            Stmt::FunctionCall { name, args } => {
                let args_str = args.iter()
                    .map(|a| self.gen_expr(a))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                self.emit(&format!("{}({});", name, args_str));
            }

            Stmt::UnsafeBlock { asm_code, body } => {
                self.emit("// UNSAFE BLOCK - yo_mama_so_ugly");
                for code in asm_code {
                    self.emit(&format!("__asm__(\"{}\");", code));
                }
                for s in body { self.gen_statement(s)?; }
            }

            Stmt::GotoStmt { label } => {
                if label != "__noop__" {
                    self.emit(&format!("goto {};", label));
                }
            }

            Stmt::LabelStmt { name } => {
                // Labels can't be indented normally in C — emit at indent 0
                self.output.push_str(&format!("{}:\n", name));
            }

            Stmt::TryCatch { try_block, catch_block } => {
                self.emit("{");
                self.indent += 1;
                self.emit("if (_daddy_issues_depth >= _DADDY_ISSUES_MAX_DEPTH) {");
                self.indent += 1;
                self.emit("fprintf(stderr, \"holy shitballs, try-catch stack overflow\\n\"); exit(1);");
                self.indent -= 1;
                self.emit("}");
                self.emit("int _daddy_idx = _daddy_issues_depth++;");
                self.emit("if (setjmp(_daddy_issues_stack[_daddy_idx]) == 0) {");
                self.indent += 1;
                for s in try_block { self.gen_statement(s)?; }
                self.indent -= 1;
                self.emit("} else {");
                self.indent += 1;
                for s in catch_block { self.gen_statement(s)?; }
                self.indent -= 1;
                self.emit("}");
                self.emit("_daddy_issues_depth--;");
                self.indent -= 1;
                self.emit("}");
            }

            Stmt::Paranoid(expr) => {
                let cond = self.gen_expr(expr)?;
                self.emit(&format!(
                    "if (rand() % 2 && !({cond})) {{ fprintf(stderr, \"PARANOID SHIT FAILED: assertion '{}' is fucked\\n\"); exit(1); }}",
                    cond.replace('"', "\\\""),
                    cond = cond,
                ));
            }

            Stmt::RewriteSelf(path) => {
                self.emit(&format!("_quantum_rewrite_source(\"{}\");", path));
            }

            Stmt::MacroDef { .. } => {
                // macro defs at statement level are no-ops in codegen
            }

            Stmt::Panic(s) => {
                self.emit(&format!("yo_mama_so_dumb(\"{}\");", s));
            }

            Stmt::PanicExpr(expr) => {
                let e = self.gen_expr(expr)?;
                self.emit(&format!("yo_mama_so_dumb({});", e));
            }
        }
        Ok(())
    }

    fn gen_var_decl(&mut self, name: &str, value: &Expr) -> Result<()> {
        match value {
            Expr::RealAll => {
                let expr = self.gen_expr(value)?;
                self.emit(&format!("quantum_float {} = {};", name, expr));
                self.float_quantum_vars.insert(name.to_string());
            }
            Expr::IntAll => {
                let expr = self.gen_expr(value)?;
                self.emit(&format!("quantum_int {} = {};", name, expr));
                self.quantum_vars.insert(name.to_string());
            }
            Expr::RationalAll => {
                let expr = self.gen_expr(value)?;
                self.emit(&format!("quantum_rational {} = {};", name, expr));
                self.rational_quantum_vars.insert(name.to_string());
            }
            Expr::IrrationalAll => {
                let expr = self.gen_expr(value)?;
                self.emit(&format!("quantum_irrational {} = {};", name, expr));
                self.irrational_quantum_vars.insert(name.to_string());
            }
            Expr::ImaginaryAll => {
                let expr = self.gen_expr(value)?;
                self.emit(&format!("quantum_complex {} = {};", name, expr));
                self.imaginary_quantum_vars.insert(name.to_string());
            }
            Expr::HeisenbergOf(vals) => {
                let val_strs = vals.iter()
                    .map(|v| self.gen_expr(v))
                    .collect::<Result<Vec<_>>>()?;
                self.emit(&format!(
                    "quantum_heisenberg {} = HEISENBERG({});",
                    name,
                    val_strs.join(", ")
                ));
                self.heisenberg_vars.insert(name.to_string());
            }
            Expr::QuaternionAll => {
                self.emit(&format!("quantum_quaternion {} = QUATERNION_ALL;", name));
                self.quaternion_vars.insert(name.to_string());
            }
            Expr::TransfiniteAll => {
                self.emit(&format!("quantum_transfinite {} = TRANSFINITE_ALL;", name));
                self.transfinite_vars.insert(name.to_string());
            }
            Expr::EntangledWith(other) => {
                self.emit(&format!("quantum_int* {} = &{};", name, other));
                self.entangled_vars.insert(name.to_string());
            }
            Expr::LazyExpr(_) => {
                // lazy_compute function already emitted before main
                self.emit(&format!("lazy_int {} = LAZY_INT_INIT;", name));
                self.lazy_vars.insert(name.to_string());
            }
            Expr::QuantumSort(_) => {
                // quantum_sort prints inline and returns quantum_int
                let expr_str = self.gen_expr(value)?;
                self.emit(&format!("quantum_int {} = {};", name, expr_str));
                self.quantum_vars.insert(name.to_string());
            }
            _ => {
                let is_quantum = self.is_quantum_expr(value);
                let expr = self.gen_expr(value)?;
                if is_quantum {
                    self.emit(&format!("quantum_int {} = {};", name, expr));
                    self.quantum_vars.insert(name.to_string());
                } else {
                    self.emit(&format!("int {} = {};", name, expr));
                }
            }
        }
        Ok(())
    }

    fn gen_print_expr(&mut self, expr: &Expr) -> Result<()> {
        if let Expr::Variable(name) = expr {
            if self.float_quantum_vars.contains(name) {
                self.emit(&format!("printf(\"%g\\n\", _quantum_float_get(&{}));", name));
                return Ok(());
            }
            if self.heisenberg_vars.contains(name) {
                self.emit(&format!("printf(\"%d\\n\", _quantum_heisenberg_get(&{}));", name));
                return Ok(());
            }
            if self.quantum_vars.contains(name) {
                self.emit(&format!("printf(\"%d\\n\", _quantum_get(&{}));", name));
                return Ok(());
            }
            if self.rational_quantum_vars.contains(name) {
                self.emit(&format!("_quantum_rational_print(&{});", name));
                return Ok(());
            }
            if self.irrational_quantum_vars.contains(name) {
                self.emit(&format!("_quantum_irrational_print(&{});", name));
                return Ok(());
            }
            if self.imaginary_quantum_vars.contains(name) {
                self.emit(&format!("_quantum_complex_print(&{});", name));
                return Ok(());
            }
            if self.quaternion_vars.contains(name) {
                self.emit(&format!("_quantum_quaternion_print(&{});", name));
                return Ok(());
            }
            if self.transfinite_vars.contains(name) {
                self.emit(&format!("_quantum_transfinite_print(&{});", name));
                return Ok(());
            }
            if self.entangled_vars.contains(name) {
                self.emit(&format!("printf(\"%d\\n\", _quantum_get({}));", name));
                return Ok(());
            }
            if self.lazy_vars.contains(name) {
                self.emit(&format!(
                    "printf(\"%d\\n\", LAZY_GET({}, _lazy_compute_{}()));",
                    name, name
                ));
                return Ok(());
            }
        }
        let expr_str = self.gen_expr(expr)?;
        self.emit(&format!("printf(\"%d\\n\", {});", expr_str));
        Ok(())
    }

    // ── expressions ─────────────────────────────────────────────────────────

    fn gen_expr(&mut self, expr: &Expr) -> Result<String> {
        match expr {
            Expr::Number(n) => Ok(n.to_string()),
            Expr::String(s) => Ok(format!("\"{}\"", s)),
            Expr::Variable(name) => {
                if self.entangled_vars.contains(name) {
                    return Ok(format!("_quantum_get({})", name));
                }
                if self.lazy_vars.contains(name) {
                    return Ok(format!("LAZY_GET({}, _lazy_compute_{}())", name, name));
                }
                if self.heisenberg_vars.contains(name) {
                    return Ok(format!("_quantum_heisenberg_get(&{})", name));
                }
                Ok(name.clone())
            }

            Expr::IntAll        => Ok("INT_ALL".to_string()),
            Expr::RealAll       => Ok("REAL_ALL".to_string()),
            Expr::RationalAll   => Ok("RATIONAL_ALL".to_string()),
            Expr::IrrationalAll => Ok("IRRATIONAL_ALL".to_string()),
            Expr::ImaginaryAll  => Ok("IMAGINARY_ALL".to_string()),
            Expr::QuaternionAll => Ok("QUATERNION_ALL".to_string()),
            Expr::TransfiniteAll => Ok("TRANSFINITE_ALL".to_string()),

            Expr::HeisenbergOf(vals) => {
                let val_strs = vals.iter()
                    .map(|v| self.gen_expr(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("HEISENBERG({})", val_strs.join(", ")))
            }

            Expr::QuantumSort(inner) => {
                match inner.as_ref() {
                    Expr::Superposition(vals) => {
                        let val_strs = vals.iter()
                            .map(|v| self.gen_expr(v))
                            .collect::<Result<Vec<_>>>()?;
                        Ok(format!("QUANTUM_SORT({})", val_strs.join(", ")))
                    }
                    _ => {
                        let v = self.gen_expr(inner)?;
                        Ok(format!("QUANTUM_SORT({})", v))
                    }
                }
            }

            Expr::EntangledWith(other) => Ok(format!("&{}", other)),

            Expr::LazyExpr(inner) => self.gen_expr(inner),

            Expr::Superposition(values) => {
                let vals = values.iter()
                    .map(|v| self.gen_expr(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("SUPERPOSITION({})", vals.join(", ")))
            }

            Expr::BinaryOp { op, left, right } => {
                let lq = self.is_quantum_var(left);
                let rq = self.is_quantum_var(right);

                if lq || rq {
                    return self.gen_quantum_binop(op, left, right, lq, rq);
                }

                let l = self.gen_expr(left)?;
                let r = self.gen_expr(right)?;
                let op_str = match op {
                    BinOp::Add   => "+",   BinOp::Sub   => "-",
                    BinOp::Mul   => "*",   BinOp::Div   => "/",
                    BinOp::Mod   => "%",   BinOp::Eq    => "==",
                    BinOp::NotEq => "!=",  BinOp::Lt    => "<",
                    BinOp::Gt    => ">",   BinOp::LtEq  => "<=",
                    BinOp::GtEq  => ">=",
                };
                Ok(format!("({} {} {})", l, op_str, r))
            }

            Expr::FunctionCall { name, args } => {
                let args_str = args.iter()
                    .map(|a| self.gen_expr(a))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                Ok(format!("{}({})", name, args_str))
            }
        }
    }

    fn gen_quantum_binop(
        &mut self,
        op: &BinOp,
        left: &Expr,
        right: &Expr,
        lq: bool,
        rq: bool,
    ) -> Result<String> {
        match op {
            BinOp::Eq | BinOp::Lt | BinOp::Gt => {
                if lq && !rq {
                    let lname = self.var_name(left)?;
                    let r = self.gen_expr(right)?;
                    Ok(match op {
                        BinOp::Eq => format!("_quantum_eq_val(&{}, {})", lname, r),
                        BinOp::Lt => format!("_quantum_lt_val(&{}, {})", lname, r),
                        BinOp::Gt => format!("_quantum_gt_val(&{}, {})", lname, r),
                        _ => unreachable!(),
                    })
                } else if rq && !lq {
                    let rname = self.var_name(right)?;
                    let l = self.gen_expr(left)?;
                    Ok(match op {
                        BinOp::Eq => format!("_quantum_eq_val(&{}, {})", rname, l),
                        BinOp::Gt => format!("_quantum_lt_val(&{}, {})", rname, l),
                        BinOp::Lt => format!("_quantum_gt_val(&{}, {})", rname, l),
                        _ => unreachable!(),
                    })
                } else {
                    let lname = self.var_name(left)?;
                    let rname = self.var_name(right)?;
                    Ok(match op {
                        BinOp::Eq => format!("_quantum_eq(&{}, &{})", lname, rname),
                        BinOp::Lt => format!("_quantum_lt(&{}, &{})", lname, rname),
                        BinOp::Gt => format!("_quantum_gt(&{}, &{})", lname, rname),
                        _ => unreachable!(),
                    })
                }
            }

            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                let l_expr = self.quantum_wrap(left, lq)?;
                let r_expr = self.quantum_wrap(right, rq)?;
                Ok(match op {
                    BinOp::Add => format!("_quantum_add({}, {})", l_expr, r_expr),
                    BinOp::Sub => format!("_quantum_sub({}, {})", l_expr, r_expr),
                    BinOp::Mul => format!("_quantum_mul({}, {})", l_expr, r_expr),
                    BinOp::Div => format!("_quantum_div({}, {})", l_expr, r_expr),
                    _ => unreachable!(),
                })
            }

            _ => {
                let l = self.gen_expr(left)?;
                let r = self.gen_expr(right)?;
                let op_str = match op {
                    BinOp::Mod   => "%",  BinOp::NotEq => "!=",
                    BinOp::LtEq  => "<=", BinOp::GtEq  => ">=",
                    _ => unreachable!(),
                };
                Ok(format!("({} {} {})", l, op_str, r))
            }
        }
    }

    // ── utilities ────────────────────────────────────────────────────────────

    fn var_name<'a>(&self, expr: &'a Expr) -> Result<&'a str> {
        match expr {
            Expr::Variable(n) => Ok(n.as_str()),
            _ => Err(YourMomError::SyntaxError("expected variable in quantum op, you dumb shit".into())),
        }
    }

    fn quantum_wrap(&mut self, expr: &Expr, is_quantum: bool) -> Result<String> {
        if is_quantum {
            if let Expr::Variable(n) = expr {
                return Ok(format!("&{}", n));
            }
        }
        let v = self.gen_expr(expr)?;
        Ok(format!(
            "&(quantum_int){{.values=(int[]){{{}}}, .count=1, .collapsed_idx=0, .collapsed_val={}}}",
            v, v
        ))
    }

    fn is_quantum_var(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Variable(n) if self.quantum_vars.contains(n))
    }

    fn is_quantum_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Superposition(_)             => true,
            Expr::Variable(n)                  => self.quantum_vars.contains(n),
            Expr::BinaryOp { left, right, .. } => {
                self.is_quantum_expr(left) || self.is_quantum_expr(right)
            }
            _ => false,
        }
    }

    fn emit(&mut self, code: &str) {
        self.output.push_str(&"    ".repeat(self.indent));
        self.output.push_str(code);
        self.output.push('\n');
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}
