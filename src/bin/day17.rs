//!
//! --- Day 17: No Such Thing as Too Much ---
//! 
//! The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to move it into smaller containers. You take an inventory of the capacities of the available containers.
//! 
//! For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there are four ways to do it:
//! 
//! 15 and 10
//! 20 and 5 (the first 5)
//! 20 and 5 (the second 5)
//! 15, 5, and 5
//! Filling all containers entirely, how many different combinations of containers can exactly fit all 150 liters of eggnog?
//! 
//! Answer: 1304
//! 
//! --- Part Two ---
//! 
//! While playing with all the containers in the kitchen, another load of
//! eggnog arrives! The shipping and receiving department is requesting as
//! many containers as you can spare.
//! 
//! Find the minimum number of containers that can exactly fit all 150 liters
//! of eggnog. How many different ways can you fill that number of containers
//! and still hold exactly 150 litres?
//! 
//! In the example above, the minimum number of containers was two. There
//! were three ways to use that many containers, and so the answer there
//! would be 3.
//! 
//! Answer: 18
//! 



use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;


fn parse_line(l: &str) -> u32 {
    l.parse().unwrap()
}

fn read_input<P,R>(filename: P, f: fn(&str) -> R) -> Vec<R> 
where P: AsRef<Path> {
    let i = File::open(filename).expect("File not found.");
    let r = BufReader::new(i);
    let y = r.lines().map(|s| f(s.unwrap().as_ref()));
    y.collect()
}

fn main() {
    let mut data = read_input("data/17/input.txt", parse_line);
    data.sort();
    let mut count_all: u32 = 0;
    let mut count_min: u32 = 0;
    for i in 1..=data.len() {
        println!("k-length combination {}", i);
        for j in data.iter().combinations(i) {
            let mut k: u32 = 0;
            for z in j.iter() {
                k += **z;
                if k > 150 {
                    break;
                }
            }
            if k == 150 {
                count_all += 1;
            }
        }
        if count_min == 0 {
            count_min = count_all;
        }
    }
    println!("Part 1 - Answer: {}",count_all);
    println!("Part 2 - Answer: {}",count_min);

}
