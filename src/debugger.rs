use crate::cpu::{cpu, util};
use text_io::read;
use std::io;
use std::io::Write;
use std::iter::Iterator;
use std::borrow::Borrow;

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
        a - add instruction \
        l - load source file");
    }

    fn print_collection<T>(&self, collection:std::slice::Iter<'_, T>, line_size: usize)
    where T:std::fmt::Debug
    {
        let mut line: Vec<String> = Vec::new();
        let mut index = 1;
        for (i, val) in collection.enumerate(){
            let s = format!("addr[{}] ={:?}", i, val);
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
        self.print_commands();
        loop {
            print!("+> ");
            let _ = io::stdout().flush();
            let line: String = read!("{}\n");
            match line.as_str(){
                "q" => break,
                "h" => self.print_commands(),
                "r" => self.print_registers(10),
                "i" => self.print_instruction_mem(5),
                "n" => self.clock_tick(),
                "c" => self.print_cpu_state(),
                "a" => {
                    print!("-> ");
                    let _ = io::stdout().flush();
                    let instr_string: String = read!("{}\n");
                    let parsed_instr = util::parse_string(&instr_string);
                    match parsed_instr {
                        Ok(i) => self.cpu.instruction_mem.push(i),
                        Err(e) => println!("failed to parse instruction {}", e.as_str()),
                    }
                },
                "l" => {
                    print!("FILE <- ");
                    let _ = io::stdout().flush();
                    let file_name: String = read!("{}\n");
                    let src_parsed = util::parse_file(file_name.as_str());
                    match src_parsed{
                        Ok(v) => self.cpu.load_instructions(v),
                        Err(e) => println!("failed to load source file"),
                    }
                },
                _ => println!("Couldn't decode instruction")
            }
        }
    }
}