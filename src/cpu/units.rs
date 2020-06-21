use crate::cpu::cpu::Instr;

pub trait Unit{
    fn execute(&mut self) -> ();
    fn result(&self) -> u32;
    fn instr(&self) -> Instr;
    fn avail(&self) -> bool{
        // TODO: what happens on stalls?
        true
    }
}

type OpType = fn(u32, u32) -> u32;
#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
}
pub struct ALU{
    x: u32,
    y: u32,
    r: u32,
    f: OpType,
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
            f: |_, _|{0},
            instr: Instr::Nop,
        }
    }

    pub fn issue(&mut self,instr: Instr, x: u32, y: u32, op: Op) -> () {
        self.x = x;
        self.y = y;
        self.f = match op {
            Op::Add => |x: u32, y:u32| -> u32 {x + y},
            Op::Sub => |x: u32, y:u32| -> u32 {x - y},
        };
        self.instr = instr;
    }
}
