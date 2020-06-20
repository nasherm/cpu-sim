pub mod cpu;
pub mod programs;
fn main() {
    let mut debug = programs::Debugger::new();
    debug.event_loop()
}


