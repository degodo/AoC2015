//! --- Day 19: Medicine for Rudolph ---
//! 
//! Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very
//! brightly, and he needs medicine.
//! 
//! Red-Nosed Reindeer biology isn't similar to regular reindeer biology;
//! Rudolph is going to need custom-made medicine. Unfortunately, Red-Nosed 
//! Reindeer chemistry isn't similar to regular reindeer chemistry, either.
//! 
//! The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission
//! plant, capable of constructing any Red-Nosed Reindeer molecule you need.
//! It works by starting with some input molecule and then doing a series of
//! replacements, one per step, until it has the right molecule.
//! 
//! However, the machine has to be calibrated before it can be used.
//! Calibration involves determining the number of molecules that can be
//! generated in one step from a given starting point.
//! 
//! For example, imagine a simpler machine that supports only the following
//! replacements:
//! 
//!     H => HO
//!     H => OH
//!     O => HH
//! 
//! Given the replacements above and starting with HOH, the following
//! molecules could be generated:
//! 
//!     HOOH (via H => HO on the first H).
//!     HOHO (via H => HO on the second H).
//!     OHOH (via H => OH on the first H).
//!     HOOH (via H => OH on the second H).
//!     HHHH (via O => HH).
//! 
//! So, in the example above, there are 4 distinct molecules (not five,
//! because HOOH appears twice) after one replacement from HOH. Santa's
//! favorite molecule, HOHOHO, can become 7 distinct molecules (over nine
//! replacements: six from H, and three from O).
//! 
//! The machine replaces without regard for the surrounding characters. For
//! example, given the string H2O, the transition H => OO would result in
//! OO2O.
//! 
//! Your puzzle input describes all of the possible replacements and, at the
//! bottom, the medicine molecule for which you need to calibrate the machine.
//! How many distinct molecules can be created after all the different ways
//! you can do one replacement on the medicine molecule?
//! 
//! Answer: 518
//! 
//! --- Part Two ---
//! 
//! Now that the machine is calibrated, you're ready to begin molecule
//! fabrication.
//! 
//! Molecule fabrication always begins with just a single electron, e, and
//! applying replacements one at a time, just like the ones during calibration.
//! 
//! For example, suppose you have the following replacements:
//! 
//!    e => H
//!    e => O
//!    H => HO
//!    H => OH
//!    O => HH
//! 
//! If you'd like to make HOH, you start with e, and then make the following
//! replacements:
//! 
//!    e => O to get 
//!    O => HH to get HH
//!    H => OH (on the second H) to get HOH
//! 
//! So, you could make HOH after 3 steps. Santa's favorite molecule, HOHOHO
//! can be made in 6 steps.
//! 
//! How long will it take to make the medicine? Given the available replacements
//! and the medicine molecule in your puzzle input, what is the fewest number
//! of steps to go from e to the medicine molecule?
//! 
//! Answer: 200

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;


#[derive(Debug)]
struct Mutator {
    foreward: HashMap<String, Vec<String>>,
    backward: HashMap<String, String>,
}

impl Mutator {
    pub fn new() -> Mutator {
        Mutator { foreward: HashMap::new(), backward: HashMap::new() }
    }

    pub fn add(&mut self, raw: &str, cooked: &str) {
        self.foreward.entry(raw.to_owned()).or_insert(Vec::new()).push(cooked.to_owned());
        self.backward.insert(cooked.to_owned(), raw.to_owned());
    }

    pub fn recipie_single_mutate(&self, recipie: &str) -> HashSet<String> {
        let mut data: Vec<String> = Vec::new();
        for pat in self.foreward.keys().sorted() {
            let Some(repls) = self.foreward.get(pat) else { todo!() };
            for repl in repls {
                Self::single_mutate(recipie, pat, repl, &mut data);
            } 
        }
        return HashSet::from_iter(data);
    }

    pub fn reverse_recipie_len(&self, recipie: &str) -> u32 {
        Self::find_root(recipie, &self.backward)
    } 

    fn find_root(recipie: &str, course: &HashMap<String, String>) -> u32 {
        for (pattern, replace) in course {
            let mut data: Vec<String> = Vec::new();
            Self::single_mutate(recipie, pattern, replace, &mut data);
            for i in data.iter().sorted() {
                if i == "e" {
                    return 1;
                } else if i.len() < recipie.len() {
                    let a = Self::find_root(i, course);
                    if a > 0 {
                        return a + 1;
                    }
                } else if i.len() == recipie.len() && *i != *recipie {
                    let a = Self::find_root(i, course);
                    if a > 0 {
                        return a + 1;
                    }
                }
            }
        }
        0_u32
    }

    pub fn reverse_recipie_len_2(&self, recipie: &str) -> u32 {
        let mut done: bool = false;
        let mut count: u32 = 0;
        let mut from_data: HashSet<String> = HashSet::new();
        let mut to_data: HashSet<String> = HashSet::new();
        from_data.insert(recipie.to_owned());
        while !done {
            let mut temp_data: Vec<String> = Vec::new();
            for raw in from_data.iter() {    
                for (pattern, replace) in self.backward.iter() {
                    // println!("{} -> {}", &pattern, &replace);
                    Self::single_mutate(&raw, pattern, replace, &mut temp_data);
                    // println!("new len: {}", temp_data.len());               
                }
                for i in &temp_data {
                    if i.len() < raw.len() {
                        to_data.insert(i.clone());
                    }
                    if i == "e" || count == 2 {
                        done = true;
                    }  
                }
                println!("delta {} {}", to_data.len(), temp_data.len() );
                temp_data.clear();
            }
            
            from_data.clone_from(&to_data);
            println!("Tansformationen: {}", from_data.len());
            to_data.clear();
            count += 1;
        }
        count
    }

    // Erzeuge alle Strings, bei denen das Muster jeweils einfach ersetzt wird.
    fn single_mutate(raw: &str, pattern: &str, replace: &str, data: &mut Vec<String>) {
        let mut result;
        for (idx, _) in raw.match_indices(pattern) {
            result = raw[0..idx].to_owned();
            result.push_str(& raw[idx..].replacen(pattern, replace, 1));
            data.push(result);
        }
    }
    
    fn single_mutate_overlapping(raw: &str, pattern: &str, replace: &str, data: &mut Vec<String>) {
        let mut result;
        let mut idx: usize = 0;
        while idx + pattern.len() < raw.len() {
            if let Some(i) = raw[idx..].find(pattern) {
                result = raw[0..i+idx].to_owned();
                result.push_str(& raw[i+idx..].replacen(pattern, replace, 1));
                data.push(result);
                idx += i + 1;
            } else {
                idx = raw.len();
            }
        }
    }

}


fn read_data<P>(filename: P) -> (Mutator, String)
where P: AsRef<Path> {
    let f = File::open(filename).expect("File not found!");
    let mut r = BufReader::new(f).lines().map(|s| s.unwrap());
    let mut mutator: Mutator = Mutator::new();
    for i in r.by_ref().take_while(|l| l.len() > 0) {
        // println!("{}", &i);
        let mut a = i.split("=>");
        mutator.add(a.next().unwrap().trim(), a.next().unwrap().trim());
    }
    let recipie: String = r.next().unwrap();
    (mutator, recipie)
}


fn main() {
    let (mutator, recipie) =  read_data("data/19/input.txt");
    println!("Result Part 1: {}", mutator.recipie_single_mutate(&recipie).len());
    println!("Result Part 2: {}", mutator.reverse_recipie_len(&recipie));
}