//! --- Day 22: Wizard Simulator 20XX ---
//! 
//! Little Henry Case decides that defeating bosses with swords and stuff is
//! boring. Now he's playing the game with a wizard. Of course, he gets stuck
//! on another boss and needs your help again.
//! 
//! In this version, combat still proceeds with the player and the boss taking
//! alternating turns. The player still goes first. Now, however, you don't
//! get any equipment; instead, you must choose one of your spells to cast.
//! The first character at or below 0 hit points loses.
//! 
//! Since you're a wizard, you don't get to wear armor, and you can't attack
//! normally. However, since you do magic damage, your opponent's armor is
//! ignored, and so the boss effectively has zero armor as well. As before,
//! if armor (from a spell, in this case) would reduce damage below 1, it
//! becomes 1 instead - that is, the boss' attacks always deal at least 1
//! damage.
//! 
//! On each of your turns, you must select one of your spells to cast. If you
//! cannot afford to cast any spell, you lose. Spells cost mana; you start
//! with 500 mana, but have no maximum limit. You must have enough mana to
//! cast a spell, and its cost is immediately deducted when you cast it.
//! Your spells are Magic Missile, Drain, Shield, Poison, and Recharge.
//! 
//!    * **Magic** Missile costs 53 mana. It instantly does 4 damage.
//!    * **Drain** costs 73 mana. It instantly does 2 damage and heals you for 2
//!      hit points.
//!    * **Shield** costs 113 mana. It starts an effect that lasts for 6 turns.
//!      While it is active, your armor is increased by 7.
//!    * **Poison** costs 173 mana. It starts an effect that lasts for 6 turns.
//!      At the start of each turn while it is active, it deals the boss 3
//!      damage.
//!    * **Recharge** costs 229 mana. It starts an effect that lasts for 5 turns.
//!      At the start of each turn while it is active, it gives you 101 new
//!      mana.
//!  
//! Effects all work the same way. Effects apply at the start of both the
//! player's turns and the boss' turns. Effects are created with a timer (the
//! number of turns they last); at the start of each turn, after they apply
//! any effect they have, their timer is decreased by one. If this decreases
//! the timer to zero, the effect ends. You cannot cast a spell that would
//! start an effect which is already active. However, effects can be started
//! on the same turn they end.
//! 
//! For example, suppose the player has 10 hit points and 250 mana, and that
//! the boss has 13 hit points and 8 damage:
//! 
//! 
//!     -- Player turn --
//!     - Player has 10 hit points, 0 armor, 250 mana
//!     - Boss has 13 hit points   
//!     Player casts Poison.
//!     
//!     -- Boss turn --
//!     - Player has 10 hit points, 0 armor, 77 mana
//!     - Boss has 13 hit points   
//!     Poison deals 3 damage; its timer is now 5.
//!     Boss attacks for 8 damage.
//!     
//!     -- Player turn --
//!     - Player has 2 hit points, 0 armor, 77 mana
//!     - Boss has 10 hit points   
//!     Poison deals 3 damage; its timer is now 4.
//!     Player casts Magic Missile, dealing 4 damage.
//!     
//!     -- Boss turn --
//!     - Player has 2 hit points, 0 armor, 24 mana
//!     - Boss has 3 hit points   
//!     Poison deals 3 damage. This kills the boss, and the player wins.
//!     Now, suppose the same initial conditions, except that the boss has
//!     14 hit points instead:
//!     
//!     -- Player turn --
//!     - Player has 10 hit points, 0 armor, 250 mana
//!     - Boss has 14 hit points   
//!     Player casts Recharge.
//!     
//!     -- Boss turn --
//!     - Player has 10 hit points, 0 armor, 21 mana
//!     - Boss has 14 hit points   
//!     Recharge provides 101 mana; its timer is now 4.
//!     Boss attacks for 8 damage!
//!     
//!     -- Player turn --
//!     - Player has 2 hit points, 0 armor, 122 mana
//!     - Boss has 14 hit points   
//!     Recharge provides 101 mana; its timer is now 3.
//!     Player casts Shield, increasing armor by 7.
//!     
//!     -- Boss turn --
//!     - Player has 2 hit points, 7 armor, 110 mana
//!     - Boss has 14 hit points   
//!     Shield's timer is now 5.   
//!     Recharge provides 101 mana; its timer is now 2.
//!     Boss attacks for 8 - 7 = 1 damage!
//!     
//!     -- Player turn --
//!     - Player has 1 hit point, 7 armor, 211 mana
//!     - Boss has 14 hit points   
//!     Shield's timer is now 4.   
//!     Recharge provides 101 mana; its timer is now 1.
//!     Player casts Drain, dealing 2 damage, and healing 2 hit points.
//!     
//!     -- Boss turn --
//!     - Player has 3 hit points, 7 armor, 239 mana
//!     - Boss has 12 hit points  
//!     Shield's timer is now 3.
//!     Recharge provides 101 mana; its timer is now 0.
//!     Recharge wears off.
//!     Boss attacks for 8 - 7 = 1 damage!
//!     
//!     -- Player turn --
//!     - Player has 2 hit points, 7 armor, 340 mana
//!     - Boss has 12 hit points   
//!     Shield's timer is now 2.
//!     Player casts Poison.
//!     
//!     -- Boss turn --
//!     - Player has 2 hit points, 7 armor, 167 mana
//!     - Boss has 12 hit points   
//!     Shield's timer is now 1.
//!     Poison deals 3 damage; its timer is now 5.
//!     Boss attacks for 8 - 7 = 1 damage!
//!     
//!     -- Player turn --
//!     - Player has 1 hit point, 7 armor, 167 mana
//!     - Boss has 9 hit points   
//!     Shield's timer is now 0.
//!     Shield wears off, decreasing armor by 7.
//!     Poison deals 3 damage; its timer is now 4.
//!     Player casts Magic Missile, dealing 4 damage.
//!     
//!     -- Boss turn --
//!     - Player has 1 hit point, 0 armor, 114 mana
//!     - Boss has 2 hit points   
//!     Poison deals 3 damage. This kills the boss, and the player wins.
//!     You start with 50 hit points and 500 mana points. The boss's actual 
//!     stats are in your puzzle input. What is the least amount of mana you
//!     can spend and still win the fight? (Do not include mana recharge
//!     effects as "spending" negative mana.)
//!    
//! Answer: 953
//! 
//! --- Part Two ---
//! 
//! On the next run through the game, you increase the difficulty to hard.
//! 
//! At the start of each player turn (before any other effects apply), you
//! lose 1 hit point. If this brings you to or below 0 hit points, you lose.
//! 
//! With the same starting stats for you and the boss, what is the least
//! amount of mana you can spend and still win the fight?
//! 
//! Answer: 1289





#[derive(Default, Debug, Clone, Copy)]
struct Data {
    boss_points: u32,
    me_points: u32,
    balance: u32,
    spent: u32,
    spent_min: u32,
    shield_ends: u32,
    poison_ends: u32,
    recharge_ends:u32,
    counter: u32,
    hard: bool,
}

impl Data {
    fn new(boss_points: u32, me_points: u32, balance: u32) -> Data {
        Data{ boss_points, me_points, balance, spent_min: u32::MAX, ..Default::default() }
    }

    fn exec_boss_hits(&mut self, n: u32) -> bool {
        if self.boss_points > n {
            self.boss_points -= n;
            true
        } else {
            false
        }
    }
    
    fn exec_me_hits(&mut self, n: u32) -> bool {
        if self.me_points > n {
            self.me_points -= n;
            true
        } else {
            false
        }
    }

    fn withdraw(&mut self, n: u32) -> bool {
        if n <= self.balance {
            self.balance -= n;
            self.spent += n;
            true
        } else {
            false
        }
    }
}

fn cast_magic_missile(mut data: Data) -> Option<u32> {
    data.counter += 1;
    if data.withdraw(53) {
        let damage: u32 = if data.poison_ends >= data.counter { 7 } else { 4 };
        if data.recharge_ends >= data.counter {
            data.balance += 101;
        }
        data.counter += 1;
        if data.exec_boss_hits(damage) {
            if data.poison_ends >= data.counter && ! data.exec_boss_hits(3) {
                return Some(data.spent);
            }
            let damage: u32 = if data.shield_ends >= data.counter { 1 } else { 8 };
            if data.recharge_ends >= data.counter {
                data.balance += 101;
            }
            if data.exec_me_hits(damage) {
                return eval(data);
            }
        } else {
            return Some(data.spent);
        }
    }
    None
}

fn cast_drain(mut data: Data) -> Option<u32> {
    data.counter += 1;
    if data.withdraw(73) {
        let damage: u32 = if data.poison_ends >= data.counter { 5 } else { 2 };
        if data.recharge_ends >= data.counter {
            data.balance += 101;
        }
        data.me_points += 2;
        data.counter += 1;
        if data.exec_boss_hits(damage) {
            if data.poison_ends >= data.counter && ! data.exec_boss_hits(3) {
                return Some(data.spent);
            }
            let damage: u32 = if data.shield_ends >= data.counter { 1 } else { 8 };
            if data.recharge_ends >= data.counter {
                data.balance += 101;
            }
            if data.exec_me_hits(damage) {
                return eval(data);
            }
        } else {
            return Some(data.spent);
        }
    }
    None
}

fn cast_shield(mut data: Data) -> Option<u32> {
    data.counter += 1;
    if data.shield_ends <= data.counter && data.withdraw(113) {
        let damage: u32 = if data.poison_ends >= data.counter { 3 } else { 0 };
        if data.recharge_ends >= data.counter {
            data.balance += 101;
        }
        data.shield_ends = data.counter + 6;
        data.counter += 1;
        if data.exec_boss_hits(damage) {
            if data.poison_ends >= data.counter && ! data.exec_boss_hits(3) {
                return Some(data.spent);
            }
            let damage: u32 = if data.shield_ends >= data.counter { 1 } else { 8 };
            if data.recharge_ends >= data.counter {
                data.balance += 101;
            }
            if data.exec_me_hits(damage) {
                return eval(data);
            }
        } else {
            return Some(data.spent);
        }
    }
    None
}

fn cast_poison(mut data: Data) -> Option<u32> {
    data.counter += 1;
    if data.poison_ends <= data.counter && data.withdraw(173) {
        let damage: u32 = if data.poison_ends >= data.counter { 3 } else { 0 };
        if data.recharge_ends >= data.counter {
            data.balance += 101;
        }
        data.poison_ends = data.counter + 6;
        data.counter += 1;
        if data.exec_boss_hits(damage) {
            if data.poison_ends >= data.counter && ! data.exec_boss_hits(3) {
                return Some(data.spent);
            }
            let damage: u32 = if data.shield_ends >= data.counter { 1 } else { 8 };
            if data.recharge_ends >= data.counter {
                data.balance += 101;
            }
            if data.exec_me_hits(damage) {
                return eval(data);
            }
        } else {
            return Some(data.spent);
        }
    }
    None
}

fn cast_recharge(mut data: Data) -> Option<u32> {
    data.counter += 1;
    if data.recharge_ends <= data.counter && data.withdraw(229) {
        let damage: u32 = if data.poison_ends >= data.counter { 3 } else { 0 };
        if data.recharge_ends >= data.counter {
            data.balance += 101;
        }
        data.recharge_ends = data.counter + 5;
        data.counter += 1;
        if data.exec_boss_hits(damage) {
            if data.poison_ends >= data.counter && ! data.exec_boss_hits(3) {
                return Some(data.spent);
            }
            let damage: u32 = if data.shield_ends >= data.counter { 1 } else { 8 };
            if data.recharge_ends >= data.counter {
                data.balance += 101;
            }
            if data.exec_me_hits(damage) {
                return eval(data);
            }
        } else {
            return Some(data.spent);
        }
    }
    None
}


fn eval(mut data: Data) -> Option<u32> {
    if data.spent >= data.spent_min {
        return None;
    }
    if data.hard && ! data.exec_me_hits(1) {
        return None;
    }
    match cast_magic_missile(data) {
        Some(spent) => { data.spent_min = data.spent_min.min(spent); }
        None => (),
    }
    match cast_drain(data) {
        Some(spent) => { data.spent_min = data.spent_min.min(spent);}
        None => (),
    }
    match cast_shield(data) {
        Some(spent) => { data.spent_min = data.spent_min.min(spent); }
        None => (),
    }
    match cast_poison(data) {
        Some(spent) => { data.spent_min = data.spent_min.min(spent); }
        None => (),
    }
    match cast_recharge(data) {
        Some(spent) => { data.spent_min = data.spent_min.min(spent); }
        None => (),
    }
    Some(data.spent_min)
}

fn main() {
    let mut data = Data::new(55, 50, 500);
    match eval(data) {
        Some(i) => { println!("Part 1: {}", i); }
        None => ()
    };
    data.hard = true;
    match eval(data) {
        Some(i) => { println!("Part 2: {}", i); }
        None => ()
    };
}