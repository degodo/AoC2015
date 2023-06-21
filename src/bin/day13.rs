//!
//! --- Day 13: Knights of the Dinner Table ---
//! 
//! In years past, the holiday feast with your family hasn't gone so well. Not
//! everyone gets along! This year, you resolve, will be different. You're
//! going to find the optimal seating arrangement and avoid all those awkward
//! conversations.
//! 
//! You start by writing up a list of everyone invited and the amount their
//! happiness would increase or decrease if they were to find themselves
//! sitting next to each other person. You have a circular table that will be
//! just big enough to fit everyone comfortably, and so each person will have
//! exactly two neighbors.
//! 
//! For example, suppose you have only four attendees planned, and you
//! calculate their potential happiness as follows:
//! 
//! Alice would gain 54 happiness units by sitting next to Bob.
//! Alice would lose 79 happiness units by sitting next to Carol.
//! Alice would lose 2 happiness units by sitting next to David.
//! Bob would gain 83 happiness units by sitting next to Alice.
//! Bob would lose 7 happiness units by sitting next to Carol.
//! Bob would lose 63 happiness units by sitting next to David.
//! Carol would lose 62 happiness units by sitting next to Alice.
//! Carol would gain 60 happiness units by sitting next to Bob.
//! Carol would gain 55 happiness units by sitting next to David.
//! David would gain 46 happiness units by sitting next to Alice.
//! David would lose 7 happiness units by sitting next to Bob.
//! David would gain 41 happiness units by sitting next to Carol.
//! 
//! Then, if you seat Alice next to David, Alice would lose 2 happiness units
//! (because David talks so much), but David would gain 46 happiness units
//! (because Alice is such a good listener), for a total change of 44.
//! 
//! If you continue around the table, you could then seat Bob next to Alice
//! (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob
//! (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41).
//! The arrangement looks like this:
//! 
//!          +41    +46
//!   +55      David     -2
//!   Carol              Alice
//!   +60       Bob      +54
//!          -7    +83
//! 
//! After trying every other seating arrangement in this hypothetical
//! scenario, you find that this one is the most optimal, with a total change
//! in happiness of 330.
//! 
//! What is the total change in happiness for the optimal seating arrangement
//! of the actual guest list?
//! 
//! Answer: 709
//! 
//! --- Part Two ---
//! In all the commotion, you realize that you forgot to seat yourself. At
//! this point, you're pretty apathetic toward the whole thing, and your
//! happiness wouldn't really go up or down regardless of who you sit next to.
//! You assume everyone else would be just as ambivalent about sitting next to
//! you, too.
//! 
//! So, add yourself to the list, and give all happiness relationships that
//! involve you a score of 0.
//! 
//! What is the total change in happiness for the optimal seating arrangement
//! that actually includes yourself?
//! 
//! Answer: 668
//! 


use std::collections::HashMap;
use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use itertools::Itertools;

fn calc_happyness(order: &Vec<&String>, gauge: &HashMap<String,i32>) -> i32 {
    let mut it = order.iter();
    let mut val: i32 = 0;
    let mut lh = &order[order.len()-1];
    while let Some(rh) = it.next() {
        val += gauge.get(&format!("{},{}", lh, rh)).unwrap();
        val += gauge.get(&format!("{},{}", rh, lh)).unwrap();
        lh = rh; 
    }
    val
}


fn splitLine(s: &str) -> (String, String, i32) {
    // letztes Zeichen der Zeile (".") ignorieren
    let mut t = s[0..s.len()-1].split_whitespace();
    let a = t.next().unwrap();
    let s = t.nth(1).unwrap();
    let Ok(mut i) = t.next().unwrap().parse::<i32>() else { panic!("") };
    let b = t.last().unwrap();
    if s.eq("lose") {
        i = (-1) * i;
    }
    (a.to_owned(), b.to_owned(), i)
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
    let mut gauge: HashMap<String,i32> = HashMap::new();
    let mut guests: Vec<String> = Vec::new();
    for i in read_input("data/13/input.txt", splitLine).iter() {
        gauge.insert(format!("{},{}", &i.0, &i.1), i.2);
        if !guests.contains(&i.0) {
            guests.push(i.0.clone());
        }
    }
    // Würden n Personen in einer Reihne sitzen, dass wäre n! Sitzprdnungen
    // mögliche. Da die n Personen in einem runden Tische mit nicht unter-
    // scheidbaren Plätzen sitzen, sind es nur (n-1)! mögliche Sitzordnungen. 
    // Zur Brechnung aller möglichen Ordnungen wird dieselbe Person bei allen
    // Ordnungen an die Position 1 gestellt und für die anderen (n-1) die
    // Permutition der Orndung (n-1) berechnet.
    // Permutationen von A,B,C sind: ABC,ACB,BAC,BCA,CAB,CBA. An einem Runden
    // Tisch sind ABC, BCA und CAB sowie ACB, BAC und CBA identisch. 
    { 
        let mut seq  = guests[1..].iter().permutations(guests.len()-1);
        let mut max_happyness: i32 = 0;
        while let Some(mut order) = seq.next() {
            order.push(&guests[0]);
            let a = calc_happyness(&order, &gauge);
            if a > max_happyness {
                max_happyness = a;
            }
        }
        println!("Part 1 - Max. happyness: {}", max_happyness);
    }
    
    {
        let mut it = guests.iter();
        while let Some(a) = it.next() {
            gauge.insert(format!("{},{}", a, "myself"), 0);
            gauge.insert(format!("{},{}", "myself", a), 0);
        }
        guests.push("myself".to_owned());
        
        let mut seq  = guests[1..].iter().permutations(guests.len()-1);
        let mut max_happyness: i32 = 0;
        while let Some(mut order) = seq.next() {
            order.push(&guests[0]);
            let a = calc_happyness(&order, &gauge);
            if a > max_happyness {
                max_happyness = a;
            }
        }
        println!("Part 2 - Max. happyness: {}", max_happyness);

    }



}

