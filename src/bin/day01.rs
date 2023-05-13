 use std::{fs::File, io::Read};

fn main() {

    // Prepare the input
    let mut file = File::open("data/01/input.txt").unwrap();
    let mut buffer: String = String::new();
    let mut floor: i32 = 0;
    file.read_to_string(&mut buffer).unwrap();

    // Part 1: To which floor does the input lead?
    for ch in buffer.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("{}", floor);

    // Part 2: At what input data position is floor == -1; count starts a 1
    floor = 0;
    for (idx, ch) in buffer.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            println!("{}", idx + 1);
            break;
        }
    }
}
