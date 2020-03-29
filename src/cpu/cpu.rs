use std::vec::Vec;
use std::string::String;
use super::units::*;
use std::collections::VecDeque;
#[allow(non_snake_case)]
/*
The main logic for the CPU. This includes key pipelining decoding of instructions,
as well as execution unit interaction and the definition of the
ISA
    */
#[derive(Debug)]
pub enum INSTR{
    MOVI(u32, u32), //  MOVI dest val (reg[dest]<-val) [dest] is a register
    MOV(u32, u32), // MOV dest src (reg[dest] <- reg[src])
    ADDI(u32, u32), // ADDI dest val (reg[dest] <- reg[dest] + val)
    SUBI(u32, u32), // SUBI dest val (reg[dest] <-reg[dest] - val)
    ADDR(u32, u32, u32), // ADDR dest src1 src2 (reg[dest] <- reg[src1] + reg[src2])
    ADD (u32, u32, u32), // ADD dest val1 val2 (reg[dest] <- val1 + val2)
    SUBR(u32, u32, u32), // SUBR dest src1 src2 (reg[dest] <- reg[src1] - reg[src2])
    SUB (u32, u32, u32), // SUB dest val1 val2 (reg[dest]<- val1 - val2)
    NOP,
}

struct Clock{
    state: u8,
    clockTicks: u32,
}

impl Clock{
    pub fn new() -> Clock{
        Clock{
            state: 0,
            clockTicks: 0,
        }
    }

    pub fn tick(&mut self) -> (){
        self.state = (self.state + 1) % 4;
        self.clockTicks += 1;
    }
}

pub struct CPU {
    registers: [u32; 256],
    clock: Clock,
    instructionMem: Vec<INSTR>,
    currentInstruction: INSTR,
    aluTasks: VecDeque<Box<dyn Unit>>,
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            registers: [0; 256],
            clock: Clock::new(),
            instructionMem: Vec::new(),
            currentInstruction: INSTR::NOP,
            aluTasks: VecDeque::new(),
        }
    }

    // fetch decode execute writeback
    pub fn fdew(&mut self)->(){
        self.fetch();
        self.decode();
        self.execute();
        self.writeback();
        ()
    }

    pub fn loadInstructions(&mut self, instructions: Vec<Result<INSTR, String>>) -> () {
        for res in instructions {
            match res {
                Ok(i) => self.instructionMem.push(i),
                Err(e) => panic!("Instruction not loaded | {:?}", e),
            }
        }
    }

    // Fetch instruction from memory
    fn fetch(&mut self) ->() {
        match self.instructionMem.pop() {
            Some(i) => {
                self.currentInstruction = i
            },
            None => (),
        }
        self.clock.tick();
    }
    // TODO: some way of checking whether functional units can be issued i.e. check dependencies before calling issue
    fn issueALUTask(&mut self, x: u32, y: u32, f: impl FnMut(u32, u32)->u32 + 'static) -> () {
        // TODO: keeping track of out-of-order execution results via reorder-buffer
        let mut alu = ALU::new();
        alu.issue(x, y, f);
        self.aluTasks.push_back(Box::new(alu));
    }


    // Decode instruction in memory
    fn decode(&mut self) -> () {
        match self.currentInstruction{
            INSTR::MOVI(dest, val) =>
                self.registers[dest as usize] = val,
            INSTR::MOV(dest, src) =>
                self.registers[dest as usize] = self.registers[src as usize],
            INSTR::ADD(_, x, y) =>
                self.issueALUTask(x, y, |x, y| x + y),
            INSTR::SUB(_, x, y) =>
                self.issueALUTask(x, y, |x, y| x - y),
            _ => ()
        }
        self.clock.tick();
    }

    // Execute instruction in memory
    fn execute(&mut self) -> () {
        self.clock.tick();
    }

    // Write back result
    fn writeback(&mut self) -> () {
        self.clock.tick();
    }
}
