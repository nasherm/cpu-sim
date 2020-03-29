use std::boxed::Box;
pub trait Unit{
    fn execute(&mut self) -> ();
}

pub struct ALU{
    x: u32,
    y: u32,
    r: u32,
    f: Box<dyn FnMut(u32, u32) -> u32>
}

impl Unit for ALU{
    fn execute(&mut self)->(){
        self.r = (self.f)(self.x, self.y);
    }
}

impl ALU{
    pub fn new()-> Self{
        ALU{
            x: 0,
            y: 0,
            r: 0,
            f: Box::new(|_, _| 0)
        }
    }
    pub fn issue(&mut self, x: u32, y: u32, f: impl FnMut(u32, u32) -> u32 + 'static) -> (){
        self.x = x;
        self.y = y;
        self.f = Box::new(f);
    }
}
