// src/tac.rs
#![allow(dead_code)]             // suprima warning-uri pentru variante neutilizate in teste

/// O adresa TAC: poate fi o variabila, un temporar, sau un literal numeric
#[derive(Debug, Clone, PartialEq)]
pub enum Addr {
    Var(String),      // variabila din program: x, y, rezultat
    Temp(usize),      // temporar generat: t0, t1, t2...
    IntLit(i64),      // literal numeric: 42, -1, 0
    BoolLit(bool),    // literal boolean: true, false
}

impl Addr {
    pub fn display(&self) -> String {
        match self {
            Addr::Var(s)       => s.clone(),
            Addr::Temp(n)      => format!("t{}", n),
            Addr::IntLit(n)    => n.to_string(),
            Addr::BoolLit(b)   => b.to_string(),
        }
    }
}

/// Operatorii binari suportati
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp { Add, Sub, Mul, Div, Eq, Ne, Lt, Gt, Le, Ge, And, Or }

/// O instructiune TAC
#[derive(Debug, Clone)]
pub enum Instr {
    /// t = y op z
    BinOp { dst: Addr, op: BinOp, left: Addr, right: Addr },

    /// t = y (copiere simpla)
    Copy { dst: Addr, src: Addr },

    /// goto L
    Goto(String),

    /// if t goto L (sare daca t e nenul/true)
    IfGoto { cond: Addr, label: String },

    /// L: (definire eticheta)
    Label(String),

    /// return expr
    Return(Addr),

    /// halt
    Halt,
}

impl Instr {
    /// Afiseaza instructiunea TAC intr-un format lizibil
    pub fn display(&self) -> String {
        match self {
            Instr::BinOp{dst,op,left,right} => format!(
                " {} = {} {:?} {}", dst.display(), left.display(), op, right.display()),
            Instr::Copy{dst,src} => format!(" {} = {}", dst.display(), src.display()),
            Instr::Goto(l)       => format!(" goto {}", l),
            Instr::IfGoto{cond,label} => format!(" if {} goto {}", cond.display(), label),
            Instr::Label(l)      => format!("{}:", l),
            Instr::Return(a)     => format!(" return {}", a.display()),
            Instr::Halt          => " halt".to_string(),
        }
    }
}
