pub mod cpu;
fn main() {
    let mut cpu = cpu::cpu::CPU::new();
    let parse = cpu::util::parseFile("tests/arith_mov");
    match parse{
        Ok(v) => cpu.loadInstructions(v),
        Err(e) => println!("{:?}", e),
    }
    cpu.fdew()
}
