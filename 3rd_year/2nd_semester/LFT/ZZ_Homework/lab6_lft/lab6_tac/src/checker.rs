// src/checker.rs
use crate::ast::*;
use crate::symbol_table::{Symbol, SymbolTable};

pub struct TypeChecker {
    pub table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker { table: SymbolTable::new() }
    }

    /// Inregistreaza toate functiile in scope-ul global (permite apeluri forward)
    pub fn register_functions(&mut self, program: &Program) {
        for f in &program.functions {
            let fn_type = Type::Fn {
                params: f.params.iter().map(|(_, t)| t.clone()).collect(),
                ret: Box::new(f.ret_ty.clone()),
            };
            self.table.declare(Symbol { name: f.name.clone(), ty: fn_type, mutable: false });
        }
    }

    /// Verifica intregul program
    pub fn check_program(&mut self, program: &Program) {
        self.register_functions(program);
        for f in &program.functions {
            self.check_function(f);
        }
    }

    /// Verifica o declaratie de functie
    fn check_function(&mut self, f: &FnDecl) {
        self.table.push_scope();
        // Adauga parametrii in scope-ul functiei
        for (name, ty) in &f.params {
            self.table.declare(Symbol { name: name.clone(), ty: ty.clone(), mutable: false });
        }
        for stmt in &f.body {
            self.check_stmt(stmt, &f.ret_ty);
        }
        self.table.pop_scope();
    }

    /// Verifica o instructiune
    fn check_stmt(&mut self, stmt: &Stmt, ret_ty: &Type) {
        match stmt {
            Stmt::Let { name, ty, init } => {
                let inferred = match init {
                    Some(expr) => self.check_expr(expr),
                    None => Type::Int,
                };
                // Daca tipul e specificat explicit, verifica compatibilitatea
                if let Some(declared_ty) = ty {
                    if &inferred != declared_ty {
                        self.table.error(format!(
                            "Tip incompatibil pentru '{}': asteptat {:?}, gasit {:?}",
                            name, declared_ty, inferred
                        ));
                    }
                }
                self.table.declare(Symbol { name: name.clone(), ty: inferred, mutable: true });
            }
            Stmt::Assign { name, value } => {
                match self.table.lookup(name).cloned() {
                    None => self.table.error(format!("Variabila nedeclarata: '{}'", name)),
                    Some(sym) => {
                        if !sym.mutable {
                            self.table.error(format!(
                                "Atribuire invalida: '{}' este imutabila",
                                name
                            ));
                            return;
                        }
                        let val_ty = self.check_expr(value);
                        let sym_ty = sym.ty;
                        if val_ty != sym_ty {
                            self.table.error(format!(
                                "Atribuire invalida pentru '{}': {:?} = {:?}",
                                name, sym_ty, val_ty
                            ));
                        }
                    }
                }
            }
            Stmt::Return(expr) => {
                let expr_ty = self.check_expr(expr);
                if &expr_ty != ret_ty {
                    self.table.error(format!(
                        "Return incompatibil: asteptat {:?}, gasit {:?}",
                        ret_ty, expr_ty
                    ));
                }
            }
            Stmt::ExprStmt(expr) => { self.check_expr(expr); }
            Stmt::Block(stmts) => {
                self.table.push_scope();
                for s in stmts { self.check_stmt(s, ret_ty); }
                self.table.pop_scope();
            }
            Stmt::While { cond, body } => {
                let cond_ty = self.check_expr(cond);
                if cond_ty != Type::Bool {
                    self.table.error(format!("Conditia while trebuie sa fie Bool, gasit {:?}", cond_ty));
                }
                self.table.push_scope();
                for s in body { self.check_stmt(s, ret_ty); }
                self.table.pop_scope();
            }
            Stmt::FnDecl { name: _, params: _, body } => {
                self.table.push_scope();
                for s in body { self.check_stmt(s, &Type::Unit); }
                self.table.pop_scope();
            }
        }
    }

    /// Verifica o expresie si returneaza tipul sau
    pub fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::IntLit(_)   => Type::Int,
            Expr::FloatLit(_) => Type::Float,
            Expr::BoolLit(_)  => Type::Bool,
            Expr::StrLit(_)   => Type::Str,

            Expr::Var(name) => {
                match self.table.lookup(name) {
                    Some(sym) => sym.ty.clone(),
                    None => {
                        self.table.error(format!("Variabila nedeclarata: '{}'", name));
                        Type::Unit       // continua cu tip placeholder
                    }
                }
            }

            Expr::BinOp { op, left, right } => {
                let lt = self.check_expr(left);
                let rt = self.check_expr(right);
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                        if lt != rt || (lt != Type::Int && lt != Type::Float) {
                            self.table.error(format!(
                                "Operatori {:?} necesita Int sau Float, gasit {:?} si {:?}",
                                op, lt, rt
                            ));
                        }
                        lt
                    }
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                        if lt != rt {
                            self.table.error(format!("Comparatie intre tipuri diferite: {:?} vs {:?}", lt, rt));
                        }
                        Type::Bool
                    }
                    BinOp::And | BinOp::Or => {
                        if lt != Type::Bool || rt != Type::Bool {
                            self.table.error(format!("And/Or necesita Bool, gasit {:?} si {:?}", lt, rt));
                        }
                        Type::Bool
                    }
                }
            }

            Expr::Call { func, args } => {
                match self.table.lookup(func).cloned() {
                    None => {
                        self.table.error(format!("Functie nedeclarata: '{}'", func));
                        Type::Unit
                    }
                    Some(sym) => {
                        if let Type::Fn { params, ret } = sym.ty {
                            if args.len() != params.len() {
                                self.table.error(format!(
                                    "'{}' asteapta {} argumente, primit {}",
                                    func, params.len(), args.len()
                                ));
                            } else {
                                for (arg, param_ty) in args.iter().zip(params.iter()) {
                                    let arg_ty = self.check_expr(arg);
                                    if &arg_ty != param_ty {
                                        self.table.error(format!(
                                            "Argument incorect pentru '{}': asteptat {:?}, gasit {:?}",
                                            func, param_ty, arg_ty
                                        ));
                                    }
                                }
                            }
                            *ret
                        } else {
                            self.table.error(format!("'{}' nu este o functie", func));
                            Type::Unit
                        }
                    }
                }
            }

            Expr::If { cond, then_, else_ } => {
                let cond_ty = self.check_expr(cond);
                if cond_ty != Type::Bool {
                    self.table.error(format!("Conditia if trebuie sa fie Bool, gasit {:?}", cond_ty));
                }
                let then_ty = self.check_expr(then_);
                if let Some(else_expr) = else_ {
                    let else_ty = self.check_expr(else_expr);
                    if then_ty != else_ty {
                        self.table.error(format!("Ramurile if/else au tipuri diferite: {:?} vs {:?}", then_ty, else_ty));
                    }
                }
                then_ty
            }
        }
    }
}
