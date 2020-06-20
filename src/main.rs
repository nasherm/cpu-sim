pub mod cpu;
pub mod debugger;
fn main() {
    let mut debug = debugger::Debugger::new();
    debug.event_loop()
}


