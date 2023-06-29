//!--- Day 18: Like a GIF For Your Yard ---
//! 
//! After the million lights incident, the fire code has gotten stricter: now,
//! at most ten thousand lights are allowed. You arrange them in a 100x100
//! grid.
//! 
//! Never one to let you down, Santa again mails you instructions on the ideal
//! lighting configuration. With so few lights, he says, you'll have to resort
//! to animation.
//! 
//! Start by setting your lights to the included initial configuration (your
//! puzzle input). A # means "on", and a . means "off".
//! 
//! Then, animate your grid in steps, where each step decides the next
//! configuration based on the current one. Each light's next state (either
//! on or off) depends on its current state and the current states of the
//! eight lights adjacent to it (including diagonals). Lights on the edge of
//! the grid might have fewer than eight neighbors; the missing ones always
//! count as "off".
//! 
//! For example, in a simplified 6x6 grid, the light marked A has the
//! neighbors numbered 1 through 8, and the light marked B, which is on an
//! edge, only has the neighbors marked 1 through 5:
//!
//!     1B5...
//!     234...
//!     ......
//!     ..123.
//!     ..8A4.
//!     ..765.
//! 
//! The state a light should have next is based on its current state (on or
//! off) plus the number of neighbors that are on:
//! 
//! * A light which is on stays on when 2 or 3 neighbors are on, and turns off
//!   otherwise.
//! * A light which is off turns on if exactly 3 neighbors are on, and stays
//!   off otherwise.
//! * All of the lights update simultaneously; they all consider the same
//!   current state before moving to the next.
//! 
//! Here's a few steps from an example configuration of another 6x6 grid:
//! 
//! Initial state:
//! 
//!     .#.#.#
//!     ...##.
//!     #....#
//!     ..#...
//!     #.#..#
//!     ####..
//!
//! After 1 step:
//! 
//!     ..##..   
//!     ..##.#   
//!     ...##.   
//!     ......   
//!     #.....   
//!     #.##..   
//! 
//! After 2 steps:
//! 
//!     ..###.   
//!     ......   
//!     ..###.   
//!     ......   
//!     .#....   
//!     .#....   
//! 
//! After 3 steps:
//! 
//!     ...#..   
//!     ......   
//!     ...#..   
//!     ..##..   
//!     ......   
//!     ......   
//! 
//! After 4 steps:
//! 
//!     ......   
//!     ......   
//!     ..##..   
//!     ..##..   
//!     ......   
//!     ......
//! 
//! After 4 steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, how many
//! lights are on after 100 steps?
//!
//! Answer: 821
//! 
//! --- Part Two ---
//! 
//! You flip the instructions over; Santa goes on to point out that this is
//! all just an implementation of Conway's Game of Life. At least, it was,
//! until you notice that something's wrong with the grid of lights you
//! bought: four lights, one in each corner, are stuck on and can't be turned
//! off. The example above will actually run like this:
//! 
//! Initial state:
//!
//!    ##.#.#
//!    ...##.
//!    #....#
//!    ..#...
//!    #.#..#
//!    ####.#
//! 
//! After 1 step:
//!
//!    #.##.#
//!    ####.#
//!    ...##.
//!    ......
//!    #...#.
//!    #.####
//! 
//! After 2 steps:
//!
//!    #..#.#
//!    #....#
//!    .#.##.
//!    ...##.
//!    .#..##
//!    ##.###
//! 
//! After 3 steps:
//!
//!    #...##
//!    ####.#
//!    ..##.#
//!    ......
//!    ##....
//!    ####.#
//! 
//! After 4 steps:
//!
//!    #.####
//!    #....#
//!    ...#..
//!    .##...
//!    #.....
//!    #.#..#
//! 
//! After 5 steps:
//!
//!    ##.###
//!    .##..#
//!    .##...
//!    .##...
//!    #.#...
//!    ##...#
//! 
//! After 5 steps, this example now has 17 lights on.
//! 
//! In your grid of 100x100 lights, given your initial configuration, but
//! with the four corners always in the on state, how many lights are on
//! after 100 steps?
//! 
//! Antwort: 886


use std::fs::File;
use std::io::{Read, BufRead, BufReader, Error};
use std::path::Path;

use itertools::Itertools;


#[derive(Debug)]
struct Grid {
    current: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>,
    rows: u32,
    columns: u32,
}


impl Grid {
    fn new(i: impl Iterator<Item = String>) -> Grid {
        let mut cols: u32 = 0;
        let mut rows: u32 = 0;
        let mut current: Vec<Vec<bool>> = Vec::new();
        for ref line in i {
            let mut row: Vec<bool> = Vec::new();
            cols = line.len() as u32;
            rows +=1;
            for char in line.chars() {
                if char == '#' {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            current.push(row);
        }
        Grid{ current: current.clone(), next: current , rows, columns: cols }
    }
    
    // Die Nachbarfelder haben die Positionen
    //  0  1  2
    //  7  *  3   
    //  6  5  4
    fn assess_neighbors(&self, row: u32, col: u32) -> (u8, u8) {
        let mut num_on: u8 = 0;
        let mut num_off: u8 = 0;

        #[derive(Debug)]
        struct NB {
            calc: bool,
            delta_row: i8,
            delta_col: i8,
        }
        let mut neighbors = [
            NB{calc: true, delta_row: -1, delta_col: -1},
            NB{calc: true, delta_row: -1, delta_col: 0},
            NB{calc: true, delta_row: -1, delta_col: 1},
            NB{calc: true, delta_row: 0, delta_col: 1},
            NB{calc: true, delta_row: 1, delta_col: 1},
            NB{calc: true, delta_row: 1, delta_col: 0},
            NB{calc: true, delta_row: 1, delta_col: -1},
            NB{calc: true, delta_row: 0, delta_col: -1},
        ];

        if col == 0 {
            // Pos.0, 6 und 7 liegen außerhalb
            neighbors[0]. calc = false;
            neighbors[6]. calc = false;
            neighbors[7]. calc = false;
            if row == 0 {
                // Pos.1 und 2 liegen außerhalt
                neighbors[1]. calc = false;
                neighbors[2]. calc = false;
            }
            if row == self.rows - 1 {
                // Pos.4 und 5 liegen außerhalb
                neighbors[4]. calc = false;
                neighbors[5]. calc = false;
            }
        } else if col == self.columns - 1 {
            // Pos.2, 3 und 4 liegen au0erhalb
            neighbors[2]. calc = false;
            neighbors[3]. calc = false;
            neighbors[4]. calc = false;
            if row == 0 {
                // Pos.0 und 1 liegen au0erhalb
                neighbors[0]. calc = false;
                neighbors[1]. calc = false;
            } 
            if row  == self.rows - 1 {
                // Pos.5 und 6 liegen außerhalb
                neighbors[5]. calc = false;
                neighbors[6]. calc = false;
            } 
        } else {
            if row == 0 {
                // Pos.0, 1 und 2 liegen au0erhalb
                neighbors[0]. calc = false;
                neighbors[1]. calc = false;
                neighbors[2]. calc = false;
            } else if row  == self.rows - 1 {
                // Pos.4, 5 und 6 liegen außerhalb
                neighbors[4]. calc = false;
                neighbors[5]. calc = false;
                neighbors[6]. calc = false;
            } 
        }
        for i in 0..8 {
            if neighbors[i].calc {
                if self.current[(row as i32 + neighbors[i].delta_row as i32) as usize][(col as i32 + neighbors[i].delta_col as i32) as usize] {
                    num_on += 1;
                } else {
                    num_off += 1;
                }
            } else {
                num_off += 1;
            }
        }

        (num_on, num_off)
    }
    
    fn toggle_lights(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.columns {
                let (num_on, num_off) = self.assess_neighbors(r, c);
                if self.current[r as usize][c as usize] {
                    if num_on == 2 || num_on == 3 {
                        self.next[r as usize][c as usize] = true;
                    } else {
                        self.next[r as usize][c as usize] = false;
                    }
                } else if !self.current[r as usize][c as usize] {
                    if num_on == 3 {
                        self.next[r as usize][c as usize] = true;
                    } else {
                        self.next[r as usize][c as usize] = false;
                    }
                }
            }
        }
    }

    fn flip_grit(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);      
    }

    fn count_on(& self) -> u32 {
        self.current.iter().flatten().map(|b| if *b { 1 } else { 0 }).sum()
    }

    fn corners_stick_on(&mut self) {
        self.current[0][0] = true;
        self.current[0][(self.columns - 1) as usize] = true;
        self.current[(self.rows - 1) as usize][(self.columns - 1) as usize] = true;
        self.current[(self.rows - 1) as usize][0] = true;
    }
    
}


fn read_data<P>(filename: P ) -> Grid
where P: AsRef<Path> {
    let mut i = File::open(filename).expect("File not found.");
    let mut y = BufReader::new(i).lines().map(|l| l.unwrap());
    Grid::new(y)
}


pub fn main() {
    let mut data: Grid = read_data("data/18/input.txt");
    println!("{}", data.count_on());
    data.corners_stick_on(); // remove for part 1
    println!("{}", data.count_on());
    for i in 0..100 {
        data.toggle_lights();
        data.flip_grit();
        data.corners_stick_on(); // remove for part 2
    }
    println!("{}", data.count_on());
}