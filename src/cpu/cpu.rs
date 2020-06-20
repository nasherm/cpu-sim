use std::vec::Vec;
use std::string::String;
use super::units::*;
use std::collections::VecDeque;
/*
The main logic for the CPU. This includes key pipelining decoding of instructions,
as well as execution unit interaction and the definition of the
ISA
    */
#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Movi(u32, u32), //  movi dest val (reg[dest]<-val) [dest] is a register
    Mov(u32, u32), // mov dest src (reg[dest] <- reg[src])
    Addi(u32, u32), // addi dest val (reg[dest] <- reg[dest] + val)
    Subi(u32, u32), // subi dest val (reg[dest] <-reg[dest] - val)
    Addr(u32, u32, u32), // addr dest src1 src2 (reg[dest] <- reg[src1] + reg[src2])
    Add(u32, u32, u32), // add dest val1 val2 (reg[dest] <- val1 + val2)
    Subr(u32, u32, u32), // subr dest src1 src2 (reg[dest] <- reg[src1] - reg[src2])
    Sub(u32, u32, u32), // sub dest val1 val2 (reg[dest]<- val1 - val2)
    Nop,
}


#[derive(Debug, PartialEq)]
pub enum Stage {
    Fetch,
    Decode,
    Execute,
    WriteBack
}

pub struct CPU {
    pub instruction_mem: VecDeque<Instr>,
    pub registers: [u32; 256],
    pub current_instruction: Instr,
    pub prev_instruction: Instr,
    pub next_instruction: Instr,
    pub next_stage: Stage,
    ticks: u32,
    task_units: VecDeque<Box<dyn Unit>>,
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            instruction_mem: VecDeque::new(),
            registers: [0; 256],
            current_instruction: Instr::Nop,
            prev_instruction: Instr::Nop,
            next_instruction: Instr::Nop,
            next_stage: Stage::Fetch,
            ticks: 0,
            task_units: VecDeque::new(),
        }
    }

    pub fn load_instr_vec(&mut self, instrs:&VecDeque<Instr>) {
        self.instruction_mem = instrs.clone();
    }

    pub fn registers(& self) -> &[u32; 256]{
        &self.registers
    }

    // fetch decode execute writeback
    pub fn clock_tick(&mut self)->(){
        self.next_stage =  match self.next_stage {
            Stage::Fetch => self.fetch(),
            Stage::Decode => self.decode(),
            Stage::Execute => self.execute(),
            Stage::WriteBack => self.writeback(),
        };
        self.ticks += 1;
    }

    pub fn run_to_end(&mut self) -> () {
        while self.instruction_mem.len() > 0 || self.current_instruction != Instr::Nop{
            self.clock_tick()
        }
    }

    pub fn load_instructions(&mut self, instructions: Vec<Result<Instr, String>>) -> () {
        for res in instructions {
            match res {
                Ok(i) => self.instruction_mem.push_back(i),
                Err(e) => panic!("Instruction not loaded | {:?}", e),
            }
        }
    }

    fn set_next_instruction (&mut self) {
        let instr = self.instruction_mem.pop_front();
        self.prev_instruction = self.current_instruction.clone();
        self.current_instruction =  match instr {
            Some(i) => i,
            _ => Instr::Nop
        };
        let instr_len = self.instruction_mem.len();
        if instr_len > 0 {
            self.next_instruction = self.instruction_mem[instr_len - 1].clone();
        } else {
            self.next_instruction = Instr::Nop;
        }
    }

    // Fetch instruction from memory
    fn fetch(&mut self) ->Stage{
        self.set_next_instruction();
        match self.current_instruction {
            // MOV(i) are single cycle instructions
            Instr::Movi(dest, val) => {
                self.registers[dest as usize] = val;
                Stage::Fetch
            }
            Instr::Mov(dest, src) => {
                self.registers[dest as usize] = self.registers[src as usize];
                Stage::Fetch
            }
            Instr::Nop => Stage::Fetch,
            _ => Stage::Decode
        }
    }

    // TODO: some way of checking whether functional units can be issued i.e. check dependencies before calling issue
    fn issue_alutask(&mut self, instr: Instr, x: u32, y: u32, f: impl FnMut(u32, u32)->u32 + 'static) -> () {
        let mut alu = ALU::new();
        alu.issue(instr,x, y, f);
        self.task_units.push_back(Box::new(alu));
    }


    // Decode instruction in memory
    fn decode(&mut self) -> Stage {
        let current_instruction = self.current_instruction.clone();
        match self.current_instruction {
            Instr::Add(_, src1, src2) => {
                let x = self.registers[src1 as usize];
                let y = self.registers[src2 as usize];
                self.issue_alutask(current_instruction, x, y, |x, y| x + y);
                Stage::Execute
            }
            Instr::Sub(_, src1, src2) => {
                let x = self.registers[src1 as usize];
                let y = self.registers[src2 as usize];
                self.issue_alutask(current_instruction, x, y, |x, y| x - y);
                Stage::Execute
            }
            _ => Stage::Execute
        }
    }

    // Execute instruction in memory
    fn execute(&mut self) -> Stage {
        for t in self.task_units.iter_mut() {
            t.execute()
        }
        Stage::WriteBack
    }

    // Write back result
    fn writeback(&mut self) -> Stage {
        for t in self.task_units.iter() {
            let result = t.result();
            match t.instr() {
                Instr::Add(dest, _, _) => self.registers[dest as usize] = result,
                Instr::Sub(dest, _, _) => self.registers[dest as usize] = result,
                _ => ()
            }
        }
        Stage::Fetch
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::cpu::*;
    use std::collections::VecDeque;

    #[test]
    fn mov_tests(){
        let mut cpu = CPU::new();
        let mut instructions = VecDeque::new();
        instructions.push_back(Instr::Movi(2, 42));
        instructions.push_back(Instr::Mov(0, 2));
        cpu.load_instr_vec(&instructions);
        cpu.clock_tick();
        assert_eq!(cpu.ticks, 1);
        assert_eq!(cpu.registers[2], 42);
        assert_eq!(cpu.next_stage, Stage::Fetch);
        cpu.clock_tick();
        assert_eq!(cpu.ticks, 2);
        assert_eq!(cpu.registers[0], 42);
        assert_eq!(cpu.next_stage, Stage::Fetch);
    }

    #[test]
    fn add_sub_tests() {
        let mut cpu = CPU::new();
        let mut instructions = VecDeque::new();
        instructions.push_back(Instr::Movi(0, 42));
        instructions.push_back(Instr::Add(1, 0, 0));
        instructions.push_back(Instr::Sub(2, 1, 0));
        cpu.load_instr_vec(&instructions);
        cpu.run_to_end();
        assert_eq!(cpu.registers[1], 84);
        assert_eq!(cpu.registers[2], 42);
    }
}