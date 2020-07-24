use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
use std::vec::Vec;
use std::string::String;
use super::cpu::Instr;
fn string_to_instr(op: &str)-> (fn(u32, u32) -> Instr){
    match op {
        "cmp" => Instr::Cmp,
        "cmpi" => Instr::Cmpi,
        "mov" => Instr::Mov,
        "movi" => Instr::Movi,
        "addi" => Instr::Addi,
        "subi" => Instr::Subi,
        _ => |_, _| {Instr::Nop},
    }
}

fn string_to_3arginstr(op: &str)-> (fn(u32, u32, u32) -> Instr){
    match op {
        "add" => Instr::Add,
        "addr" => Instr::Addr,
        "sub" => Instr::Sub,
        "subr" => Instr::Subr,
        _ => |_, _, _| {Instr::Nop},
    }
}

fn to_instr(v: &mut Vec<&str>) -> Result<Instr, String>{
    match v[0]{
        "cmp"  |
        "cmpi" |
        "mov"  |
        "movi" |
        "addi" |
        "subi" =>{
            match (v.pop(), v.pop()){
                (Some(y), Some(x)) =>{
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val  = y[1..].to_string().parse::<u32>().unwrap();
                    Ok(string_to_instr(v[0])(dest, val))
                }
                _ => Err(format!("Not enough arguments to instr = {}", v[0])),
            }
        },
        "add" |
        "addr"|
        "sub" |
        "subr" => {
            match (v.pop(), v.pop(), v.pop()){
                (Some(z), Some(y), Some(x)) => {
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val1 = y[1..].to_string().parse::<u32>().unwrap();
                    let val2 = z[1..].to_string().parse::<u32>().unwrap();
                    Ok(string_to_3arginstr(v[0])(dest, val1, val2))
                },
                _ => Err(format!("Not enough arguments to instr = {}", v[0])),
            }
        },
        _ => Err(format!("can't read instruction = {}", v[0])),
    }
}

pub fn parse_file(file_path: &str ) -> io::Result<Vec<Result<Instr, String>>> {
    let file = File::open(file_path);
    let mut instr= Vec::new();
    match file {
        Ok(f) =>{
            let reader = BufReader::new(f);
            for split in reader.lines(){
                match split{
                    Ok(s) => {
                        let mut collect = s.split(' ').collect();
                        instr.push(to_instr(&mut collect));
                    },
                    Err(_) => (),
                }
            }
            Ok(instr)
        },
        _ => Err(Error::new(ErrorKind::Other, format!("FAILED TO OPEN FILE {}", file_path))),
    }
}

pub fn parse_string(string:&String) -> Result<Instr, String> {
    let mut collect = string.split(' ').collect();
    to_instr(&mut collect)
}

#[cfg(test)]
mod tests{
    use crate::cpu::util::*;
    use crate::cpu::cpu::Instr;

    #[test]
    fn parse_mov() {
        let mut instr = vec!["mov", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Mov(0, 1)) => (),
            _ => panic!(format!("Failed to parse MOV instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_movi() {
        let mut instr = vec!["movi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Movi(0, 1)) => (),
            _ => panic!(format!("Failed to parse MOVI instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_addi() {
        let mut instr = vec!["addi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Addi(0, 1)) => (),
            _ => panic!(format!("Failed to parse ADDi instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_subi() {
        let mut instr = vec!["subi", "r0", "#1"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Subi(0, 1)) => (),
            _ => panic!(format!("Failed to parse SUBI instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_add() {
        let mut instr = vec!["add", "r0", "#1", "#2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Add(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse ADD instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_addr() {
        let mut instr = vec!["addr", "r0", "r1", "r2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Addr(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse ADDR instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_sub() {
        let mut instr = vec!["sub", "r0", "#1", "#2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Sub(0, 1, 2)) => (),
            _ => panic!(format!("Failed to parse SUB instruction = {:?}", instr))
        }
    }
    #[test]
    fn parse_subr() {
        let mut instr = vec!["subr", "r0", "r1", "r2"];
        let r = to_instr(&mut instr);
        match r{
            Ok(Instr::Subr(0, 1, 2)) => (),
            _ => panic!("Failed to parse SUBR instruction = {:?}", instr)
        }
    }

    #[test]
    fn parse_src_file() {
        let res = parse_file("./src/programs/arith_mov");
        let expected = vec![
            Instr::Movi(0, 42),
            Instr::Mov(0, 0),
            Instr::Addr(0, 0, 0),
            Instr::Addi(0, 42),
            Instr::Add(0, 42, 42),
            Instr::Subr(0, 0, 0),
            Instr::Subi(0, 42),
            Instr::Sub(0, 42, 42)
        ];
        match res {
            Ok(v) => {
                assert_eq!(v.len(), expected.len());
                for (l, r) in expected.into_iter().zip(v){
                    match r{
                        Ok(instr) => assert_eq!(l, instr),
                        Err(e) => panic!("Failed to parse string {}", e),
                    }
                }
            }
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn parse_cmp() {
        let mut instr = vec!["cmp", "r0", "r1"];
        let result = to_instr(&mut instr);
        match result {
            Ok(instr) => assert_eq!(instr, Instr::Cmp(0, 1)),
            Err(e) => panic!("Failed to parse cmp instruction: {}", e),
        }
    }

    #[test]
    fn parse_cmpi(){
        let mut instr = vec!["cmpi", "r0", "#1"];
        let result = to_instr(&mut instr);
        match result {
            Ok(instr) => assert_eq!(instr, Instr::Cmpi(0, 1)),
            Err(e) => panic!("Failed to parse cmpi instruction: {}", e),
        }
    }
}
