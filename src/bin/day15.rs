//!
//! --- Day 15: Science for Hungry People ---
//! 
//! Today, you set out on the task of perfecting your milk-dunking cookie
//! recipe. All you have to do is find the right balance of ingredients.
//! 
//! Your recipe leaves room for exactly 100 teaspoons of ingredients. You
//! make a list of the remaining ingredients you could use to finish the
//! recipe (your puzzle input) and their properties per teaspoon:
//! 
//! capacity (how well it helps the cookie absorb milk)
//! durability (how well it keeps the cookie intact when full of milk)
//! flavor (how tasty it makes the cookie)
//! texture (how it improves the feel of the cookie
//! calories (how many calories it adds to the cookie)
//! 
//! You can only measure ingredients in whole-teaspoon amounts accurately,
//! and you have to be accurate so you can reproduce your results in the
//! future. The total score of a cookie can be found by adding up each of
//! the properties (negative totals become 0) and then multiplying together 
//! everything except calories.
//! 
//! For instance, suppose you have these two ingredients:
//! 
//! Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
//! Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
//! 
//! Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of
//! cinnamon (because the amounts of each ingredient must add up to 100)
//! would result in a cookie with the following properties:
//! 
//! A capacity of 44*-1 + 56*2 = 68
//! A durability of 44*-2 + 56*3 = 80
//! A flavor of 44*6 + 56*-2 = 152
//! A texture of 44*3 + 56*-1 = 76
//! 
//! Multiplying these together (68 * 80 * 152 * 76, ignoring calories for
//! now) results in a total score of 62842880, which happens to be the best
//! score possible given these ingredients. If any properties had produced
//! a negative total, it would have instead become zero, causing the whole
//! score to multiply to zero.
//! 
//! Given the ingredients in your kitchen and their properties, what is the
//! total score of the highest-scoring cookie you can make.
//! 
//! Answer: 18965440
//! 
//! --- Part Two ---
//! 
//! Your cookie recipe becomes wildly popular! Someone asks if you can make
//! another recipe that has exactly 500 calories per cookie (so they can use
//! it as a meal replacement). Keep the rest of your award-winning process
//! the same (100 teaspoons, same ingredients, same scoring system).
//! 
//! For example, given the ingredients above, if you had instead selected 40
//! teaspoons of butterscotch and 60 teaspoons of cinnamon (which still adds
//! to 100), the total calorie count would be 40*8 + 60*3 = 500. The total
//! score would go down, though: only 57600000, the best you can do in such
//! trying circumstances.
//! 
//! Given the ingredients in your kitchen and their properties, what is the
//! total score of the highest-scoring cookie you can make with a calorie
//! total of 500?
//! 
//! Answer: 15862900
//! 


use std::fs::File; 
use std::io::{BufRead, BufReader};

#[derive(Debug,Default)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Ingredient {
        Ingredient {name: name.to_string(), capacity, durability, flavor, texture, calories }
    }
}

fn rate_recipe(ingredients: &Vec<Ingredient>, mixture: &Vec<i32>) -> (i32,i32) {
    let mut capacity: i32 = 0;
    let mut durability: i32 = 0;
    let mut flavor: i32 = 0;
    let mut texture: i32 = 0;
    let mut calories: i32 = 0;

    for (i,q) in ingredients.iter().zip(mixture) {
        capacity += i.capacity * q;
        durability += i.durability * q;
        flavor += i.flavor * q;
        texture += i.texture * q;
        calories += i.calories * q;
    }
    ((if capacity > 0 { capacity } else { 0 })
    * (if durability > 0 { durability } else { 0 }) 
    * (if flavor > 0 { flavor } else { 0 }) 
    * (if texture > 0 { texture } else { 0 }),
    calories)
}


fn parse_Line(l: &str) -> Ingredient {
    let mut i = l.split_whitespace();
    let name = i.next().unwrap().trim_end_matches(':');
    let capacity: i32 = i.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
    let durability: i32 = i.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
    let falvor: i32 = i.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
    let texture: i32 = i.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
    let calories: i32 = i.nth(1).unwrap().parse().unwrap();
    Ingredient::new(name, capacity, durability, falvor, texture, calories)
}


fn read_input<P, R>(filename: P, s: fn(&str) -> R) -> Vec<R>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let x = buf.lines();
    let y = x.map(|l| s(l.unwrap().as_ref()));
    y.collect()
}

fn main() {
    let profiles = read_input("data/15/input.txt", parse_Line);
    println!("{:?}", profiles);
    let mut maxrating_a: i32 = 0;
    let mut maxrating_b: i32 = 0;
    let mut mixture: Vec<i32> = Vec::new();
    for a in 1..100-profiles.len() {
        for b in (1..100-a).rev() {
            for c in (1..100-a-b).rev() {
                mixture.clear();
                mixture.push(a as i32);
                mixture.push(b as i32);
                mixture.push(c as i32);
                mixture.push(100_i32-(a as i32)-(b as i32)-(c as i32));
                let (rating, calories) = rate_recipe(&profiles, &mixture); 
                maxrating_a = maxrating_a.max(rating);
                if calories == 500 {
                    maxrating_b = maxrating_b.max(rating);
                }
            }
        }
    }
    println!("Part 1 - Answer {}", maxrating_a);
    println!("Part 2 - Answer {}", maxrating_b);
}

