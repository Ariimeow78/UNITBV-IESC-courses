// src/interpreter.rs
use std::collections::HashMap;
use crate::tac::{Addr, BinOp, Instr};

pub struct Interpreter {
    memory: HashMap<String, i64>,                 // variabile si temporare
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { memory: HashMap::new() }
    }

    pub fn set_var(&mut self, name: &str, val: i64) {
        self.memory.insert(name.to_string(), val);
    }

    fn load(&self, addr: &Addr) -> i64 {
        match addr {
            Addr::IntLit(n) => *n,
            Addr::BoolLit(b) => if *b { 1 } else { 0 },
            Addr::Var(s)     => *self.memory.get(s).unwrap_or(&0),
            Addr::Temp(n)    => *self.memory.get(&format!("t{}",n)).unwrap_or(&0),
        }
    }

    fn store(&mut self, addr: &Addr, val: i64) {
        let key = match addr {
            Addr::Var(s) => s.clone(),
            Addr::Temp(n) => format!("t{}", n),
            _ => panic!("Nu se poate scrie in literal"),
        };
        self.memory.insert(key, val);
    }

    fn dump_memory(&self) -> String {
        let mut entries: Vec<_> = self.memory.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        let parts: Vec<String> = entries
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{{{}}}", parts.join(", "))
    }

    fn labels_index(instrs: &[Instr]) -> HashMap<String, usize> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (i, instr) in instrs.iter().enumerate() {
            if let Instr::Label(l) = instr {
                labels.insert(l.clone(), i);
            }
        }
        labels
    }

    /// Ruleaza programul TAC cu tracing optional si limita de pasi.
    pub fn run_with_options(
        &mut self,
        instrs: &[Instr],
        trace: bool,
        max_steps: usize,
    ) -> Result<Option<i64>, String> {
        // Construieste un index de etichete: label -> index in instrs
        let labels = Self::labels_index(instrs);

        let mut pc: usize = 0;
        let mut steps: usize = 0;
        while pc < instrs.len() {
            steps += 1;
            if steps > max_steps {
                return Err(format!(
                    "Executie oprita: posibil loop infinit (peste {} pasi)",
                    max_steps
                ));
            }
            match &instrs[pc] {
                Instr::BinOp { dst, op, left, right } => {
                    let l = self.load(left);
                    let r = self.load(right);
                    let result = match op {
                        BinOp::Add => l + r,
                        BinOp::Sub => l - r,
                        BinOp::Mul => l * r,
                        BinOp::Div => l / r,
                        BinOp::Eq => if l == r { 1 } else { 0 },
                        BinOp::Ne => if l != r { 1 } else { 0 },
                        BinOp::Lt => if l < r { 1 } else { 0 },
                        BinOp::Gt => if l > r { 1 } else { 0 },
                        BinOp::Le => if l <= r { 1 } else { 0 },
                        BinOp::Ge => if l >= r { 1 } else { 0 },
                        BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
                        BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
                    };
                    self.store(dst, result);
                    pc += 1;
                }
                Instr::Copy { dst, src } => {
                    let val = self.load(src);
                    self.store(dst, val);
                    pc += 1;
                }
                Instr::Goto(label) => {
                    pc = *labels.get(label).expect("Eticheta negasita");
                    pc += 1; // sarim peste Label() insusi
                }
                Instr::IfGoto { cond, label } => {
                    let val = self.load(cond);
                    if val != 0 {
                        pc = *labels.get(label).expect("Eticheta negasita");
                        pc += 1;
                    } else {
                        pc += 1;
                    }
                }
                Instr::Label(_) => { pc += 1; }
                Instr::Return(addr) => { return Ok(Some(self.load(addr))); }
                Instr::Halt => { return Ok(None); }
            }
            if trace {
                println!("step={} pc={} mem={}", steps, pc, self.dump_memory());
            }
        }
        Ok(None)
    }

    /// Ruleaza programul TAC si returneaza valoarea returnata (sau None)
    pub fn run(&mut self, instrs: &[Instr]) -> Result<Option<i64>, String> {
        self.run_with_options(instrs, false, 10000)
    }

    pub fn run_trace(&mut self, instrs: &[Instr]) -> Result<Option<i64>, String> {
        self.run_with_options(instrs, true, 10000)
    }

    pub fn run_function(&mut self, instrs: &[Instr], func_label: &str) -> Result<Option<i64>, String> {
        let labels = Self::labels_index(instrs);
        let start = labels
            .get(func_label)
            .copied()
            .ok_or_else(|| format!("Functie necunoscuta: {}", func_label))?;
        self.run_from_pc(instrs, start + 1, false, 10000)
    }

    fn run_from_pc(
        &mut self,
        instrs: &[Instr],
        start_pc: usize,
        trace: bool,
        max_steps: usize,
    ) -> Result<Option<i64>, String> {
        let labels = Self::labels_index(instrs);
        let mut pc: usize = start_pc;
        let mut steps: usize = 0;
        while pc < instrs.len() {
            steps += 1;
            if steps > max_steps {
                return Err(format!(
                    "Executie oprita: posibil loop infinit (peste {} pasi)",
                    max_steps
                ));
            }
            match &instrs[pc] {
                Instr::BinOp { dst, op, left, right } => {
                    let l = self.load(left);
                    let r = self.load(right);
                    let result = match op {
                        BinOp::Add => l + r,
                        BinOp::Sub => l - r,
                        BinOp::Mul => l * r,
                        BinOp::Div => l / r,
                        BinOp::Eq => if l == r { 1 } else { 0 },
                        BinOp::Ne => if l != r { 1 } else { 0 },
                        BinOp::Lt => if l < r { 1 } else { 0 },
                        BinOp::Gt => if l > r { 1 } else { 0 },
                        BinOp::Le => if l <= r { 1 } else { 0 },
                        BinOp::Ge => if l >= r { 1 } else { 0 },
                        BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
                        BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
                    };
                    self.store(dst, result);
                    pc += 1;
                }
                Instr::Copy { dst, src } => {
                    let val = self.load(src);
                    self.store(dst, val);
                    pc += 1;
                }
                Instr::Goto(label) => {
                    pc = *labels.get(label).expect("Eticheta negasita");
                    pc += 1;
                }
                Instr::IfGoto { cond, label } => {
                    let val = self.load(cond);
                    if val != 0 {
                        pc = *labels.get(label).expect("Eticheta negasita");
                        pc += 1;
                    } else {
                        pc += 1;
                    }
                }
                Instr::Label(_) => { pc += 1; }
                Instr::Return(addr) => { return Ok(Some(self.load(addr))); }
                Instr::Halt => { return Ok(None); }
            }
            if trace {
                println!("step={} pc={} mem={}", steps, pc, self.dump_memory());
            }
        }
        Ok(None)
    }
}
