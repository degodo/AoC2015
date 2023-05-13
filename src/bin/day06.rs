//!
//! --- Day 6: Probably a Fire Hazard ---
//! 
//! Because your neighbors keep defeating you in the holiday house decorating
//! contest year after year, you've decided to deploy one million lights in a
//! 1000x1000 grid.
//! 
//! Furthermore, because you've been especially nice this year, Santa has
//! mailed you instructions on how to display the ideal lighting
//! configuration.
//! 
//! Lights in your grid are numbered from 0 to 999 in each direction; the
//! lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The
//! instructions include whether to turn on, turn off, or toggle various
//! inclusive ranges given as coordinate pairs. Each coordinate pair
//! represents opposite corners of a rectangle, inclusive; a coordinate pair
//! like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The
//! lights all start turned off.
//! 
//! To defeat your neighbors this year, all you have to do is set up your
//! lights by doing the instructions Santa sent you in order.
//! 
//! For example:
//! turn on 0,0 through 999,999        -> would turn on (or leave on) every
//!                                       light.
//! toggle 0,0 through 999,0           -> would toggle the first line of 1000
//!                                       lights, turning off the ones that
//!                                       were on, and turning on the ones
//!                                       that were off.
//! turn off 499,499 through 500,500   -> would turn off (or leave off) the
//!                                       middle four lights.
//! 
//! After following the instructions, how many lights are lit?
//!
//! 
//! --- Part Two ---
//! 
//! You just finish implementing your winning light pattern when you realize
//! you mistranslated Santa's message from Ancient Nordic Elvish.
//! 
//! The light grid you bought actually has individual brightness controls;
//! each light can have a brightness of zero or more. The lights all start at
//! zero.
//! 
//! The phrase turn on actually means that you should increase the brightness
//! of those lights by 1.
//! 
//! The phrase turn off actually means that you should decrease the brightness
//! of those lights by 1, to a minimum of zero.
//! 
//! The phrase toggle actually means that you should increase the brightness
//! of those lights by 2.
//! 
//! What is the total brightness of all lights combined after following
//! Santa's instructions?
//! 
//! For example:
//! turn on 0,0 through 0,0     -> would increase the total brightness by 1.
//! toggle 0,0 through 999,999  -> would increase the total brightness by 2000000.

use std::{fs::File, io::{BufReader, BufRead}};


#[derive(Debug, Clone, Copy)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
    Nop,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct Action {
    action_type: Instruction,
    start: Pos,
    end: Pos,
}

static NOP_ACTION: Action = Action { action_type: Instruction::Nop,
                                     start: Pos{ x: 0, y: 0 },
                                     end: Pos{ x:0, y:0 } };


fn parse_pos(s: &str) -> Pos {
    let mut i = s.split(',');
    Pos { x: i.next().unwrap().parse::<u32>().unwrap(),
          y: i.next().unwrap().parse::<u32>().unwrap()}
}


fn split_line(s: &str) -> Action {
    let mut i = s.trim().split_whitespace();
    match i.next().expect("Wrong line structure.") {
        "turn" => match i.next().expect("Wring inner line structure.") {
            "on"  => { let start = parse_pos(i.next().unwrap());
                       i.next();
                       let end = parse_pos(i.next().unwrap());
                       Action { action_type: Instruction::TurnOn, start, end }
            },
            "off" => { let start = parse_pos(i.next().unwrap());
                       i.next();
                       let end = parse_pos(i.next().unwrap());
                       Action { action_type: Instruction::TurnOff, start, end }
            },
            &_    => NOP_ACTION,
        },
        "toggle" => { let start = parse_pos(i.next().unwrap());
                      i.next();
                      let end = parse_pos(i.next().unwrap());
                      Action { action_type: Instruction::Toggle, start, end }
        },
        &_ =>       NOP_ACTION,
    }
}


fn read_lines<P>(filename: P) -> Vec<Action>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let x = buf.lines();
    let y = x.map(|l| split_line(l.unwrap().as_ref())); 
    y.collect()

}

fn main (){
    let mut grid = [[(false, 0u32); 1000]; 1000];
    let actions = read_lines("data/06/input.txt");

    for i in actions {
        let lenx = i.end.x - i.start.x + 1;
        let leny = i.end.y - i.start.y + 1;
        println!{"x: {}, y: {}, dx: {}, dy: {}, do: {:?}", i.start.x, i.start.y, lenx, leny, i.action_type};
        for dx in 0..lenx {
            for dy in 0..leny {
                let x = (i.start.x + dx) as usize;
                let y = (i.start.y + dy) as usize;
                match i.action_type {
                    Instruction::TurnOn  => {
                        grid[x][y].0 = true;
                        grid[x][y].1 += 1;
                    },
                    Instruction::TurnOff => {
                        grid[x][y].0 = false;
                        if grid[x][y].1 > 0 {
                            grid[x][y].1 -= 1;
                        }
                    },
                    Instruction::Toggle  => {
                        grid[x][y].0 = !grid[x][y].0;
                        grid[x][y].1 += 2;
                    },
                    Instruction::Nop     => (),
                }
            }
        }
    }
    let mut count = 0u32;
    let mut brightnes = 0u32;
    for x in 0..1000 {
        for y in 0..1000 {
            brightnes += grid[x as usize][y as usize].1;
            if grid[x as usize][y as usize].0 == true {
                count += 1;
            }
        }
    } println!("count: {}, brightnes: {} ", count, brightnes);
}

