use std::boxed::Box;
use crate::cpu::cpu::Instr;

pub trait Unit{
    fn execute(&mut self) -> ();
    fn result(&self) -> u32;
    fn instr(&self) -> Instr;
}

pub struct ALU{
    x: u32,
    y: u32,
    r: u32,
    f: Box<dyn FnMut(u32, u32) -> u32>,
    instr: Instr
}

impl Unit for ALU{
    fn execute(&mut self)->(){
        self.r = (self.f)(self.x, self.y);
    }
    fn result(&self) -> u32 {self.r.clone()}
    fn instr(&self) -> Instr {self.instr.clone()}
}

impl ALU{
    pub fn new()-> Self{
        ALU{
            x: 0,
            y: 0,
            r: 0,
            f: Box::new(|_, _| 0),
            instr: Instr::Nop,
        }
    }
    pub fn issue<F>(&mut self,instr: Instr, x: u32, y: u32, f:F) -> ()
    where F: FnMut(u32, u32) -> u32 + 'static {
        self.x = x;
        self.y = y;
        self.f = Box::new(f);
        self.instr = instr;
    }
}
