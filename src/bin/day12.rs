//!
//! --- Day 12: JSAbacusFramework.io ---
//! 
//! Santa's Accounting-Elves need help balancing the books after a recent
//! order. Unfortunately, their accounting software uses a peculiar storage
//! format. That's where you come in.
//! 
//! They have a JSON document which contains a variety of things:
//! arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings.
//! Your first job is to simply find all of the numbers throughout the
//! document and add them together.
//! 
//! For example:
//! [1,2,3] and {"a":2,"b":4} both have a sum of 6.
//! [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
//! {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
//! [] and {} both have a sum of 0.
//! You will not encounter any strings containing numbers.
//! 
//! What is the sum of all numbers in the document?
//! 
//! Answer: 111754
//! 
//! 
//! --- Part Two ---
//! 
//! Uh oh - the Accounting-Elves have realized that they double-counted
//! everything red.
//! 
//! Ignore any object (and all of its children) which has any property with
//! the value "red". Do this only for objects ({...}), not arrays ([...]).
//! 
//! [1,2,3] still has a sum of 6.
//! [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is
//! ignored.
//! {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire
//! structure is ignored.
//! [1,"red",5] has a sum of 6, because "red" in an array has no effect.
//! 
//! Answer: 65402
//! 

use std::collections::{HashSet, BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::ops::Index;
use std::str::Chars;

use serde_json::Value;


fn print_Type(v: &Value) {
    match v {
        Value::Bool(b)  => println!("Root is boolean"),
        Value::Number(n) => println!("Root is number"),
        Value::String(s) => println!("Root is string"),
        Value::Array(v) => println!("Root is vector"),
        Value::Object(m) => println!("Root is complex object"),
        Value::Null => println!("null"),
    }
}


fn traverse_and_sum(v: &Value, red: bool) -> i64 {
    match v {
        Value::Null => 0,
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap()
            } else {
                0
            }
        },
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Array(a) => {
            let mut sum: i64 = 0;
            for i in a.iter() {
                sum += traverse_and_sum(i, red);
            }
            sum
        },
        Value::Object(o) => {
            let mut sum: i64 = 0;
            if red && o.values().any(|x| if let Value::String(s) = x { s.eq("red") } else { false }) {

            } else {
                for i in o.values() {
                    sum += traverse_and_sum(i, red);
                }
            } 
            sum
        },
    }

}


fn read_input<P>(filename: P) -> u32
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let data: Value = serde_json::from_reader(buf).unwrap();

    println!("{}", traverse_and_sum(&data, false));
    println!("{}", traverse_and_sum(&data, true));
    0   
}

fn main() {
    let sum = read_input("data/12/input.txt");
   
}
