//! --- Day 20: Infinite Elves and Infinite Houses ---
//!
//! To keep the Elves busy, Santa has them deliver some presents by hand,
//! door-to-door. He sends them down a street with infinite houses numbered
//! sequentially: 1, 2, 3, 4, 5, and so on.
//! 
//! Each Elf is assigned a number, too, and delivers presents to houses based
//! on that number:
//! 
//!    The first Elf (number 1) delivers presents to
//!    every house: 1, 2, 3, 4, 5, ....
//!    The second Elf (number 2) delivers presents to
//!    every second house: 2, 4, 6, 8, 10, ....
//!    Elf number 3 delivers presents to
//!    every third house: 3, 6, 9, 12, 15, ....
//! 
//!    There are infinitely many Elves, numbered starting with 1. Each Elf
//!    delivers presents equal to ten times his or her number at each house.
//! 
//! So, the first nine houses on the street end up like this:
//! 
//!    House 1 got 10 presents.
//!    House 2 got 30 presents.
//!    House 3 got 40 presents.
//!    House 4 got 70 presents.
//!    House 5 got 60 presents.
//!    House 6 got 120 presents.
//!    House 7 got 80 presents.
//!    House 8 got 150 presents.
//!    House 9 got 130 presents.
//! 
//! The first house gets 10 presents: it is visited only by Elf 1, which
//! delivers 1 * 10 = 10 presents. The fourth house gets 70 presents, because
//! it is visited by Elves 1, 2, and 4, for a total of 10 + 20 + 40 = 70
//! presents.
//! 
//! What is the lowest house number of the house to get at least as many
//! presents as the number in your puzzle input?
//! 
//! Your puzzle input is 36000000.
//! 
//! Answer: 831600
//! 
//! --- Part Two ---
//! 
//! The Elves decide they don't want to visit an infinite number of houses.
//! Instead, each Elf will stop after delivering presents to 50 houses. To
//! make up for it, they decide to deliver presents equal to eleven times
//! their number at each house.
//! 
//! With these changes, what is the new lowest house number of the house to 
//! get at least as many presents as the number in your puzzle input?
//! 
//! Answer: 884520


use std::ops::{Mul, Div};

pub fn get_divisors(n: u32) -> Vec<u32> {
    
    let _0 = u32::from(0_u32);
    let _1 = u32::from(1_u32);
    let _2 = u32::from(2_u32);
    let mut _n = n;
    let mut v: Vec<u32> = Vec::new();
    
    let mut count_divisors_2: usize = 0;
    while _n & _1 == _0 {
        v.push(_2 << count_divisors_2);
        count_divisors_2 += 1;
        _n = _n >> 1;
    }
    
    let mut _x = u32::from(3_u32);
    let mut _n_sqrt = approximated_sqrt(_n);
    while _x < _n_sqrt {        
        let mut _pow_x = _x;
        let v_len = v.len();
        let mut x_is_a_divisors = false;

        let mut pow_x_is_a_divisors = _n % _x == _0;
        while pow_x_is_a_divisors == true {
            _n = _n.div(_x);
            v.push(_pow_x);
            push_new_divisors(&mut v, v_len, _pow_x);
            pow_x_is_a_divisors = _n % _x == _0;
            if pow_x_is_a_divisors == true {
                _pow_x = _pow_x.mul(_x);                
            }
            x_is_a_divisors = true;
        }
        _x = _x + _2;
        if x_is_a_divisors == true {
            _n_sqrt = approximated_sqrt(_n);
        }
    }
    
    if _n > _1 && _n != n {
        let v_len = v.len();
        v.push(_n);
        push_new_divisors(&mut v, v_len, _n);
    }

    if v.len() > 1 {
        v.sort();
        v.pop();
    }
    
    v
}

pub fn approximated_sqrt(n: u32) -> u32 {
    let _0 = u32::from(0_u32);
    let _1 = u32::from(1_u32);
    let mut num_bits = (std::mem::size_of::<u32>() << 3) - 1;
    while ((n >> num_bits) & _1) == _0 {
        num_bits -= 1;
    }
    _1 << ((num_bits >> 1) + 1)
}

fn push_new_divisors(v: &mut Vec<u32>, v_len: usize, _x: u32) {
    for i in 0..v_len {
        v.push(_x.mul(unsafe { *v.get_unchecked(i) }));
    }
}

fn main() {
    let mut found_1: bool = false;
    let mut found_2: bool = false;

    for i in 1.. {
        if !found_1 && (1 + i + get_divisors(i).iter().sum::<u32>() >= 3600000) {
            println!("Ergebnis 1: {}", i);
            found_1 = true;
        };
        let a = i / 50;
        if !found_2 && (11*(i + get_divisors(i).iter().filter(|x| x > &&a).sum::<u32>()) >= 36000000) {
            println!("Ergebnis 2: {}", i);
            found_2 = true;
        } 
        if found_1 && found_2 {
            break;
        }
    }



}