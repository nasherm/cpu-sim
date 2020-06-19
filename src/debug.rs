use crate::cpu::cpu;
use text_io::read;
use std::io;
use std::io::Write;
use std::iter::Iterator;
pub struct Debugger{
    cpu: cpu::CPU,
}

impl Debugger{
    pub fn new() -> Debugger {
        Debugger{
            cpu: cpu::CPU::new(),
        }
    }

    fn print_commands(&self){
        println!("\
        q - quit\n\
        h - help\n\
        r - print registers\n\
        i - instruction memory\n\
        n - execute a clock step\n\
        c - cpu state\n\
        a - add instruction \n");
    }

    fn print_collection<T>(&self, collection:std::slice::Iter<'_, T>, line_size: usize)
    where T:std::fmt::Debug
    {
        let mut line: Vec<String> = Vec::new();
        let mut index = 1;
        for (i, val) in collection.enumerate(){
            let s = format!("reg[{}] ={:?}", i, val);
            line.push(s.clone());

            if index % line_size == 0 {
                println!("{:?}", line);
                index = 1;
                line = Vec::new();
            } else {
                index += 1;
            }
        }
        if self.cpu.registers().len()% line_size != 0 {
            println!("{:?}", line);
        }
    }

    fn print_registers(&self, line_size: usize) {
        self.print_collection(self.cpu.registers.iter(), line_size);
    }

    fn print_instruction_mem(&self, line_size: usize) {
        self.print_collection(self.cpu.instruction_mem.iter(), line_size)
    }

    fn clock_tick(&mut self) {
        println!("INSTRUCTION = {:?}, STAGE={:?}", self.cpu.current_instruction, self.cpu.next_stage);
        self.cpu.clock_tick();
    }

    fn print_cpu_state(&self) {
        println!("PREV INSTRUCTION = {:?}", self.cpu.prev_instruction);
        println!("CURRENT INSTRUCTION = {:?}, STAGE={:?}", self.cpu.current_instruction, self.cpu.next_stage);
        println!("NEXT INSTRUCTION = {:?}", self.cpu.next_instruction);
    }

    pub fn event_loop(&mut self) -> () {
        println!("Welcome to the debugger");
        loop {
            print!("+> ");
            io::stdout().flush();
            let line:String = read!("{}\n");
            match line.as_str(){
                "q" => break,
                "h" => self.print_commands(),
                "r" => self.print_registers(10),
                "i" => self.print_instruction_mem(10),
                "n" => self.clock_tick(),
                "c" => self.print_cpu_state(),
                _ => println!("Couldn't decode instruction")
            }
        }
    }
}