//!
//! --- Day 8: Matchsticks ---
//!
//! Space on the sleigh is limited this year, and so Santa will be bringing
//! his list as a digital copy. He needs to know how much space it will take
//! up when stored.
//!
//! It is common in many programming languages to provide a way to escape
//! special characters in strings. For example, C, JavaScript, Perl, Python,
//! and even PHP handle special characters in very similar ways.
//!
//! However, it is important to realize the difference between the number of
//! characters in the code representation of the string literal and the number
//! of characters in the in-memory string itself.
//!
//! For example:
//!
//! ""          is 2 characters of code (the two double quotes), but the
//!             string contains zero characters.
//! "abc"       is 5 characters of code, but 3 characters in the string data.
//! "aaa\"aaa"  is 10 characters of code, but the string itself contains six
//!             "a" characters and a single, escaped quote character, for a
//!             total of 7 characters in the string data.
//! "\x27"      is 6 characters of code, but the string itself contains just
//!             one - an apostrophe ('), escaped using hexadecimal notation.
//!
//! Santa's list is a file that contains many double-quoted string literals,
//! one on each line. The only escape sequences used are \\ (which represents
//! a single backslash), \" (which represents a lone double-quote character),
//! and \x plus two hexadecimal characters (which represents a single
//! character with that ASCII code).
//!
//! Disregarding the whitespace in the file, what is the number of characters
//! of code for string literals minus the number of characters in memory for
//! the values of the strings in total for the entire file?
//!
//! For example, given the four strings above, the total number of characters
//! of string code (2 + 5 + 10 + 6 = 23) minus the total number of characters
//! in memory for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.
//! 
//! Your puzzle answer was 1350.
//! 
//! --- Part Two ---
//! 
//! Now, let's go the other way. In addition to finding the number of
//! characters of code, you should now encode each code representation as a
//! new string and find the number of characters of the new encoded
//! representation, including the surrounding double quotes.
//! 
//! For example:
//! "" encodes to "\"\"", an increase from 2 characters to 6.
//! "abc" encodes to "\"abc\"", an increase from 5 characters to 9.
//! "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.
//! "\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.
//! 
//! Your task is to find the total number of characters to represent the newly
//! encoded strings minus the number of characters of code in each original
//! string literal. For example, for the strings above, the total encoded
//! length (6 + 9 + 16 + 11 = 42) minus the characters in the original code
//! representation (23, just like in the first part of this puzzle) is
//! 42 - 23 = 19.
//! 
//! Your puzzle answer was 2085.



use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn count(s: &str) -> u32 {
    let mut count: usize = 0;
    let mut idx: usize = 0;
    let last: usize = s.len() - 1;
    let ch = s.as_bytes();
    
    if ch[0] == '\"' as u8 {
        idx +=1;
    }
    if ch[last] != '\"' as u8 {
        println!("error at line end");
        return 0 as u32;
    }
    
    while idx < last {
        if ch[idx] == '\\' as u8 {
            if ch[idx+1] == '\\' as u8 || ch[idx+1] == '\"' as u8 {
                idx += 2;
            } else if ch[idx+1] == 'x' as u8 {
                idx += 4;
            } else {
                println!("error");
            }
        } else {
            idx += 1;
        }
        count += 1;
    }
    (s.len() - count) as u32
}

fn expand(s: &str) -> u32 {

    fn inflate(c: u8) -> String {
        if c == '\\' as u8 {
            "\\\\".to_owned()
        } else if c == '\"' as u8 {
            "\\\"".to_owned()
        } else {
            let a = &[c;1];
            String::from_utf8_lossy(a).to_string()
        }
    }

    let r = s.bytes().map(inflate).collect::<String>();
    (r.len() + 2 - s.len()) as u32
}

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

fn main() {
    let actions: Vec<u32> = read_input("data/08/input.txt", count);
    println!("Ergebnis Teil 1: {}", actions.iter().sum::<u32>());
    let actions: Vec<u32> = read_input("data/08/input.txt", expand);
    println!("Ergebnis Teil 2: {}", actions.iter().sum::<u32>());
}
