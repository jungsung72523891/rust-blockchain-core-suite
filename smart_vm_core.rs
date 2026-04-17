use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum VMInstruction {
    Load(u64),
    Store,
    Add,
    Sub,
    Mul,
    Div,
    Push(String),
    Transfer(u64),
    GetBalance,
    Halt,
}

pub struct SmartVM {
    stack: Vec<u64>,
    memory: HashMap<String, u64>,
    gas_used: u64,
    max_gas: u64,
}

impl SmartVM {
    pub fn new(max_gas: u64) -> Self {
        Self {
            stack: Vec::new(),
            memory: HashMap::new(),
            gas_used: 0,
            max_gas,
        }
    }

    fn consume_gas(&mut self, gas: u64) -> bool {
        if self.gas_used + gas > self.max_gas {
            return false;
        }
        self.gas_used += gas;
        true
    }

    pub fn execute(&mut self, instructions: &[VMInstruction]) -> Result<u64, String> {
        for instr in instructions {
            if !self.consume_gas(1) {
                return Err("Out of gas".to_string());
            }

            match instr {
                VMInstruction::Load(val) => self.stack.push(*val),
                VMInstruction::Store => {
                    let val = self.stack.pop().ok_or("Stack underflow")?;
                    let key = format!("mem_{}", self.memory.len());
                    self.memory.insert(key, val);
                }
                VMInstruction::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a + b);
                }
                VMInstruction::Sub => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a.saturating_sub(b));
                }
                VMInstruction::Mul => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a * b);
                }
                VMInstruction::Div => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a / b);
                }
                VMInstruction::Halt => break,
                _ => {}
            }
        }
        Ok(self.stack.last().copied().unwrap_or(0))
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }
}
