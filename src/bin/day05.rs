use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

const VOWEL: &str = "aeiou";

fn part_1() {
    let file = File::open("data/05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut good_counter: u32 = 0;

    while let Some(Ok(line)) = (&mut lines).next() {
        let mut vowels_count: u32 = 0;
        let mut double: bool = false;
        let mut bad: bool = false;
        let mut last_char: char = ' ';

        for ch in line.chars() {
            if VOWEL.contains(ch) {
                vowels_count += 1;
            }
            double |= ch == last_char;
            bad |= (last_char == 'a' && ch == 'b') ||
                    (last_char == 'c' && ch == 'd') ||
                    (last_char == 'p' && ch == 'q') ||
                    (last_char == 'x' && ch == 'y');
            last_char = ch;
        }
        if vowels_count > 2 && double && !bad {
            good_counter += 1;
        }
    }

    println!("Good: {}", good_counter);
}


fn part_2() {
    let file: File = File::open("data/05/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines: Lines<BufReader<File>> = reader.lines();
    let mut good_count: u32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut pairs = HashMap::new();
        let mut split_double: bool = false;
        let mut double_pair: bool = false;

        // Positon, an dem das Paar erstmalig auftritt
        pairs.insert(&line[0..2], 0);

        for idx in 1..(line.len()-1) {
            let key = &line[idx..idx+2];
            if let Some(pos) = pairs.get_mut(key) {
                if  *pos + 2 <= idx {
                    double_pair |= true;
                }
            } else {
                pairs.insert(key, idx);
            }
            split_double |= &line[idx-1..idx] == &line[idx+1..idx+2];// && &line[idx..idx+1] != &line[idx+1..idx+2];

        }
        if double_pair && split_double {
            good_count += 1;
            //println!("{} -> {}, {}", &line, double_pair, split_double);
        };
        println!("{} -> {:?}, {}, {}", &line, pairs, double_pair, split_double);
    }
    println!("Good: {}", good_count);
}




fn main() {
    //part_1();
    part_2();
}