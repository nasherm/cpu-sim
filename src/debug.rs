use crate::cpu::cpu;

pub struct Debugger{
    cpu: cpu::CPU,
}

impl Debugger{
    pub fn new() -> Debugger {
        Debugger{
            cpu: cpu::CPU::new(),
        }
    }
}