use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
use std::vec::Vec;
use std::string::String;
use super::cpu::INSTR;
#[allow(non_snake_case)]
fn toInstr(v: &mut Vec<&str>) -> Result<INSTR, String>{
    let op = v[0];
    match op{
        "MOV" | "MOVI" | "ADDI" | "SUBI" =>{
            let arg2 = v.pop();
            let arg1 = v.pop();
            match (arg1, arg2){
                (Some(x), Some(y)) =>{
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val = y[1..].to_string().parse::<u32>().unwrap();
                    if op == "MOV" {
                        Ok(INSTR::MOV(dest, val))
                    }
                    else if op == "MOVI" {
                        Ok(INSTR::MOVI(dest, val))
                    }
                    else if op == "ADDI"{
                        Ok(INSTR::ADDI(dest, val))
                    }
                    else {
                        Ok(INSTR::SUBI(dest, val))
                    }
                }
                _ => Err(format!("Not enough arguments to instr = {}", op)),
            }
        }
        "ADD" | "ADDR" | "SUB" | "SUBR" => {
            let arg3 = v.pop();
            let arg2 = v.pop();
            let arg1 = v.pop();
            match (arg1, arg2, arg3){
                (Some(x), Some(y), Some(z)) => {
                    let dest = x[1..].to_string().parse::<u32>().unwrap();
                    let val1 = y[1..].to_string().parse::<u32>().unwrap();
                    let val2 = z[1..].to_string().parse::<u32>().unwrap();
                    if op == "ADD"{
                        Ok(INSTR::ADD(dest, val1, val2))
                    }
                    else if op == "SUB"{
                        Ok(INSTR::SUB(dest, val1, val2))
                    }
                    else if op == "ADDR" {
                        Ok(INSTR::ADDR(dest, val1, val2))
                    }
                    else {
                        Ok(INSTR::SUBR(dest, val1, val2))
                    }
                },
                _ => Err(format!("Not enough arguments to instr = {}", op)),
            }
        }
        _ => Err(format!("can't read instruction = {}", op)),
    }
}

pub fn parseFile(filePath: &'static str ) -> io::Result<Vec<Result<INSTR, String>>> {
    let file = File::open(filePath);
    let mut instr: Vec<Result<INSTR, String>> = Vec::new();
    match file {
        Ok(f) =>{
            let reader = BufReader::new(f);
            for split in reader.lines(){
                match split{
                    Ok(s) => {
                        let mut collect: Vec<&str> = s.split(' ').collect();
                        instr.push(toInstr(&mut collect));
                    },
                    Err(_) => (),
                }
            }
            Ok(instr)
        },
        _ => Err(Error::new(ErrorKind::Other, format!("FAILED TO OPEN FILE {}",filePath))),
    }
}
