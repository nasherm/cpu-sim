    /*
 The main logic for the CPU. This includes key pipelining decoding of instructions,
as well as execution unit interaction and the definition of the
ISA
 */
#![allow(non_snake_case)]
enum INSTR{
    MOVi,
    MOV,
    ADD,
    SUB,
    MULT,
}

pub struct CPU {
    registers: [u32; 256],
    clockTicks: u32,
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            registers: [0; 256],
            clockTicks: 0,
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

    // Fetch instruction from memory
    fn fetch(&mut self) ->() {
        self.clockTicks += 1;
        ()
    }

    // Decode instruction in memory
    fn decode(&mut self) -> () {
        self.clockTicks += 1;
        ()
    }

    // Execute instruction in memory
    fn execute(&mut self) -> () {
        self.clockTicks += 1;
        ()
    }

    // Write back result
    fn writeback(&mut self) -> () {
        self.clockTicks += 1;
        ()
    }
}
