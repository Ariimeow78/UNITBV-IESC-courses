// src/codegen.rs
use crate::ast::{Expr, Program, Stmt, BinOp as ABinOp};
use crate::tac::{Addr, BinOp as TBinOp, Instr};
use std::collections::HashMap;

pub struct CodegenStats {
    pub temporare: usize,
    pub etichete: usize,
    pub salturi: usize,
    pub aritmetice: usize,
}

pub struct CodeGen {
    pub instrs: Vec<Instr>,                 // instructiunile generate pana acum
    temp_count: usize,                      // contor pentru temporare: t0, t1, t2...
    label_count: usize,                     // contor pentru etichete: L0, L1, L2...
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen { instrs: Vec::new(), temp_count: 0, label_count: 0 }
    }

    /// Aloca un temporar nou: t0, t1, t2...
    fn new_temp(&mut self) -> Addr {
        let n = self.temp_count;
        self.temp_count += 1;
        Addr::Temp(n)
    }

    /// Aloca o eticheta noua: L0, L1, L2...
    fn new_label(&mut self) -> String {
        let n = self.label_count;
        self.label_count += 1;
        format!("L{}", n)
    }

    /// Emite o instructiune
    fn emit(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    /// Traduce un operator AST in operator TAC
    fn translate_op(op: &ABinOp) -> TBinOp {
        match op {
            ABinOp::Add => TBinOp::Add, ABinOp::Sub => TBinOp::Sub,
            ABinOp::Mul => TBinOp::Mul, ABinOp::Div => TBinOp::Div,
            ABinOp::Eq => TBinOp::Eq, ABinOp::Ne => TBinOp::Ne,
            ABinOp::Lt => TBinOp::Lt, ABinOp::Gt => TBinOp::Gt,
            ABinOp::Le => TBinOp::Le, ABinOp::Ge => TBinOp::Ge,
            ABinOp::And => TBinOp::And, ABinOp::Or => TBinOp::Or,
        }
    }

    /// Genereaza TAC pentru o expresie si returneaza adresa rezultatului.
    /// Acoperim toate variantele din Expr Lab 5.
    pub fn gen_expr(&mut self, expr: &Expr) -> Addr {
        match expr {
            // ── Literale: rezultatul e chiar literalul ──────────────────
            Expr::IntLit(n)   => Addr::IntLit(*n),
            Expr::FloatLit(_) => Addr::IntLit(0), // simplificat: float -> 0
            Expr::BoolLit(b)  => Addr::BoolLit(*b),
            Expr::StrLit(_)   => Addr::IntLit(0), // simplificat: str -> 0

            // ── Variabila: adresa ei directa ─────────────────────────────
            Expr::Var(name)   => Addr::Var(name.clone()),

            // ── Operatie binara: generam operanzii, emitem instructiunea ──
            Expr::BinOp { op, left, right } => {
                let l = self.gen_expr(left);
                let r = self.gen_expr(right);
                let dst = self.new_temp();
                self.emit(Instr::BinOp {
                    dst: dst.clone(),
                    op: Self::translate_op(op),
                    left: l, right: r,
                });
                dst
            }

            // ── Apel de functie: generam argumentele, emitem Call ─────────
            Expr::Call { func, args } => {
                let arg_addrs: Vec<Addr> = args.iter()
                    .map(|a| self.gen_expr(a))
                    .collect();
                let dst = self.new_temp();
                for (i, addr) in arg_addrs.iter().enumerate() {
                    self.emit(Instr::Copy {
                        dst: Addr::Var(format!("arg{}", i)),
                        src: addr.clone(),
                    });
                }
                self.emit(Instr::Copy {
                    dst: dst.clone(),
                    src: Addr::Var(format!("call_{}", func)),
                });
                dst
            }

            // ── If ca expresie (din Lab 5): generam ramurile ─────────────
            Expr::If { cond, then_, else_ } => {
                let l_then = self.new_label();
                let l_else = self.new_label();
                let l_end = self.new_label();
                let result = self.new_temp();
                let t_cond = self.gen_expr(cond);
                self.emit(Instr::IfGoto { cond: t_cond, label: l_then.clone() });
                self.emit(Instr::Goto(l_else.clone()));
                self.emit(Instr::Label(l_then));
                let then_val = self.gen_expr(then_);
                self.emit(Instr::Copy { dst: result.clone(), src: then_val });
                self.emit(Instr::Goto(l_end.clone()));
                self.emit(Instr::Label(l_else));
                if let Some(else_expr) = else_ {
                    let else_val = self.gen_expr(else_expr);
                    self.emit(Instr::Copy { dst: result.clone(), src: else_val });
                }
                self.emit(Instr::Label(l_end));
                result
            }
        }
    }

    /// Genereaza TAC pentru o instructiune.
    /// Variantele Stmt sunt exact cele din Lab 5; adaugam doar Stmt::While.
    pub fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {

            // ── Preluat din Lab 5, nemodificat ──────────────────────────
            Stmt::Let { name, ty: _, init } => {
                // ty ignorat la generare cod (tipul e verificat de type checker)
                let src = match init {
                    Some(expr) => self.gen_expr(expr),
                    None => Addr::IntLit(0),
                };
                self.emit(Instr::Copy { dst: Addr::Var(name.clone()), src });
            }

            Stmt::Assign { name, value } => {
                let src = self.gen_expr(value);
                self.emit(Instr::Copy { dst: Addr::Var(name.clone()), src });
            }

            Stmt::Return(expr) => {
                let src = self.gen_expr(expr);
                self.emit(Instr::Return(src));
            }

            Stmt::ExprStmt(expr) => {
                // Generam codul expresiei; daca e Expr::If, gen_expr
                // produce automat etichetele si sariturile corecte
                self.gen_expr(expr);
            }

            Stmt::Block(stmts) => {
                for s in stmts { self.gen_stmt(s); }
            }

            // ── NOU in Lab 6: Stmt::While ───────────────────────────────
            Stmt::While { cond, body } => {
                let l_start = self.new_label();
                let l_body = self.new_label();
                let l_end   = self.new_label();
                // Eticheta de start (se revine aici la fiecare iteratie)
                self.emit(Instr::Label(l_start.clone()));
                // Evalueaza conditia
                let t_cond = self.gen_expr(cond);
                self.emit(Instr::IfGoto { cond: t_cond, label: l_body.clone() });
                self.emit(Instr::Goto(l_end.clone()));
                // Corpul buclei
                self.emit(Instr::Label(l_body));
                for s in body { self.gen_stmt(s); }
                self.emit(Instr::Goto(l_start));    // inapoi la conditie
                self.emit(Instr::Label(l_end));
            }
            Stmt::FnDecl { name, params: _, body } => {
                self.emit(Instr::Label(name.clone()));
                for s in body { self.gen_stmt(s); }
                self.emit(Instr::Halt);
            }
        }
    }

    pub fn gen_program(&mut self, program: &Program) {
        for f in &program.functions {
            self.emit(Instr::Label(f.name.clone()));
            for s in &f.body {
                self.gen_stmt(s);
            }
            self.emit(Instr::Halt);
        }
    }

    /// Afiseaza toate instructiunile generate
    pub fn print(&self) {
        for instr in &self.instrs {
            println!("{}", instr.display());
        }
    }

    pub fn optimize_redundant_temps(instrs: &[Instr]) -> Vec<Instr> {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < instrs.len() {
            if i + 1 < instrs.len() {
                if let Instr::Copy { dst: Addr::Temp(t0), src } = &instrs[i] {
                    if let Instr::Copy { dst, src: Addr::Temp(t1) } = &instrs[i + 1] {
                        if t0 == t1 {
                            out.push(Instr::Copy { dst: dst.clone(), src: src.clone() });
                            i += 2;
                            continue;
                        }
                    }
                }
            }
            out.push(instrs[i].clone());
            i += 1;
        }
        out
    }

    pub fn optimize_constant_folding(instrs: &[Instr]) -> Vec<Instr> {
        fn key_of(addr: &Addr) -> Option<String> {
            match addr {
                Addr::Var(s) => Some(s.clone()),
                Addr::Temp(n) => Some(format!("t{}", n)),
                _ => None,
            }
        }
        fn b2i(v: bool) -> i64 { if v { 1 } else { 0 } }
        let mut known: HashMap<String, i64> = HashMap::new();
        let mut out = Vec::with_capacity(instrs.len());
        for instr in instrs {
            match instr {
                Instr::BinOp { dst, op, left, right } => {
                    let l = if let Some(k) = key_of(left) {
                        known.get(&k).copied().map(Addr::IntLit).unwrap_or_else(|| left.clone())
                    } else {
                        left.clone()
                    };
                    let r = if let Some(k) = key_of(right) {
                        known.get(&k).copied().map(Addr::IntLit).unwrap_or_else(|| right.clone())
                    } else {
                        right.clone()
                    };
                    if let (Addr::IntLit(li), Addr::IntLit(ri)) = (&l, &r) {
                        let folded = match op {
                            TBinOp::Add => Some(Addr::IntLit(li + ri)),
                            TBinOp::Sub => Some(Addr::IntLit(li - ri)),
                            TBinOp::Mul => Some(Addr::IntLit(li * ri)),
                            TBinOp::Div => Some(Addr::IntLit(li / ri)),
                            TBinOp::Eq => Some(Addr::IntLit(b2i(li == ri))),
                            TBinOp::Ne => Some(Addr::IntLit(b2i(li != ri))),
                            TBinOp::Lt => Some(Addr::IntLit(b2i(li < ri))),
                            TBinOp::Gt => Some(Addr::IntLit(b2i(li > ri))),
                            TBinOp::Le => Some(Addr::IntLit(b2i(li <= ri))),
                            TBinOp::Ge => Some(Addr::IntLit(b2i(li >= ri))),
                            TBinOp::And => Some(Addr::IntLit(b2i(*li != 0 && *ri != 0))),
                            TBinOp::Or => Some(Addr::IntLit(b2i(*li != 0 || *ri != 0))),
                        };
                        if let Some(src) = folded {
                            if let (Some(k), Addr::IntLit(v)) = (key_of(dst), &src) {
                                known.insert(k, *v);
                            }
                            out.push(Instr::Copy { dst: dst.clone(), src });
                            continue;
                        }
                    }
                    if let Some(k) = key_of(dst) {
                        known.remove(&k);
                    }
                    out.push(Instr::BinOp {
                        dst: dst.clone(),
                        op: op.clone(),
                        left: l,
                        right: r,
                    });
                }
                Instr::Copy { dst, src } => {
                    let rewritten_src = if let Some(k) = key_of(src) {
                        known.get(&k).copied().map(Addr::IntLit).unwrap_or_else(|| src.clone())
                    } else {
                        src.clone()
                    };
                    if let Some(k) = key_of(dst) {
                        if let Addr::IntLit(v) = rewritten_src {
                            known.insert(k, v);
                        } else {
                            known.remove(&k);
                        }
                    }
                    out.push(Instr::Copy { dst: dst.clone(), src: rewritten_src });
                }
                Instr::Label(_) | Instr::Goto(_) | Instr::IfGoto { .. } | Instr::Return(_) | Instr::Halt => {
                    known.clear();
                    out.push(instr.clone());
                }
            }
        }
        out
    }

    pub fn optimize_dce(instrs: &[Instr]) -> Vec<Instr> {
        let mut out = Vec::with_capacity(instrs.len());
        let mut skip_until_label = false;
        for instr in instrs {
            if skip_until_label {
                if matches!(instr, Instr::Label(_)) {
                    skip_until_label = false;
                    out.push(instr.clone());
                }
                continue;
            }
            out.push(instr.clone());
            if matches!(instr, Instr::Return(_) | Instr::Goto(_)) {
                skip_until_label = true;
            }
        }
        out
    }

    pub fn optimize_dead_temp_defs(instrs: &[Instr]) -> Vec<Instr> {
        let mut needed: HashMap<usize, ()> = HashMap::new();
        let mut keep_rev: Vec<Instr> = Vec::with_capacity(instrs.len());

        for instr in instrs.iter().rev() {
            match instr {
                Instr::BinOp { dst, left, right, .. } => {
                    if let Addr::Temp(n) = left { needed.insert(*n, ()); }
                    if let Addr::Temp(n) = right { needed.insert(*n, ()); }
                    match dst {
                        Addr::Temp(n) => {
                            if needed.remove(n).is_some() {
                                keep_rev.push(instr.clone());
                            }
                        }
                        _ => keep_rev.push(instr.clone()),
                    }
                }
                Instr::Copy { dst, src } => {
                    if let Addr::Temp(n) = src { needed.insert(*n, ()); }
                    match dst {
                        Addr::Temp(n) => {
                            if needed.remove(n).is_some() {
                                keep_rev.push(instr.clone());
                            }
                        }
                        _ => keep_rev.push(instr.clone()),
                    }
                }
                Instr::IfGoto { cond, .. } => {
                    if let Addr::Temp(n) = cond { needed.insert(*n, ()); }
                    keep_rev.push(instr.clone());
                }
                Instr::Return(addr) => {
                    if let Addr::Temp(n) = addr { needed.insert(*n, ()); }
                    keep_rev.push(instr.clone());
                }
                _ => keep_rev.push(instr.clone()),
            }
        }
        keep_rev.reverse();
        keep_rev
    }

    pub fn optimize_all(instrs: &[Instr]) -> Vec<Instr> {
        let folded = Self::optimize_constant_folding(instrs);
        let copied = Self::optimize_redundant_temps(&folded);
        let no_dead_temps = Self::optimize_dead_temp_defs(&copied);
        Self::optimize_dce(&no_dead_temps)
    }

    pub fn stats(&self) -> CodegenStats {
        let mut salturi = 0usize;
        let mut aritmetice = 0usize;
        for instr in &self.instrs {
            match instr {
                Instr::Goto(_) | Instr::IfGoto { .. } => salturi += 1,
                Instr::BinOp { op, .. } => {
                    if matches!(op, TBinOp::Add | TBinOp::Sub | TBinOp::Mul | TBinOp::Div) {
                        aritmetice += 1;
                    }
                }
                _ => {}
            }
        }
        CodegenStats {
            temporare: self.temp_count,
            etichete: self.label_count,
            salturi,
            aritmetice,
        }
    }
}
