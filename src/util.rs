pub mod util {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
    use std::vec::Vec;
    use std::result;
    use crate::cpu::cpu::INSTR;

    fn toInstr(v: Vec<&str>) -> Result<INSTR, &'static str>{
        match v[0]{
            _ => Err("can't read instruction"),
        }
    }

    pub fn parseFile(filePath: &'static str ) -> io::Result<Vec<Result<INSTR, &'static str>>> {
        let file = File::open(filePath);
        let mut instr: Vec<Result<INSTR, &'static str>> = Vec::new();
        match file {
            Ok(f) =>{
                let reader = BufReader::new(f);
                for split in reader.lines(){
                    match split{
                        Ok(s) => {
                            let collect: Vec<&str> = s.split(' ').collect();
                            instr.push(toInstr(collect));
                        },
                        Err(_) => (),
                    }
                }
                Ok(instr)
            },
            _ => Err(Error::new(ErrorKind::Other, format!("FAILED TO OPEN FILE {}",filePath))),
        }
    }
}
