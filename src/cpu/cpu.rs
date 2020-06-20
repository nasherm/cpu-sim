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
pub enum INSTR {
    MOVI(u32, u32), //  movi dest val (reg[dest]<-val) [dest] is a register
    MOV(u32, u32), // mov dest src (reg[dest] <- reg[src])
    ADDI(u32, u32), // addi dest val (reg[dest] <- reg[dest] + val)
    SUBI(u32, u32), // subi dest val (reg[dest] <-reg[dest] - val)
    ADDR(u32, u32, u32), // addr dest src1 src2 (reg[dest] <- reg[src1] + reg[src2])
    ADD (u32, u32, u32), // add dest val1 val2 (reg[dest] <- val1 + val2)
    SUBR(u32, u32, u32), // subr dest src1 src2 (reg[dest] <- reg[src1] - reg[src2])
    SUB (u32, u32, u32), // sub dest val1 val2 (reg[dest]<- val1 - val2)
    NOP,
}


#[derive(Debug, PartialEq)]
pub enum Stage {
    Fetch,
    Decode,
    Execute,
    WriteBack
}

pub struct CPU {
    pub instruction_mem: Vec<INSTR>,
    pub registers: [u32; 256],
    pub current_instruction: INSTR,
    pub prev_instruction: INSTR,
    pub next_instruction: INSTR,
    pub next_stage: Stage,
    ticks: u32,
    alu_tasks: VecDeque<Box<dyn Unit>>,
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            instruction_mem: Vec::new(),
            registers: [0; 256],
            current_instruction: INSTR::NOP,
            prev_instruction: INSTR::NOP,
            next_instruction: INSTR::NOP,
            next_stage: Stage::Fetch,
            ticks: 0,
            alu_tasks: VecDeque::new(),
        }
    }

    pub fn load_instr_vec(&mut self, instrs:&Vec<INSTR>) {
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

    pub fn load_instructions(&mut self, instructions: Vec<Result<INSTR, String>>) -> () {
        for res in instructions {
            match res {
                Ok(i) => self.instruction_mem.push(i),
                Err(e) => panic!("Instruction not loaded | {:?}", e),
            }
        }
    }

    fn set_next_instruction (&mut self) {
        let instr = self.instruction_mem.pop();
        self.prev_instruction = self.current_instruction.clone();
        self.current_instruction =  match instr {
            Some(i) => i,
            _ => INSTR::NOP
        };
        let instr_len = self.instruction_mem.len();
        if instr_len > 0 {
            self.next_instruction = self.instruction_mem[instr_len - 1].clone();
        } else {
            self.next_instruction = INSTR::NOP;
        }
    }

    // Fetch instruction from memory
    fn fetch(&mut self) ->Stage{
        self.set_next_instruction();
        match self.current_instruction {
            // MOV(i) are single cycle instructions
            INSTR::MOVI(dest, val) => {
                self.registers[dest as usize] = val;
                Stage::Fetch
            }
            INSTR::MOV(dest, src) => {
                self.registers[dest as usize] = self.registers[src as usize];
                Stage::Fetch
            }
            INSTR::NOP => Stage::Fetch,
            _ => Stage::Decode
        }
    }

    // TODO: some way of checking whether functional units can be issued i.e. check dependencies before calling issue
    fn issue_alutask(&mut self, x: u32, y: u32, f: impl FnMut(u32, u32)->u32 + 'static) -> () {
        let mut alu = ALU::new();
        alu.issue(x, y, f);
        self.alu_tasks.push_back(Box::new(alu));
    }


    // Decode instruction in memory
    fn decode(&mut self) -> Stage {
        match self.current_instruction {
            INSTR::ADD(_, x, y) => {
                self.issue_alutask(x, y, |x, y| x + y);
                Stage::Execute
            }
            INSTR::SUB(_, x, y) => {
                self.issue_alutask(x, y, |x, y| x - y);
                Stage::Execute
            }
            _ => Stage::Execute
        }
    }

    // Execute instruction in memory
    fn execute(&mut self) -> Stage {
        Stage::WriteBack
    }

    // Write back result
    fn writeback(&mut self) -> Stage {
        Stage::Fetch
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::cpu::*;

    #[test]
    fn mov_tests(){
        let mut cpu = CPU::new();
        cpu.load_instr_vec(&vec![
            INSTR::MOV(0, 2),
            INSTR::MOVI(2, 42)
        ]);
        cpu.clock_tick();
        assert_eq!(cpu.ticks, 1);
        assert_eq!(cpu.registers[2], 42);
        assert_eq!(cpu.next_stage, Stage::Fetch);
        cpu.clock_tick();
        assert_eq!(cpu.ticks, 2);
        assert_eq!(cpu.registers[0], 42);
        assert_eq!(cpu.next_stage, Stage::Fetch);
    }
}