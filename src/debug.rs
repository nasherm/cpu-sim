use crate::cpu::cpu;
use text_io::read;
use std::io;
use std::io::Write;

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
        c - cpu state\n")
    }


    fn print_registers(&self) {
        const LINE_SIZE:usize = 10;
        let mut line: Vec<String> = Vec::new();
        let mut index = 1;
        for (i, val) in self.cpu.registers().iter().enumerate(){
            let s = format!("reg[{}] ={}", i, val);
            line.push(s.clone());

            if index % LINE_SIZE == 0 {
                println!("{:?}", line);
                index = 1;
                line = Vec::new();
            } else {
                index += 1;
            }
        }
        if self.cpu.registers().len()%LINE_SIZE != 0 {
            println!("{:?}", line);
        }
    }

    pub fn event_loop(&self) -> () {
        println!("Welcome to the debugger");
        loop {
            print!("+> ");
            io::stdout().flush();
            let line:String = read!("{}\n");
            match line.as_str(){
                "q" => break,
                "h" => self.print_commands(),
                "r" => self.print_registers(),
                _ => ()
            }
        }
    }
}