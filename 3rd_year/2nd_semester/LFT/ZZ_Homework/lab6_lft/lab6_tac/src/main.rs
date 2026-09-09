// src/main.rs
mod ast;
mod tac;
mod codegen;
mod interpreter;
mod symbol_table;
mod checker;

use ast::{BinOp, Expr, FnDecl, Program, Stmt, Type};
use checker::TypeChecker;
use codegen::CodeGen;
use interpreter::Interpreter;

fn run_pipeline(stmts: &[Stmt], ret_ty: Type) -> Result<Option<i64>, Vec<String>> {
    let program = Program {
        functions: vec![FnDecl {
            name: "main".into(),
            params: vec![],
            ret_ty,
            body: stmts.to_vec(),
        }],
    };
    let mut checker = TypeChecker::new();
    checker.check_program(&program);
    if checker.table.has_errors() {
        return Err(checker.table.errors);
    }

    let mut cg = CodeGen::new();
    for s in stmts { cg.gen_stmt(s); }
    cg.instrs = CodeGen::optimize_all(&cg.instrs);
    let mut interp = Interpreter::new();
    interp.run(&cg.instrs).map_err(|e| vec![e])
}

fn run_test(name: &str, stmts: Vec<Stmt>) {
    println!("\n=== {} ===", name);
    match run_pipeline(&stmts, Type::Int) {
        Err(errors) => {
            println!("Erori semantice:");
            for e in &errors {
                println!("  EROARE: {}", e);
            }
        }
        Ok(result) => {
            let mut cg = CodeGen::new();
            for s in &stmts { cg.gen_stmt(s); }
            cg.instrs = CodeGen::optimize_all(&cg.instrs);
            println!("TAC generat:");
            cg.print();
            let stats = cg.stats();
            println!(
                "Stats: temporare={}, etichete={}, salturi={}, aritmetice={}",
                stats.temporare, stats.etichete, stats.salturi, stats.aritmetice
            );
            println!("Rezultat: {:?}", result);
        }
    }
}

fn main() {
    // Test 1: let x = 2 + 3 * 4; return x
    // Folosim Stmt::Let cu ty: None (exact ca in Lab 5)
    run_test("Expresie: x = 2 + 3 * 4", vec![
        Stmt::Let { name: "x".into(), ty: None, init: Some(
            Expr::BinOp { op: BinOp::Add,
                left: Box::new(Expr::IntLit(2)),
                right: Box::new(Expr::BinOp { op: BinOp::Mul,
                    left: Box::new(Expr::IntLit(3)),
                    right: Box::new(Expr::IntLit(4)),
                }),
            }
        )},
        Stmt::Return(Expr::Var("x".into())),
    ]);

    // Test 2: max(a, b) cu Stmt::While (NOU in Lab 6)
    // a = 5, b = 3; while a > b { a = a - 1 }; return a
    run_test("While: coboara a pana la b", vec![
        Stmt::Let { name: "a".into(), ty: None, init: Some(Expr::IntLit(5)) },
        Stmt::Let { name: "b".into(), ty: None, init: Some(Expr::IntLit(3)) },
        // Stmt::While este NOU in Lab 6 — nu exista in Lab 5
        Stmt::While {
            cond: Expr::BinOp { op: BinOp::Gt,
                left: Box::new(Expr::Var("a".into())),
                right: Box::new(Expr::Var("b".into())),
            },
            body: vec![
                // Stmt::Assign cu camp 'value' — exact ca in Lab 5
                Stmt::Assign { name: "a".into(),
                    value: Expr::BinOp { op: BinOp::Sub,
                        left: Box::new(Expr::Var("a".into())),
                        right: Box::new(Expr::IntLit(1)),
                    }
                },
            ],
        },
        Stmt::Return(Expr::Var("a".into())),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_expr_from_lab5_shape_runs() {
        let stmts = vec![
            Stmt::Let { name: "x".into(), ty: None, init: Some(
                Expr::BinOp { op: BinOp::Add,
                    left: Box::new(Expr::IntLit(2)),
                    right: Box::new(Expr::BinOp { op: BinOp::Mul,
                        left: Box::new(Expr::IntLit(3)),
                        right: Box::new(Expr::IntLit(4)),
                    }),
                }
            )},
            Stmt::Return(Expr::Var("x".into())),
        ];
        let result = run_pipeline(&stmts, Type::Int).expect("semantic check should pass");
        assert_eq!(result, Some(14));
    }

    #[test]
    fn pipeline_while_runs_and_returns_expected_value() {
        let stmts = vec![
            Stmt::Let { name: "a".into(), ty: None, init: Some(Expr::IntLit(5)) },
            Stmt::Let { name: "b".into(), ty: None, init: Some(Expr::IntLit(3)) },
            Stmt::While {
                cond: Expr::BinOp { op: BinOp::Gt,
                    left: Box::new(Expr::Var("a".into())),
                    right: Box::new(Expr::Var("b".into())),
                },
                body: vec![
                    Stmt::Assign { name: "a".into(),
                        value: Expr::BinOp { op: BinOp::Sub,
                            left: Box::new(Expr::Var("a".into())),
                            right: Box::new(Expr::IntLit(1)),
                        }
                    },
                ],
            },
            Stmt::Return(Expr::Var("a".into())),
        ];
        let result = run_pipeline(&stmts, Type::Int).expect("semantic check should pass");
        assert_eq!(result, Some(3));
    }

    #[test]
    fn semantic_error_stops_pipeline() {
        let stmts = vec![
            Stmt::Let { name: "x".into(), ty: None, init: Some(Expr::BoolLit(true)) },
            Stmt::Return(Expr::Var("x".into())),
        ];
        let err = run_pipeline(&stmts, Type::Int).expect_err("should fail semantic check");
        assert!(err.iter().any(|e| e.contains("Return incompatibil")));
    }

    #[test]
    fn let_without_init_defaults_to_zero() {
        let stmts = vec![
            Stmt::Let { name: "x".into(), ty: None, init: None },
            Stmt::Return(Expr::Var("x".into())),
        ];
        let result = run_pipeline(&stmts, Type::Int).expect("semantic check should pass");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn redundant_temp_optimization_collapses_copy_chain() {
        use crate::tac::{Addr, Instr};
        let input = vec![
            Instr::Copy { dst: Addr::Temp(0), src: Addr::Var("x".into()) },
            Instr::Copy { dst: Addr::Var("y".into()), src: Addr::Temp(0) },
        ];
        let out = CodeGen::optimize_redundant_temps(&input);
        assert_eq!(out.len(), 1);
        match &out[0] {
            Instr::Copy { dst: Addr::Var(y), src: Addr::Var(x) } => {
                assert_eq!(y, "y");
                assert_eq!(x, "x");
            }
            _ => panic!("unexpected optimized instruction"),
        }
    }

    #[test]
    fn constant_folding_turns_binop_into_copy() {
        use crate::tac::{Addr, BinOp as TBinOp, Instr};
        let input = vec![
            Instr::BinOp {
                dst: Addr::Temp(0),
                op: TBinOp::Mul,
                left: Addr::IntLit(3),
                right: Addr::IntLit(4),
            },
        ];
        let out = CodeGen::optimize_constant_folding(&input);
        assert_eq!(out.len(), 1);
        match &out[0] {
            Instr::Copy { dst: Addr::Temp(0), src: Addr::IntLit(12) } => {}
            _ => panic!("constant folding did not produce expected copy"),
        }
    }

    #[test]
    fn dce_removes_instructions_after_return_until_label() {
        use crate::tac::{Addr, Instr};
        let input = vec![
            Instr::Return(Addr::IntLit(1)),
            Instr::Copy { dst: Addr::Var("x".into()), src: Addr::IntLit(99) },
            Instr::Label("L1".into()),
            Instr::Copy { dst: Addr::Var("y".into()), src: Addr::IntLit(7) },
        ];
        let out = CodeGen::optimize_dce(&input);
        assert_eq!(out.len(), 3);
        assert!(matches!(out[0], Instr::Return(_)));
        assert!(matches!(out[1], Instr::Label(_)));
        assert!(matches!(out[2], Instr::Copy { .. }));
    }

    #[test]
    fn combined_optimizations_reduce_expr_to_x_eq_14() {
        let stmts = vec![
            Stmt::Let { name: "x".into(), ty: None, init: Some(
                Expr::BinOp {
                    op: BinOp::Add,
                    left: Box::new(Expr::IntLit(2)),
                    right: Box::new(Expr::BinOp {
                        op: BinOp::Mul,
                        left: Box::new(Expr::IntLit(3)),
                        right: Box::new(Expr::IntLit(4)),
                    }),
                }
            )},
        ];
        let mut cg = CodeGen::new();
        for s in &stmts { cg.gen_stmt(s); }
        let out = CodeGen::optimize_all(&cg.instrs);
        assert_eq!(out.len(), 1);
        match &out[0] {
            crate::tac::Instr::Copy { dst: crate::tac::Addr::Var(name), src: crate::tac::Addr::IntLit(14) } => {
                assert_eq!(name, "x");
            }
            _ => panic!("expected x = 14 after combined optimizations"),
        }
    }

    #[test]
    fn interpreter_set_var_works_for_max_like_program() {
        use crate::tac::{Addr, BinOp as TBinOp, Instr};
        let instrs = vec![
            Instr::BinOp {
                dst: Addr::Temp(0),
                op: TBinOp::Gt,
                left: Addr::Var("a".into()),
                right: Addr::Var("b".into()),
            },
            Instr::IfGoto { cond: Addr::Temp(0), label: "L_then".into() },
            Instr::Goto("L_else".into()),
            Instr::Label("L_then".into()),
            Instr::Return(Addr::Var("a".into())),
            Instr::Label("L_else".into()),
            Instr::Return(Addr::Var("b".into())),
        ];
        let mut interp = Interpreter::new();
        interp.set_var("a", 7);
        interp.set_var("b", 3);
        let result = interp.run(&instrs).expect("runtime should succeed");
        assert_eq!(result, Some(7));
    }

    #[test]
    fn interpreter_trace_runs_for_while_to_five() {
        let stmts = vec![
            Stmt::Let { name: "i".into(), ty: None, init: Some(Expr::IntLit(0)) },
            Stmt::While {
                cond: Expr::BinOp {
                    op: BinOp::Lt,
                    left: Box::new(Expr::Var("i".into())),
                    right: Box::new(Expr::IntLit(5)),
                },
                body: vec![
                    Stmt::Assign {
                        name: "i".into(),
                        value: Expr::BinOp {
                            op: BinOp::Add,
                            left: Box::new(Expr::Var("i".into())),
                            right: Box::new(Expr::IntLit(1)),
                        },
                    },
                ],
            },
            Stmt::Return(Expr::Var("i".into())),
        ];
        let mut cg = CodeGen::new();
        for s in &stmts { cg.gen_stmt(s); }
        let mut interp = Interpreter::new();
        let result = interp.run_trace(&cg.instrs).expect("runtime should succeed");
        assert_eq!(result, Some(5));
    }

    #[test]
    fn interpreter_detects_infinite_loop() {
        use crate::tac::Instr;
        let instrs = vec![
            Instr::Label("L0".into()),
            Instr::Goto("L0".into()),
        ];
        let mut interp = Interpreter::new();
        let err = interp
            .run_with_options(&instrs, false, 10000)
            .expect_err("should detect infinite loop");
        println!("{}", err);
        assert!(err.contains("loop infinit"));
    }

    #[test]
    fn pipeline_is_semantic_then_codegen() {
        let stmts = vec![
            Stmt::Assign { name: "x".into(), value: Expr::IntLit(1) },
            Stmt::Return(Expr::IntLit(0)),
        ];
        let err = run_pipeline(&stmts, Type::Int).expect_err("semantic check should fail first");
        assert!(err.iter().any(|e| e.contains("Variabila nedeclarata")));
    }

    #[test]
    fn codegen_supports_stmt_fndecl_with_label_and_halt() {
        let program = Program { functions: vec![FnDecl {
            name: "foo".into(),
            params: vec![],
            ret_ty: Type::Int,
            body: vec![Stmt::FnDecl {
                name: "bar".into(),
                params: vec!["a".into()],
                body: vec![Stmt::Return(Expr::IntLit(1))],
            }],
        }]};
        let mut cg = CodeGen::new();
        cg.gen_program(&program);
        assert!(matches!(cg.instrs[0], crate::tac::Instr::Label(_)));
        assert!(cg.instrs.iter().any(|i| matches!(i, crate::tac::Instr::Label(name) if name == "bar")));
        assert!(cg.instrs.iter().any(|i| matches!(i, crate::tac::Instr::Halt)));
    }

    #[test]
    fn stmt_fndecl_emits_label_body_halt() {
        let stmt = Stmt::FnDecl {
            name: "inner".into(),
            params: vec!["a".into()],
            body: vec![Stmt::Return(Expr::IntLit(1))],
        };
        let mut cg = CodeGen::new();
        cg.gen_stmt(&stmt);
        assert!(matches!(cg.instrs[0], crate::tac::Instr::Label(_)));
        assert!(matches!(cg.instrs[1], crate::tac::Instr::Return(_)));
        assert!(matches!(cg.instrs[2], crate::tac::Instr::Halt));
    }

    #[test]
    fn interpreter_environment_is_program_level_for_two_functions() {
        use crate::tac::{Addr, BinOp as TBinOp, Instr};
        let instrs = vec![
            Instr::Label("f_set".into()),
            Instr::Copy { dst: Addr::Var("g".into()), src: Addr::IntLit(41) },
            Instr::Halt,
            Instr::Label("f_inc".into()),
            Instr::BinOp {
                dst: Addr::Temp(0),
                op: TBinOp::Add,
                left: Addr::Var("g".into()),
                right: Addr::IntLit(1),
            },
            Instr::Copy { dst: Addr::Var("g".into()), src: Addr::Temp(0) },
            Instr::Return(Addr::Var("g".into())),
            Instr::Halt,
        ];
        let mut interp = Interpreter::new();
        let r1 = interp.run_function(&instrs, "f_set").expect("f_set should execute");
        assert_eq!(r1, None);
        let r2 = interp.run_function(&instrs, "f_inc").expect("f_inc should execute");
        assert_eq!(r2, Some(42));
    }
}
