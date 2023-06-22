//!
//! --- Day 14: Reindeer Olympics ---
//! 
//! This year is the Reindeer Olympics! Reindeer can fly at high speeds, but
//! must rest occasionally to recover their energy. Santa would like to know
//! which of his reindeer is fastest, and so he has them race.
//! 
//! Reindeer can only either be flying (always at their top speed) or resting 
//! (not moving at all), and always spend whole seconds in either state.
//! 
//! For example, suppose you have the following Reindeer:
//! 
//! Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
//! Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
//! 
//! After one second, Comet has gone 14 km, while Dancer has gone 16 km. After
//! ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the
//! eleventh second, Comet begins resting (staying at 140 km), and Dancer
//! continues on for a total distance of 176 km. On the 12th second, both
//! reindeer are resting. They continue to rest until the 138th second, when
//! Comet flies for another ten seconds. On the 174th second, Dancer flies for 
//! another 11 seconds.
//!         
//! In this example, after the 1000th second, both reindeer are resting, and
//! Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by
//! that point). So, in this situation, Comet would win (if the race ended at
//! 1000 seconds).
//! 
//! Given the descriptions of each reindeer (in your puzzle input), after
//! exactly 2503 seconds, what distance has the winning reindeer traveled?
//! 
//! Answer: 
//! 
//! 
//! --- Part Two ---
//! 
//! Seeing how reindeer move in bursts, Santa decides he's not pleased with
//! the old scoring system.
//! 
//! Instead, at the end of each second, he awards one point to the reindeer
//! currently in the lead. (If there are multiple reindeer tied for the lead,
//! they each get one point.) He keeps the traditional 2503 second time limit,
//! of course, as doing otherwise would be entirely ridiculous.
//! 
//! Given the example reindeer from above, after the first second, Dancer is
//! in the lead and gets one point. He stays in the lead until several seconds
//! into Comet's second burst: after the 140th second, Comet pulls into the
//! lead and gets his first point. Of course, since Dancer had been in the
//! lead for the 139 seconds before that, he has accumulated 139 points by the
//! 140th second.
//! 
//! After the 1000th second, Dancer has accumulated 689 points, while poor 
//! Comet, our old champion, only has 312. So, with the new scoring system,
//! Dancer would win (if the race ended at 1000 seconds).
//! 
//! Again given the descriptions of each reindeer (in your puzzle input),
//! after exactly 2503 seconds, how many points does the winning reindeer
//! have?
//! 
//! Answer: 1084



use std::collections::HashMap;
use std::fs::File; 
use std::io::{BufRead, BufReader};


#[derive(Debug)]
struct Profile {
    speed: u32,
    duration: u32,
    pause: u32,
}


impl Profile {
    fn  new(speed: u32, duration: u32, pause: u32) -> Profile {
        Profile{speed, duration, pause}
    }
}


#[derive(Debug)]
enum Actions {
    run,
    pause,
}

#[derive(Debug)]
struct StepState {
    name: String, 
    speed: u32,
    duration: u32,
    pause: u32,
    distance: u32,
    points: u32,
    action: Actions,
    remain: u32,
}


impl StepState {
    fn new(n: &str, p: &Profile) -> StepState {
        StepState{
            name: n.to_string(),
            speed: p.speed,
            duration: p.duration,
            pause: p.pause,
            distance: 0,
            points: 0,
            action: Actions::run,
            remain: p.duration,
        }
    }

    fn doStep(&mut self) -> u32 {
        self.remain -= 1;
        match self.action {
            Actions::run => {
                self.distance += self.speed;
                if self.remain == 0 {
                    self.remain = self.pause;
                    self.action = Actions::pause;
                }
            },
            Actions::pause => {
                if self.remain == 0 {
                    self.remain = self.duration;
                    self.action = Actions::run;
                }
             }
        }
        self.distance
    }
    
    fn doBonus(&mut self, d: u32) {
        if self.distance == d {
            self.points += 1;
        }
    }
}


fn solve_part1(p: &HashMap<String,Profile>) {
    let mut maxDistance: u32 = 0;
    for i in p.values() {
        let cycleDuration = i.duration + i.pause;
        let cycleDistrance = i.duration * i.speed;
        let cycles = 2503 / cycleDuration;
        let excessTime = u32::min(2503 - cycles * cycleDuration, i.duration);
        let distance = cycles * cycleDistrance + excessTime * i.speed;
        if distance > maxDistance {
            maxDistance = distance;
        }    
    }    
    println!("größte Entfernung: {}", maxDistance);
}    



fn solve_part2(p: &HashMap<String, Profile>) {
    let mut s: Vec<StepState> = Vec::new();
    for (name, profile) in p.iter() {
        s.push(StepState::new(name, profile))
    }
    for i in 1..2504 {
        let mut maxdistance = 0;
        for j in s.iter_mut() {
            maxdistance = maxdistance.max(j.doStep());
        }
        for j in s.iter_mut() {
            j.doBonus(maxdistance);
        }
    }
    println!("Ergebnis Teil 2: {}", s.iter().max_by(|a, b| a.points.cmp(&b.points)).unwrap().points);
}


fn parse_Line(l: &str) -> (String, Profile) {
    let mut i = l.split_whitespace();
    (i.next().unwrap().to_owned(),
     Profile::new(i.nth(2).unwrap().parse::<u32>().unwrap(),
     i.nth(2).unwrap().parse::<u32>().unwrap(),
     i.nth(6).unwrap().parse::<u32>().unwrap()))
}


fn read_input<P, R>(filename: P, s: fn(&str) -> (String,R)) -> HashMap<String, R>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let x = buf.lines();
    let y = x.map(|l| s(l.unwrap().as_ref()));
    let mut res: HashMap<String, R> = HashMap::new();
    y.for_each(|(key, value)| { res.insert(key, value); });
    res
}

fn main() {
    let profiles = read_input("data/14/input.txt", parse_Line);
    solve_part1(&profiles);
    solve_part2(&profiles);
}

