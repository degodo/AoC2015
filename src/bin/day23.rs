//! --- Day 23: Opening the Turing Lock ---
//! 
//! Little Jane Marie just got her very first computer for Christmas from some
//! unknown benefactor. It comes with instructions and an example program, but
//! the computer itself seems to be malfunctioning. She's curious what the
//! program does, and would like you to help her run it.
//! 
//! The manual explains that the computer supports two registers and six
//! instructions (truly, it goes on to remind the reader, a state-of-the-art
//! technology). The registers are named a and b, can hold any non-negative
//! integer, and begin with a value of 0. The instructions are as follows:
//! 
//!     hlf r -- sets register r to half its current value, then continues
//!              with the next instruction.
//!     tpl r -- sets register r to triple its current value, then continues 
//!              with the next instruction.
//!     inc r -- increments register r, adding 1 to it, then continues with
//!              the next instruction.
//!     jmp offset -- is a jump; it continues with the instruction offset away
//!              relative to itself.
//!     jie r, offset -- is like jmp, but only jumps if register r is even
//!              ("jump if even").
//!     jio r, offset -- is like jmp, but only jumps if register r is 1
//!              ("jump if one", not odd).
//! 
//! All three jump instructions work with an offset relative to that
//! instruction. The offset is always written with a prefix + or - to
//! indicate the direction of the jump (forward or backward, respectively).
//! For example, jmp +1 would simply continue with the next instruction, while
//! jmp +0 would continuously jump back to itself forever.
//! 
//! The program exits when it tries to run an instruction beyond the ones
//! defined.
//! 
//! For example, this program sets a to 2, because the jio instruction causes
//! it to skip the tpl instruction:
//! 
//!     inc a
//!     jio a, +2
//!     tpl a
//!     inc a
//! 
//! What is the value in register b when the program in your puzzle input is
//! finished executing?
//! 
//! Ansmer: 170
//! 
//! --- Part Two ---
//! 
//! The unknown benefactor is very thankful for releasier, helping little
//! Jane Marie with her computer. Definitely not to distract you, what is
//! the value in register b after the program is finished executing if
//! register a starts as 1 instead?
//! 
//! Answer: 247


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum OpCode {
    Half_A,
    Half_B,
    Tpl_A,
    Tpl_B,
    Inc_A,
    Inc_B,
    jmp(i32),
    jie_a(i32),
    jie_b(i32),
    jio_a(i32),
    jio_b(i32),
    invalid,
}

fn decode(s: &str) -> OpCode {
    let mut e = s.split_whitespace();
    let a = e.next().unwrap();
    let b = e.next().unwrap();

    match (a, b) {
        ("hlf", "a") => OpCode::Half_A,
        ("hlf", "b") => OpCode::Half_B,
        ("tpl", "a")  => OpCode::Tpl_A,
        ("tpl", "b")  => OpCode::Tpl_B,
        ("inc", "a")  => OpCode::Inc_A,
        ("inc", "b")  => OpCode::Inc_B,
        ("jmp", _) => OpCode::jmp(b.parse().unwrap()),
        ("jie", "a,") => OpCode::jie_a(e.next().unwrap().parse().unwrap()),
        ("jie", "b,") => OpCode::jie_b(e.next().unwrap().parse().unwrap()),
        ("jio", "a,") => OpCode::jio_a(e.next().unwrap().parse().unwrap()),
        ("jio", "b,") => OpCode::jio_b(e.next().unwrap().parse().unwrap()),
        _      => OpCode::invalid,
    }
}

fn adjust_pc(pc: &mut usize, delta: i32) {
    if delta < 0 {
        *pc -= delta.abs() as usize;
    } else {
        *pc += delta as usize;
    }
}

fn read_input<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let f = BufReader::new(File::open(name).unwrap());
    f.lines().map(|x| x.unwrap()).collect()
}

fn eval(code: &Vec<String>, init_a: u32, init_b: u32) -> (u32, u32) {
    let mut pc: usize = 0;
    let mut reg_a: u32 = init_a;
    let mut reg_b: u32 = init_b;
    let code_size: usize = code.len();
    
    while pc < code_size {
        match decode(&code[pc]) {
            OpCode::Half_A => { reg_a /= 2; pc += 1; },
            OpCode::Half_B => { reg_b /= 2; pc += 1;},
            OpCode::Tpl_A => { reg_a *= 3; pc += 1; },
            OpCode::Tpl_B => { reg_b *= 3; pc += 1; },
            OpCode::Inc_A => { reg_a += 1; pc += 1; },
            OpCode::Inc_B => { reg_b += 1; pc += 1; },
            OpCode::jmp(w) => { adjust_pc(&mut pc, w); },
            OpCode::jie_a(w) => { if reg_a > 0 && reg_a % 2 == 0  { adjust_pc(&mut pc, w); } else { pc += 1; }; },
            OpCode::jie_b(w) => { if reg_b > 0 && reg_b % 2 == 0  { adjust_pc(&mut pc, w); } else { pc += 1; }; },
            OpCode::jio_a(w) => { if reg_a == 1 { adjust_pc(&mut pc, w); } else { pc += 1; }; },
            OpCode::jio_b(w) => { if reg_a == 1 { adjust_pc(&mut pc, w); } else { pc += 1; }; },
            OpCode::invalid => panic!(),
        }
    }
    (reg_a, reg_b)
}

fn main() {
    let code: Vec<String> = read_input("data/23/input.txt");
    let mut reg_a: u32 = 0;
    let mut reg_b: u32 = 0;
    (reg_a, reg_b) = eval(&code, 0, 0);
    println!("Part 1: reg_a: {}, reg_b: {}", reg_a, reg_b);
    (reg_a, reg_b) = eval(&code, 1, 0);
    println!("Part 2: reg_a: {}, reg_b: {}", reg_a, reg_b);
}