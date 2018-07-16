pub mod instructions;
use instructions::*;

use std::env::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let run_args : Vec<String> = args().collect();
    let mut idx = 1;
    let mut inp_file = "roms/tapereader.chip8";
    let mut out_file = "a.c8";
    while idx < run_args.len() {
        let cur_arg = &run_args[idx];
        if cur_arg == "-o" || cur_arg == "--output" {
            idx += 1;
            out_file = &run_args[idx];
        }
        else {
            inp_file = cur_arg;
        }
        idx += 1;
    }
    
    let mut inp_fobj = File::open(inp_file).unwrap();
    let mut source = String::new();
    let _read_result = inp_fobj.read_to_string(&mut source).unwrap();


    let label_map = parse_labels(&source);
    let mut code : Vec<u8> = Vec::new();

    for ln in source.lines() {
        let instr = Instruction::parse_args(ln).unwrap().resolve_labels(&label_map);
        let opc = instr.to_opcode();
        code.push((opc & 0xFF00 >> 8) as u8);
        code.push((opc & 0xFF) as u8);
    }

    let mut out_fobj = File::create(out_file).unwrap();

    let _write_result = out_fobj.write_all(&mut code).unwrap();

}