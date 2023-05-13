//!
//! --- Day 9: All in a Single Night ---
//! 
//! Every year, Santa manages to deliver all of his presents in a single night.
//! 
//! This year, however, he has some new locations to visit; his elves have
//! provided him the distances between every pair of locations. He can start
//! and end at any two (different) locations he wants, but he must visit each
//! location exactly once. What is the shortest distance he can travel to
//! achieve this?
//! 
//! For example, given the following distances:
//! 
//! London to Dublin = 464
//! London to Belfast = 518
//! Dublin to Belfast = 141
//! 
//! The possible routes are therefore:
//! Dublin -> London -> Belfast = 982
//! London -> Dublin -> Belfast = 605
//! London -> Belfast -> Dublin = 659
//! Dublin -> Belfast -> London = 659
//! Belfast -> Dublin -> London = 605
//! Belfast -> London -> Dublin = 982
//! 
//! The shortest of these is London -> Dublin -> Belfast = 605, and so the
//! answer is 605 in this example.
//! 
//! What is the distance of the shortest route?
//! My Answer: 117
//! 
//! --- Part Two ---
//! 
//! The next year, just to show off, Santa decides to take the route with the
//! longest distance instead.
//! 
//! He can still start and end at any two (different) locations he wants, and
//! he still must visit each location exactly once.
//! 
//! For example, given the distances above, the longest route would be 982 via
//! (for example) Dublin -> London -> Belfast.
//! 
//! What is the distance of the longest route?
//! My Answer: 909


use std::collections::{HashSet, BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone, Debug, PartialEq)]
struct Distance{
    posA: String,
    posB: String,
    distance: u32,
}

impl Distance {
    fn new(s: &str) -> Distance {
        let mut tokenstream = s.split_whitespace();
        let A = tokenstream.next().unwrap();
        let B = tokenstream.nth(1).unwrap();
        let Ok(distance) = tokenstream.nth(1).unwrap().parse::<u32>() else { todo!() };
        Distance{posA: A.to_owned(), posB: B.to_owned(), distance}
    }

}

fn create_distanceMap(con: &Vec<Distance>) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    con.iter().for_each(|c| { map.insert(c.posA.to_owned() + ";" + &c.posB, c.distance); } );
    con.iter().for_each(|c| { map.insert(c.posB.to_owned() + ";" + &c.posA, c.distance); } );
    map
}


fn locations(connections: &Vec<Distance>) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    for connection in connections.iter() {
        set.insert(connection.posA.clone());
        set.insert(connection.posB.clone());
    }
    set
}

/// Calculate the next permutation of a lexicographically sortied sequense
/// following this schema:
///
/// (1) Finding the rightmost element which is smaller than the element to
///     its right.
/// (2) Swap that element with the smallest element to its right which is
///     larger than it.
/// (3) Reverse the part of the permutation to the right of where that
///     element was.
/// 
/// Both steps (1) and (3) are worst-case O(n), but it is easy to prove that
/// the average time for those steps is O(1).
fn permutate<T: PartialOrd>(sequence: &mut Vec<T>) -> bool {
    let mut rightmost: i32 = -1;
    let mut swapparty: usize;

    // (1) find rightmust elemet that is less than its right neighbour
    // Start from the last but one element and run to the first
    for i in (0..=(sequence.len()-2)).rev() {
        if sequence[i] < sequence[i+1] {
            rightmost = i as i32;
            break;
        }
    };
    if rightmost == -1 {
        // no further permutation possible
        return false;
    }
    // find largest element to the right that is smaller than rightmost
    swapparty = (rightmost as usize) + 1;
    for i in (rightmost as usize + 1)..=(sequence.len()-1) {
        if sequence[i] > sequence[rightmost as usize] {
            if sequence[i] < sequence[swapparty] {
                swapparty = i;
            }
        }
    }
    sequence.swap(rightmost as usize, swapparty);
    sequence[(rightmost+1) as usize..].reverse();
    return true;
}


fn calcDistanceCost(locations: &Vec<String>, locationOrder: &Vec<u32>, distanceCost: &HashMap<String, u32>) -> u32 {
    let mut distance: u32 = 0;
    let mut fromIdx: usize = 0;
    let mut toIdx: usize = 0;
    let mut distanceKey: String = String::new();
    
    for i in 0..locationOrder.len()-1 {
        fromIdx = locationOrder[i] as usize - 1;
        toIdx = locationOrder[i+1] as usize - 1;
        distanceKey.clear();
        distanceKey.push_str(&locations[fromIdx]);
        distanceKey.push(';');
        distanceKey.push_str(&locations[toIdx]);
        distance += distanceCost.get(&distanceKey).unwrap();
    }
    distance
}


fn factorial(num: u32) -> u32 {
    (1..=num).product()
}


fn locationsOrder_ToString(locations: &Vec<String>, order: &Vec<u32>) -> String {
    let mut result: String = locations[0].clone();
    for i in 1..order.len() {
        result.push(',');
        result.push_str(&locations[i]);
    }
    result
}


fn calc_fastest_path(connections: &Vec<Distance>) -> (String, u32) {
    let distanceMap: HashMap<String, u32> = create_distanceMap(&connections);
    let locations: Vec<String> = locations(&connections).drain().collect();
    let mut locationsOrder: Vec<u32> = (1..=locations.len() as u32).collect();
    let mut fastestOrder: Vec<u32> = locationsOrder.clone();
    let mut distance: u32 = calcDistanceCost(&locations, &locationsOrder, &distanceMap);

    for i in 1..factorial(locations.len() as u32) {
        permutate(&mut locationsOrder);
        let mut newdist = calcDistanceCost(&locations, &locationsOrder, &distanceMap);
        if newdist < distance {
           distance = newdist;
           fastestOrder = locationsOrder.clone();
        }
    }
    (locationsOrder_ToString(&locations, &fastestOrder), distance)
}


fn calc_slowest_path(connections: &Vec<Distance>) -> (String, u32) {
    let distanceMap: HashMap<String, u32> = create_distanceMap(&connections);
    let locations: Vec<String> = locations(&connections).drain().collect();
    let mut locationsOrder: Vec<u32> = (1..=locations.len() as u32).collect();
    let mut slowestOrder: Vec<u32> = locationsOrder.clone();
    let mut distance: u32 = calcDistanceCost(&locations, &locationsOrder, &distanceMap);

    for i in 1..factorial(locations.len() as u32) {
        permutate(&mut locationsOrder);
        let mut newdist = calcDistanceCost(&locations, &locationsOrder, &distanceMap);
        if newdist > distance {
           distance = newdist;
           slowestOrder = locationsOrder.clone();
        }
    }
    (locationsOrder_ToString(&locations, &slowestOrder), distance)
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
    let connections: Vec<Distance> = read_input("data/09/input.txt", Distance::new );
    let (tour, distance) = calc_fastest_path(&connections);
    println!("Fastest trip ist {} with distance {}", tour, distance);
    let (tour, distance) = calc_slowest_path(&connections);
    println!("slowest trip ist {} with distance {}", tour, distance);
}
