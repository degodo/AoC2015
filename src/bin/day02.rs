use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn main() {
    let mut file = File::open("data/02/input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut sum: u32 = 0;

    // Part 1 - Amount of wrapping paper
    for line in reader.lines() {
        /*
        let dims = line.unwrap().split("x").collect::<Vec<&str>>();
        let length = dims[0].parse::<u32>().unwrap();
        let width = dims[1].parse::<u32>().unwrap();
        let height = dims[2].parse::<u32>().unwrap();
        */
        let dims = line.unwrap().split('x').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        sum += 2 * (dims[0] * dims[1] + dims[0] * dims[2] + dims[1] * dims[2]) 
                    + min(min(dims[0]*dims[1], dims[0]*dims[2]), min(dims[1]*dims[2], dims[1]*dims[2]));
    }
    println!("Part 1: {}", sum);
 
    // Part 2 - Length of ribbon
    file.rewind().unwrap();
    let reader = BufReader::new(&file);
    let mut len: u32 = 0;
    for line in reader.lines() {
        let mut dims = line.unwrap().split('x').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        dims.sort();
        len += dims[0] * dims[1] * dims[2] + 2 * (dims[0] + dims[1]);
    }
    println!("Part 2: {}", len);
}