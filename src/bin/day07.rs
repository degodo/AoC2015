//!
//! --- Day 7: Some Assembly Required ---
//! 
//! This year, Santa brought little Bobby Tables a set of wires and bitwise
//! logic gates! Unfortunately, little Bobby is a little under the recommended
//! age range, and he needs help assembling the circuit.
//! 
//! Each wire has an identifier (some lowercase letters) and can carry a
//! 16-bit signal (a number from 0 to 65535). A signal is provided to each
//! wire by a gate, another wire, or some specific value. Each wire can only
//! get a signal from one source, but can provide its signal to multiple
//! destinations. A gate provides no signal until all of its inputs have a
//! signal.
//! 
//! The included instructions booklet describes how to connect the parts
//! together: x AND y -> z means to connect wires x and y to an AND gate,
//! and then connect its output to wire z.
//! 
//! For example:
//! 
//! 123 -> x          means that the signal 123 is provided to wire x.
//! x AND y -> z      means that the bitwise AND of wire x and wire y is
//!                   provided to wire z.
//! p LSHIFT 2 -> q   means that the value from wire p is left-shifted by 2
//!                   and then provided to wire q.
//! NOT e -> f        means that the bitwise complement of the value from
//!                   wire e is provided to wire f.
//! 
//! Other possible gates include OR (bitwise OR) and RSHIFT (right-shift).
//! If, for some reason, you'd like to emulate the circuit instead, almost
//! all programming languages (for example, C, JavaScript, or Python) provide
//! operators for these gates.
//! 
//! For example, here is a simple circuit:
//! 
//!   123 -> x
//!   456 -> y
//!   x AND y -> d
//!   x OR y -> e
//!   x LSHIFT 2 -> f
//!   y RSHIFT 2 -> g
//!   NOT x -> h
//!   NOT y -> i
//! 
//! After it is run, these are the signals on the wires:
//! 
//!   d: 72
//!   e: 507
//!   f: 492
//!   g: 114
//!   h: 65412
//!   i: 65079
//!   x: 123
//!   y: 456
//!
//! In little Bobby's kit's instructions booklet (provided as yourpuzzle input),
//! what signal is ultimately provided to wire a?
//! 
//! Your puzzle answer was 46065.
//! 
//! --- Part Two ---
//! Now, take the signal you got on wire a, override wire b to that signal,
//! and reset the other wires (including wire a). What new signal is
//! ultimately provided to wire a?
//! 
//! Your puzzle answer was 14134.




use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Assign,
    And,
    Or,
    Not,
    RShift,
    LShift,
}

fn into_operator(s: &str) -> Operator {
    match s {
        "AND"    => Operator::And,
        "OR"     => Operator::Or,
        "NOT"    => Operator::Not,
        "RSHIFT" => Operator::RShift,
        "LSHIFT" => Operator::LShift,
        &_       => Operator::Assign,
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Label(String),
    Number(i32),
}

#[derive(Debug, Clone)]
struct Instruction {
    lbl: String,
    op: Operator,
    ora: Operand,
    orb: Option<Operand>,
}

impl Instruction {
    fn new(s: &str, l: &str) -> Instruction {
        let ora: Operand;
        let orb: Option<Operand>;
        let op: Operator;
        let mut e = s.trim().split_whitespace();
        let a = e.next().unwrap();
        if let Some(b) = e.next() {
            if let Some(c) = e.next() {
                op = into_operator(b);
                // three space-separeted strings <Operand> <Operator> <Operanad>
                if let Ok(v) = a.parse::<i32>() {
                    ora = Operand::Number(v);
                } else {
                    ora = Operand::Label(a.to_owned());
                }
                if let Ok(v) = c.parse::<i32>() {
                    orb = Some(Operand::Number(v));
                } else {
                    orb = Some(Operand::Label(c.to_owned()));
                }
            } else {
                // two space-separated strings <Operator> <Operand>
                op = into_operator(a);
                orb = None;
                if let Ok(v) = b.parse::<i32>() {
                    ora = Operand::Number(v);
                } else {
                    ora = Operand::Label(b.to_owned());
                }
            }
        } else {
            // only one space-separated string means a label or an scalar
            op = Operator::Assign;
            orb = None;
            if let Ok(v) = a.parse::<i32>() {
                ora = Operand::Number(v);
            } else{
                ora = Operand::Label(a.to_owned());
            }
        }
        Instruction{lbl: l.to_owned(), op, ora, orb}
    }
    
}


#[derive(Debug, Clone)]
struct Command {
    lhs: String,
    rhs: String,
}

impl Command {
    fn new(line: &str) -> Command {
        let mut l = line.trim().split("->");
        Command {
            lhs: l.next().unwrap().trim().to_owned(),
            rhs: l.next().unwrap().trim().to_owned() 
        }
    }
}


fn do_logical_and(ora: &Operand, orb: &Operand) -> Operand {
    let Operand::Number(a) = *ora else { todo!( )};
    let Operand::Number(b) = *orb else { todo!( )};
    Operand::Number( a & b)
} 

fn do_logical_or(ora: &Operand, orb: &Operand) -> Operand {
    let Operand::Number(a) = *ora else { todo!( )};
    let Operand::Number(b) = *orb else { todo!( )};
    Operand::Number( a | b)
} 

fn do_logical_not(ora: &Operand) -> Operand {
    let Operand::Number(a) = *ora else { todo!( )};
    Operand::Number( !a )
} 

fn do_logical_lshift(ora: &Operand, orb: &Operand) -> Operand {
    let Operand::Number(a) = *ora else { todo!( )};
    let Operand::Number(b) = *orb else { todo!( )};
    Operand::Number( a << b)
} 

fn do_logical_rshift(ora: &Operand, orb: &Operand) -> Operand {
    let Operand::Number(a) = *ora else { todo!( )};
    let Operand::Number(b) = *orb else { todo!( )};
    Operand::Number( a >> b)
} 


fn eval_instructions(instructions: &HashMap<String, Instruction>, sig: &str) -> i32{
    let mut stack: Vec<Instruction> = Vec::new();
    let mut results: HashMap<String, Operand> = HashMap::new();
    let mut inst = instructions.get(sig).unwrap().clone();
    let mut res: Operand;
    
    loop {
        if let Operand::Label(l) = &inst.ora {
            if let Some(r) = results.get(l) {
                inst.ora = r.clone();
            } else {
                stack.push(inst.clone());
                inst = instructions.get(l).unwrap().clone();
            }
        } else if let Some(Operand::Label(l)) = inst.orb.as_ref() {
            if let Some(r) = results.get(l) {
                inst.orb = Some(r.clone());
            } else {
                stack.push(inst.clone());
                inst = instructions.get(l).unwrap().clone();
            }
        } else {
            if Operator::Assign == inst.op {
               res = inst.ora;
            } else if Operator::Not == inst.op {
                res = do_logical_not(&inst.ora);
            } else if Operator::And == inst.op {
                res = do_logical_and(&inst.ora, inst.orb.as_ref().unwrap());
            } else if Operator::Or == inst.op {
                res = do_logical_or(&inst.ora, inst.orb.as_ref().unwrap());
            } else if Operator::RShift == inst.op {
                res = do_logical_rshift(&inst.ora, inst.orb.as_ref().unwrap());
            } else if Operator::LShift == inst.op {
                res = do_logical_lshift(&inst.ora, inst.orb.as_ref().unwrap()); 
            } else {
                println!("unhandled operator");
                break;
            }
            results.insert(inst.lbl.clone(), res.clone());
            if stack.is_empty() {
                break;
            }
            inst = stack.pop().unwrap();
            if let Operand::Label(_) = inst.ora {
                inst.ora = res;
            } else if let Some(a) = inst.orb.as_ref() {
                if let Operand::Label(_) = a {
                    inst.orb = Some(res);
                }
            } else {
                println!{"error"};
            }
        }
    }
    if let Operand::Number(n) = results.get(sig).unwrap() {*n} else {-1}
}


fn read_input<P, R>(filename: P, s: fn(&str) -> R) -> Vec<R>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let x = buf.lines();
    let y = x.map(|l| s(l.unwrap().as_ref())); 
    y.collect()
}

fn main (){
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    let actions: Vec<Command> = read_input("data/07/input.txt", Command::new);
    actions.iter().for_each(|e| {instructions.insert(e.rhs.clone(), Instruction::new(e.lhs.as_ref(), &e.rhs));} );
    let sig_a = eval_instructions(&instructions, "a");
    println!("Leitung a hat das Signal {:4}", sig_a);
    instructions.get_mut("b").unwrap().ora = Operand::Number(sig_a);
    println!("Leitung a hat nach {} -> b das Signal {:4}", sig_a, eval_instructions(&instructions,"a"));
}