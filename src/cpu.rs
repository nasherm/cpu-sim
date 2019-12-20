pub mod cpu{
    use std::vec::Vec;
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
    }

    pub struct CPU {
        registers: [i32; 256],
        clockTicks: u32,
        instructionMem: Vec<INSTR>,
    }

    impl CPU{
        pub fn new() -> CPU{
            CPU{
                registers: [0; 256],
                clockTicks: 0,
                instructionMem: Vec::new(),
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
}
