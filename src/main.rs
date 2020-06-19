pub mod cpu;
pub mod debug;
fn main() {
    let mut debug = debug::Debugger::new();
    debug.event_loop()
}


