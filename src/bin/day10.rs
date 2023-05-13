//!
//!  --- Day 10: Elves Look, Elves Say ---
//! 
//! Today, the Elves are playing a game called look-and-say. They take turns
//! making sequences by reading aloud the previous sequence and using that
//! reading as the next sequence. For example, 211 is read as "one two, two
//! ones", which becomes 1221 (1 2, 2 1s).
//! Look-and-say sequences are generated iteratively, using the previous
//! value as input for the next step. For each step, take the previous value,
//! and replace each run of digits (like 111) with the number of digits (3)
//! followed by the digit itself (1).
//! 
//! For example:
//! 
//! 1 becomes 11 (1 copy of digit 1).
//! 11 becomes 21 (2 copies of digit 1).
//! 21 becomes 1211 (one 2 followed by one 1).
//! 1211 becomes 111221 (one 1, one 2, and two 1s).
//! 111221 becomes 312211 (three 1s, two 2s, and one 1).
//! 
//! Starting with the digits in your puzzle input, apply this process 40
//! times. What is the length of the result?
//! 
//! Your puzzle input is 1113122113.
//! My Answer is: 360154
//! 
//! --- Part 2 ---
//! Neat, right? You might also enjoy hearing John Conway talking about this
//! sequence (that's Conway of Conway's Game of Life fame).
//! (https://www.youtube.com/watch?v=ea7lJkEhytA)
//! 
//! Now, starting again with the digits in your puzzle input, apply this
//! process 50 times. What is the length of the new result?
//! 
//! My Answer: 5103798



use std::io::{BufRead, BufReader};
use std::fs::File;
use std::iter::Peekable;
use std::ops::Index;
use std::str::Chars;

fn read_input<P, R>(filename: P, s: fn(&str) -> R) -> Vec<R>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let x = buf.lines();
    let y = x.map(|l| s(l.unwrap().as_ref()));
    y.collect()
}

fn part1() {
    let mut datafrom:String = String::new();
    let mut datato:String = String::new();
    let mut chars: Peekable<Chars>;
    let mut count: u32 = 1;

    datafrom.push_str("1113122113");
    
    for round in 1..42 {
        let mut ch: char;
        chars = datafrom.chars().peekable();
        datato.clear();
        while let Some(ch) = chars.next() {
            if chars.peek().is_some() {
                if ch.ne(chars.peek().unwrap()) {
                    datato.push(char::from_digit(count, 10).unwrap());
                    datato.push(ch);
                    count = 1;
                } else {
                    count += 1;
                }
            } else {
                datato.push(char::from_digit(count, 10).unwrap());
                datato.push(ch);
            }
        }
		(datafrom, datato) = (datato, datafrom);
    }
    println!("Answer part I: {}", datato.len());
}

fn part2() {
    let mut datafrom:String = String::new();
    let mut datato:String = String::new();
    let mut chars: Peekable<Chars>;
    let mut count: u32 = 1;

    datafrom.push_str("1113122113");
    
    for round in 1..52 {
        let mut ch: char;
        chars = datafrom.chars().peekable();
        datato.clear();
        while let Some(ch) = chars.next() {
            if chars.peek().is_some() {
                if ch.ne(chars.peek().unwrap()) {
                    datato.push(char::from_digit(count, 10).unwrap());
                    datato.push(ch);
                    count = 1;
                } else {
                    count += 1;
                }
            } else {
                datato.push(char::from_digit(count, 10).unwrap());
                datato.push(ch);
            }
        }
		(datafrom, datato) = (datato, datafrom);
    }
    println!("Answer part II: {}", datato.len());
}

fn main() {
    part1();
    part2();
}
