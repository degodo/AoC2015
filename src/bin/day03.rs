use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn part_1() {
    let mut file = File::open("data/03/input.txt").unwrap();
    let mut buf = String::new();
    // pos is (X,Y) of santa clause
    let mut pos_cache: HashSet<(i32, i32)> = HashSet::new();
    let mut pos: (i32, i32) = (0, 0);
    
    file.read_to_string(&mut buf).unwrap();
    
    pos_cache.insert(pos);
    
    for ch in buf.chars() {
        match ch {
            '>' => {
                pos.0 += 1;
                pos_cache.insert(pos);
            },
            '<' => {
                pos.0 -= 1;
                pos_cache.insert(pos);
            },
            '^' => {
                pos.1 += 1;
                pos_cache.insert(pos);
            },
            'v' => {
                pos.1 -= 1;
                pos_cache.insert(pos);
            },
            _ => panic!("Unknown character")
        }
    }
    
    println!("Part 1: {}", pos_cache.len());
}

fn part_2() {
    let mut file = File::open("data/03/input.txt").unwrap();
    let mut buf = String::new();
    // pos is (X,Y) of santa clause
    let mut pos_cache: HashSet<(i32, i32)> = HashSet::new();
    let mut pos_s: (i32, i32) = (0, 0);
    let mut pos_r: (i32, i32) = (0, 0);
    let mut turn_santa: bool = true;
    
    file.read_to_string(&mut buf).unwrap();
    
    pos_cache.insert(pos_s);
    pos_cache.insert(pos_r);
    

    for ch in buf.chars() {
        let mut pos: (i32, i32);

        if turn_santa {
            pos = pos_s
        } else {
            pos = pos_r
        }
        
        match ch {
            '>' => {
                pos.0 += 1;
            },
            '<' => {
                pos.0 -= 1;
            },
            '^' => {
                pos.1 += 1;
            },
            'v' => {
                pos.1 -= 1;
            },
            _ => panic!("Unknown character")
        }
        pos_cache.insert(pos);
        if turn_santa {
            pos_s = pos;
        } else {
            pos_r = pos;
        }
        turn_santa = !turn_santa;
    }
    
    println!("Part 2: {}", pos_cache.len());
}

fn main () {
    part_1();
    part_2();
}