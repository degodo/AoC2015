//! --- Day 16: Aunt Sue ---
//! 
//! Your Aunt Sue has given you a wonderful gift, and you'd like to send her
//! a thank you card. However, there's a small problem: she signed it "From,
//! Aunt Sue".
//! 
//! You have 500 Aunts named "Sue".
//! 
//! So, to avoid sending the card to the wrong person, you need to figure out
//! which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave
//! you the gift. You open the present and, as luck would have it, good ol'
//! Aunt Sue got you a My First Crime Scene Analysis Machine! Just what you
//! wanted. Or needed, as the case may be.
//! 
//! The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a
//! few specific compounds in a given sample, as well as how many distinct
//! kinds of those compounds there are. According to the instructions, these
//! are what the MFCSAM can detect:
//! * children, by human DNA age analysis.
//! * cats (It doesn't differentiate individual breeds.)
//! * Several seemingly random breeds of dog: samoyeds, pomeranians, akitas,
//!   and vizslas.
//! * goldfish (No other kinds of fish.)
//! * trees (All in one group.)
//! * cars (Presumably by exhaust or gasoline or something.)
//! * perfumes (Which is handy, since many of your Aunts Sue wear a few kinds.)
//! 
//! In fact, many of your Aunts Sue have many of these. You put the wrapping
//! from the gift into the MFCSAM. It beeps inquisitively at you a few times
//! and then prints out a message on ticker tape:
//! 
//! children: 3
//! cats: 7
//! samoyeds: 2
//! pomeranians: 3
//! akitas: 0
//! vizslas: 0
//! goldfish: 5
//! trees: 3
//! cars: 2
//! perfumes: 1
//! 
//! You make a list of the things you can remember about each Aunt Sue. Things
//! missing from your list aren't zero - you simply don't remember the value.
//! 
//! What is the number of the Sue that got you the gift?
//! 
//! Answer: 373
//! 
//! --- Part Two ---
//! 
//! As you're about to send the thank you note, something in the MFCSAM's
//! instructions catches your eye. Apparently, it has an outdated
//! retroencabulator, and so the output from the machine isn't exact values -
//! some of them indicate ranges.
//! 
//! In particular, the cats and trees readings indicates that there are
//! greater than that many (due to the unpredictable nuclear decay of cat
//! dander and tree pollen), while the pomeranians and goldfish readings
//! indicate that there are fewer than that many (due to the modial
//! interaction of magnetoreluctance).
//! 
//! What is the number of the real Aunt Sue?
//! 
//! Answer: 260
//! 



use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default)]
struct Indicator {
    index: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Indicator {
    fn new(idx: u32) -> Indicator {
        let mut a = Indicator::default();
        a.index = idx;
        a
    }
    
    fn set_children(&mut self, v: u32) {
        self.children = Some(v);
    }
    
    fn set_cats(&mut self, v: u32) {
        self.cats = Some(v);
    }
    
    fn set_samoyeds(&mut self, v: u32) {
        self.samoyeds = Some(v);
    }
    
    fn set_pomeranians(&mut self, v: u32) {
        self.pomeranians = Some(v);
    }
    
    fn set_akitas(&mut self, v: u32) {
        self.akitas = Some(v);
    }
    
    fn set_vizslas(&mut self, v: u32) {
        self.vizslas = Some(v);
    }
    
    fn set_goldfish(&mut self, v: u32) {
        self.goldfish = Some(v);
    }
    
    fn set_trees(&mut self, v: u32) {
        self.trees = Some(v);
    }
    
    fn set_cars(&mut self, v: u32) {
        self.cars = Some(v);
    }
    
    fn set_perfumes(&mut self, v: u32) {
        self.perfumes = Some(v);
    }

    fn matches(&self, other: &Self, exact: bool) -> bool {
        let mut result: bool = true;

        if self.children.is_some() && other.children.is_some() { 
            result &= Some(self.children) == Some(other.children);
        }
        if self.cats.is_some() && other.cats.is_some() {
            if exact {
                result &= Some(self.cats) == Some(other.cats);
            } else {
                result &= Some(self.cats) > Some(other.cats);
            }
        }
        if self.samoyeds.is_some() && other.samoyeds.is_some() {
            result &= Some(self.samoyeds) == Some(other.samoyeds);
        }
        if self.pomeranians.is_some() && other.pomeranians.is_some() {
            if exact {
                result &= Some(self.pomeranians) == Some(other.pomeranians);
            } else {
                result &= Some(self.pomeranians) < Some(other.pomeranians);
            }
        }
        if self.akitas.is_some() && other.akitas.is_some() {
            result &= Some(self.akitas) == Some(other.akitas);
        }
        if self.vizslas.is_some() && other.vizslas.is_some() {
            result &= Some(self.vizslas) == Some(other.vizslas)
        }
        if self.goldfish.is_some() && other.goldfish.is_some() {
            if exact {
                result &= Some(self.goldfish) == Some(other.goldfish)
            } else {
                result &= Some(self.goldfish) < Some(other.goldfish)
            }
        }
        if self.trees.is_some() && other.trees.is_some() {
            if exact {
                result &= Some(self.trees) == Some(other.trees)
            } else {
                result &= Some(self.trees) > Some(other.trees)
            }
        }
        if self.cars.is_some() && other.cars.is_some() {
            result &= Some(self.cars) == Some(other.cars)
        }
        if self.perfumes.is_some() && other.perfumes.is_some() {
            result &= Some(self.perfumes) == Some(other.perfumes)
        }
        result
    }
}



fn parse_input(s: &str) -> Indicator {
    let mut i = s.split_whitespace();
    let mut indicator = Indicator::new(i.nth(1).unwrap().trim_end_matches(':').parse().unwrap());
    while let Some(attribute) = i.next() {
        let val: u32 = i.next().unwrap().trim_end_matches(',').parse().unwrap();
        match attribute.trim_end_matches(':') {
            "children" => indicator.set_children(val),
            "cats" => indicator.set_cats(val),
            "samoyeds" => indicator.set_samoyeds(val),
            "pomeranians" =>indicator.set_pomeranians(val),
            "akitas" =>indicator.set_akitas(val),
            "vizslas" =>indicator.set_vizslas(val),
            "goldfish" =>indicator.set_goldfish(val),
            "trees" =>indicator.set_trees(val),
            "cars" =>indicator.set_cars(val),
            "perfumes" =>indicator.set_perfumes(val),
            _ => ()
        }
    } 
    indicator
}

fn read_input<P,R>(filename: P, f: fn(&str) -> R) -> Vec<R>
where P: AsRef<Path> {
    let i = File::open(filename).expect("File not found!");
    let r = BufReader::new(i);
    let y = r.lines().map(|a| f(a.unwrap().as_ref()));
    y.collect()
}

fn main() {
    let indicators = read_input("data/16/input.txt", parse_input);
    let mut pattern = Indicator::new(0);
    pattern.set_children(3);
    pattern.set_cats(7);
    pattern.set_samoyeds(2);
    pattern.set_pomeranians(3);
    pattern.set_akitas(0);
    pattern.set_vizslas(0);
    pattern.set_goldfish(5);
    pattern.set_trees(3);
    pattern.set_cars(2);
    pattern.set_perfumes(1);

    for i in indicators.iter() {
        if i.matches(&pattern, true) {
            println!("Part 1: Matches Aunt {}", i.index);
        }
        if i.matches(&pattern, false) {
            println!("Part 2: Matches Aunt {}", i.index);
        }
    }
}

