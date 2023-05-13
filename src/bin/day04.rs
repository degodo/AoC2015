//; --- Day 4: The Ideal Stocking Stuffer ---
//;
//; Santa needs help mining some AdventCoins (very similar to bitcoins) to use
//; as gifts for all the economically forward-thinking little girls and boys.
//;
//; To do this, he needs to find MD5 hashes which, in hexadecimal, start with
//; at least five zeroes. The input to the MD5 hash is some secret key (your
//; puzzle input, given below) followed by a number in decimal. To mine
//; AdventCoins, you must find Santa the lowest positive number (no leading
//; zeroes: 1, 2, 3, ...) that produces such a hash.
//;
//; For example:
//;
//; If your secret key is abcdef, the answer is 609043, because the MD5 hash
//; of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the
//; lowest such number to do so.
//; If your secret key is pqrstuv, the lowest number it combines with to make
//; an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of
//; pqrstuv1048970 looks like 000006136ef....

use std::fmt::Write;
use md5::{Digest, Md5};

const secret_key: &str = include_str!("../../data/04/input.txt");

fn part1(start: &str) -> u64 {
    let mut hash = Md5::new();
    for i in 0_u64.. {
        let s = format!("{}{}", secret_key, i);
        let mut t = String::new();
        hash.update(&s);
        let dig = hash.finalize_reset(); 
        let a: &[u8] = dig.as_ref();
        for &ch in a {
            write!(&mut t, "{:02X}", ch);
        }
        if t.starts_with(start) {
            return i;
        }
        t.truncate(0);
    }
    0_u64
}


fn main() {
    println!("{}", part1("00000"));
    println!("{}", part1("000000"));
}
