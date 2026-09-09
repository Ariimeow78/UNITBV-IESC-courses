// src/ast.rs
#![allow(dead_code)]  // suprima warning-uri pentru variante neutilizate in teste

// ── TIP: nemodificat din Lab 5 ─────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, Float, Bool, Str, Unit,
    Fn { params: Vec<Type>, ret: Box<Type> },
}

// ── EXPRESII: nemodificate din Lab 5 ────────────────────────────────
#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StrLit(String),
    Var(String),
    BinOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    Call { func: String, args: Vec<Expr> },
    If    { cond: Box<Expr>, then_: Box<Expr>, else_: Option<Box<Expr>> },
}

// ── OPERATORI BINARI: adaugam Le si Ge fata de Lab 5 ───────────────
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    // Preluati din Lab 5 (nemodificati):
    Add, Sub, Mul, Div,
    Eq, Ne, Lt, Gt,
    And, Or,
    // NOU in Lab 6 (necesari pentru conditii de bucla):
    Le, Ge,
}

// ── INSTRUCTIUNI: adaugam While fata de Lab 5 ───────────────────────
#[derive(Debug, Clone)]
pub enum Stmt {
    // Preluate din Lab 5 (nemodificate):
    Let    { name: String, ty: Option<Type>, init: Option<Expr> },
    Assign { name: String, value: Expr },
    Return(Expr),
    ExprStmt(Expr),
    Block(Vec<Stmt>),

    // NOU in Lab 6:
    While { cond: Expr, body: Vec<Stmt> },
    FnDecl { name: String, params: Vec<String>, body: Vec<Stmt> },
}

// ── FUNCTII SI PROGRAM: nemodificate din Lab 5 ─────────────────────
#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name:   String,
    pub params: Vec<(String, Type)>,
    pub ret_ty: Type,
    pub body:   Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FnDecl>,
}
