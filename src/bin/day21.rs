//! --- Day 21: RPG Simulator 20XX ---
//! 
//! Little Henry Case got a new video game for Christmas. It's an RPG, and
//! he's stuck on a boss. He needs to know what equipment to buy at the shop.
//! He hands you the controller.
//! 
//! In this game, the player (you) and the enemy (the boss) take turns
//! attacking. The player always goes first. Each attack reduces the
//! opponent's hit points by at least 1. The first character at or below 0
//! hit points loses.
//! 
//! Damage dealt by an attacker each turn is equal to the attacker's damage
//! score minus the defender's armor score. An attacker always does at least
//! 1 damage. So, if the attacker has a damage score of 8, and the defender
//! has an armor score of 3, the defender loses 5 hit points. If the defender
//! had an armor score of 300, the defender would still lose 1 hit point.
//! 
//! Your damage score and armor score both start at zero. They can be
//! increased by buying items in exchange for gold. You start with no items
//! and have as much gold as you need. Your total damage or armor is equal to
//! the sum of those stats from all of your items. You have 100 hit points.
//! 
//! Here is what the item shop is selling:
//! 
//!     Weapons:    Cost  Damage  Armor
//!     Dagger        8     4       0
//!     Shortsword   10     5       0
//!     Warhammer    25     6       0
//!     Longsword    40     7       0
//!     Greataxe     74     8       0
//!     
//!     Armor:      Cost  Damage  Armor
//!     Leather      13     0       1
//!     Chainmail    31     0       2
//!     Splintmail   53     0       3
//!     Bandedmail   75     0       4
//!     Platemail   102     0       5
//!     
//!     Rings:      Cost  Damage  Armor
//!     Damage +1    25     1       0
//!     Damage +2    50     2       0
//!     Damage +3   100     3       0
//!     Defense +1   20     0       1
//!     Defense +2   40     0       2
//!     Defense +3   80     0       3
//! 
//! You must buy exactly one weapon; no dual-wielding. Armor is optional,
//! but you can't use more than one. You can buy 0-2 rings (at most one for
//! each hand). You must use any items you buy. The shop only has one of
//! each item, so you can't buy, for example, two rings of Damage +3.
//! 
//! For example, suppose you have 8 hit points, 5 damage, and 5 armor, and
//! that the boss has 12 hit points, 7 damage, and 2 armor:
//! 
//! The player deals 5-2 = 3 damage; the boss goes down to 9 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 6 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 6 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 4 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 3 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 2 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 0 hit points.
//! 
//! In this scenario, the player wins! (Barely.)
//! 
//! You have 100 hit points. The boss's actual stats are in your puzzle
//! input. What is the least amount of gold you can spend and still win
//! the fight?
//! 
//! Answer: 91
//! 
//! --- Part Two ---
//! 
//! Turns out the shopkeeper is working with the boss, and can persuade you
//! to buy whatever items he wants. The other rules still apply, and he still
//! only has one of each item.
//! 
//! What is the most amount of gold you can spend and still lose the fight?
//! 
//! Answer: 158
//! 
//! ----
//!  
//! Grundlagen für Teil 1:
//! Damit Spieler me gewinnen kann, muss der Schaden, dem er dem Chef zufügt,
//! größer sein als der erhaltene Schaden, beides bezogen auf die "Lebens-
//! stärke" (hier hitcount). Da nich nach Abschluss einer Runde sondern
//! sofort abgerechnet wird, ist hier auch Gleichheit des Schadens möglich, 
//! weil me immer zuerst schlägt.
//! 
//! Es muss gelten:
//! 
//!     (me_damage - chef_armor) / chef_hitcount >= (chef_damage - me_armor) / me_hitcount  
//! 
//! Die Spieldaten sind:
//!
//!     chef_armor = 2
//!     chef_damage = 8
//!     chef_hitcount = 100
//!     me_hitcount = 100
//! 
//! Damit vereinfacht sich die Bedingung zu:
//! 
//!     me_damage + me_armor >= 10
//! 
//! Für Teil 2 ändert sich die Bedidngung zu
//! 
//!     me_damage + me_armor < 10

use itertools::Itertools;

#[derive(Debug, Clone)]
enum ItemType {
    Weapon,
    Shield,
    Ring
}

#[derive(Debug, Clone)]
struct Item {
    t: ItemType,
    name: String,
    damage: u32,
    armor: u32,
    price: u32,
}

impl Item {
    fn new(t: ItemType, name: String, damage: u32, armor: u32, price: u32) -> Self {
        Item{ t, name, damage, armor, price }
    }
}


fn waepons_list() -> Vec<Item> {
    let mut data = Vec::new();
    data.push(Item::new(ItemType::Weapon, "Dagger".to_owned(), 4, 0, 8));
    data.push(Item::new(ItemType::Weapon, "Shortsword".to_owned(), 5, 0, 10));
    data.push(Item::new(ItemType::Weapon, "Warhammer".to_owned(), 6, 0, 25));
    data.push(Item::new(ItemType::Weapon, "Longsword".to_owned(), 7, 0, 40));
    data.push(Item::new(ItemType::Weapon, "Greataxe".to_owned(), 8, 0, 74));
    data
}

fn shield_list() -> Vec<Item> {
    let mut data = Vec::new();
    data.push(Item::new(ItemType::Shield, "Unshielded".to_owned(), 0, 0, 0));
    data.push(Item::new(ItemType::Shield, "Leather".to_owned(), 0, 1, 13));
    data.push(Item::new(ItemType::Shield, "Chainmail".to_owned(), 0, 2, 31));
    data.push(Item::new(ItemType::Shield, "Splintmail".to_owned(), 0, 3, 53));
    data.push(Item::new(ItemType::Shield, "Bundedmail".to_owned(), 0, 4, 75));
    data.push(Item::new(ItemType::Shield, "Platemail".to_owned(), 0, 5, 102));
    data
}

fn ring_list() -> Vec<Item> {
    let mut data = Vec::new();
    data.push(Item::new(ItemType::Shield, "NoRing".to_owned(), 0, 0, 0));
    data.push(Item::new(ItemType::Shield, "Damage+1".to_owned(), 1, 0, 25));
    data.push(Item::new(ItemType::Shield, "Damage+2".to_owned(), 2, 0, 50));
    data.push(Item::new(ItemType::Shield, "Damage+3".to_owned(), 3, 0, 100));
    data.push(Item::new(ItemType::Shield, "Defense+1".to_owned(), 0, 1, 20));
    data.push(Item::new(ItemType::Shield, "Defense+2".to_owned(), 0, 2, 40));
    data.push(Item::new(ItemType::Shield, "Defense+3".to_owned(), 0, 3, 80));
    data
}

fn combinations(weapons: Vec<Item>, shields: Vec<Item>, rings: Vec<Item>) -> Vec<Vec<Item>> {
    let mut result = Vec::new();
    for w in &weapons {
        for s in &shields {
            for r in &rings {
                let mut data: Vec<Item> = Vec::new();
                data.push(w.clone());
                data.push(s.clone());
                data.push(r.clone());
                result.push(data);
            }
            for r in rings.iter().combinations(2) {
                let mut data: Vec<Item> = Vec::new();
                data.push(w.clone());
                data.push(s.clone());
                for i in r {
                    data.push(i.clone());
                }
                result.push(data);
            }
        }
    }
    
    result
}

fn main() {
    let data = combinations(waepons_list(), shield_list(), ring_list());
    let mut cheepest: &Vec<Item> = &Vec::new(); 
    let mut lowprice: u32 = 999999;
    let mut mostcostly: &Vec<Item> = cheepest;
    let mut highprice: u32 = 0;

    for i in &data {
        let strength = i.iter().map(|x| x.damage + x.armor).sum::<u32>();
        let price: u32 = i.iter().map(|x| x.price).sum();
        if strength >= 10 && price < lowprice {
                lowprice = price;
                cheepest = i;
        }
        if strength < 10 && price > highprice {
                highprice = price;
                mostcostly = i;
        }
    }
    println!("Answer Part 1: {} with {:?}", lowprice, cheepest);
    println!("Answer Part 2: {} with {:?}", highprice, mostcostly);
    


}




