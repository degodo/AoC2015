//!
//! --- Day 11: Corporate Policy ---
//!
//! Santa's previous password expired, and he needs help choosing a new one.
//!
//! To help him remember his new password after the old one expires, Santa
//! has devised a method of coming up with a password based on the previous
//! one. Corporate policy dictates that passwords must be exactly eight
//! lowercase letters (for security reasons), so he finds his new password by
//! incrementing his old password string repeatedly until it is valid.
//!
//! Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and
//! so on. Increase the rightmost letter one step; if it was z, it wraps
//! around to a, and repeat with the next letter to the left until one
//! doesn't wrap around.
//!
//! Unfortunately for Santa, a new Security-Elf recently started, and he has
//! imposed some additional password requirements:
//!
//! - Passwords must include one increasing straight of at least three
//!   letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip
//!   letters; abd doesn't count.
//! - Passwords may not contain the letters i, o, or l, as these letters can
//!   be mistaken for other characters and are therefore confusing.
//! - Passwords must contain at least two different, non-overlapping pairs
//!   of letters, like aa, bb, or zz.
//!
//! For example:
//! hijklmmn meets the first requirement (because it contains the straight
//!          hij) but fails the second requirement requirement (because it
//!          contains i and l).
//! abbceffg meets the third requirement (because it repeats bb and ff) but
//!          fails the first requirement.
//! abbcegjk fails the third requirement, because it only has one double
//!          letter (bb).
//! The next password after abcdefgh is abcdffaa.
//! The next password after ghijklmn is ghjaabcc, because you eventually
//! skip all the passwords that start with ghi..., since i is not allowed.
//!
//! Given Santa's current password (your puzzle input), what should his next
//! password be?
//!
//! Your puzzle input is hxbxwxba.
//! Answer:  hxbxxyzz
//!
//!--- Part Two ---
//! Santa's password expired again. What's the next one?
//! 
//! Answer: hxcaabcc
//!

#[derive(Debug, PartialEq)]
enum Status {
    Start,
    Seq1,
    Seq,
    SeqDub,
    SeqDubDub1,
    Dub,
    DubSeq1,
    DubSeq,
    DubDub1,
    DubDub,
    DubDubSeq1,
    Success,
    Fail,
}

enum Transition {
    C,
    Cplus,
    N,
    Nstar,
    Nminus,
}

struct StateMachine {
    state: Status,
    ch: char,
    tickCounter: u32,
}

impl StateMachine {
    pub fn init(ch: char) -> StateMachine {
        StateMachine {
            state: Status::Start,
            ch,
            tickCounter: 0,
        }
    }

    pub fn is_Final(&self) -> bool {
        self.state == Status::Success
    }

    pub fn is_Failed(&self) -> bool {
        self.state == Status::Fail
    }

    fn updateState(&mut self, st: Status, ch: char) {
        self.state = st;
        self.ch = ch;
    }

    pub fn tick(&mut self, ch: char) {
        let newState: Status;

        if ch == 'i' || ch == 'l' || ch == 'o' {
            newState = Status::Fail;
        } else {
            newState = match &self.state {
                Status::Start => {
                    if self.ch == ch {
                        Status::Dub
                    } else if nextChar(self.ch) == 'a' {
                        Status::Start
                    } else if nextChar(self.ch) == ch {
                        Status::Seq1
                    } else {
                        Status::Start
                    }
                }
                Status::Seq1 => {
                    if self.ch == ch {
                        Status::Dub
                    } else if nextChar(self.ch) == 'a' {
                        Status::Start
                    } else if nextChar(self.ch) == ch {
                        Status::Seq
                    } else {
                        Status::Start
                    }
                }
                Status::Seq => {
                    if self.ch == ch {
                        Status::SeqDub
                    } else {
                        Status::Seq
                    }
                }
                Status::SeqDub => {
                    if self.ch != ch {
                        Status::SeqDubDub1
                    } else {
                        Status::SeqDub
                    }
                }
                Status::SeqDubDub1 => {
                    if self.ch == ch {
                        Status::Success
                    } else {
                        Status::SeqDubDub1
                    }
                }
                Status::Dub => {
                    if self.ch == ch || nextChar(self.ch) == 'a' {
                        Status::Dub
                    } else if nextChar(self.ch) == ch {
                        Status::DubSeq1
                    } else {
                        Status::DubDub1
                    }
                }
                Status::DubSeq1 => {
                    if self.ch == ch {
                        Status::DubDub
                    } else if nextChar(self.ch) == 'a' {
                        Status::Dub
                    } else if nextChar(self.ch) == ch {
                        Status::DubSeq
                    } else {
                        Status::Dub
                    }
                }
                Status::DubSeq => {
                    if self.ch == ch {
                        Status::Success
                    } else {
                        Status::DubSeq
                    }
                }
                Status::DubDub1 => {
                    if self.ch == ch {
                        Status::DubDub
                    } else if nextChar(self.ch) == 'a' {
                        Status::DubDub1
                    } else if nextChar(self.ch) == ch {
                        Status::DubSeq1
                    } else {
                        Status::DubDub1
                    }
                }
                Status::DubDub => {
                    if nextChar(self.ch) == 'a' {
                        Status::DubDub
                    } else if nextChar(self.ch) == ch {
                        Status::DubDubSeq1
                    } else {
                        Status::DubDub
                    }
                }
                Status::DubDubSeq1 => {
                    if nextChar(self.ch) == 'a' {
                        Status::DubDub
                    } else if nextChar(self.ch) == ch {
                        Status::Success
                    } else {
                        Status::DubDubSeq1
                    }
                }
                Status::Success => Status::Success,
                Status::Fail => Status::Fail,
            };
        };
        self.state = newState;
        self.ch = ch;
    }
}

/// Berechnet das nächste Zeichen aus dem Alplabet nach der Regel: zwischen
/// 'a' und 'y' wähle den nächsten Zeichen und überspringe 'o', 'l' und 'o'.
/// Nach 'z' kommt 'a'.  
fn nextChar(ch: char) -> char {
    let mut b_char = u32::from(ch);

    if b_char < 122 && b_char > 96 {
        b_char += 1;
        if b_char == 105 || b_char == 108 || b_char == 111 {
            b_char += 1;
        }
        char::from_u32(b_char).unwrap()
    } else {
        'a'
    }
}


fn nextString(s: &str) -> String {
    let mut chs: Vec<char> = s.chars().collect();
    for i in (0..chs.len()).rev() {
        chs[i] = nextChar(chs[i]);
        if chs[i] != 'a' {
            break;
        }
    }
    chs.iter().collect()
}

fn validateString(s: &str) -> bool {
    let mut ch_iter = s.chars();
    let mut statemachine = StateMachine::init(ch_iter.next().unwrap());
    
    for i in  ch_iter {
        statemachine.tick(i);
    }
    
    statemachine.is_Final()
}

fn main() {
    let mut s = String::from("hxbxwxba");
    s = nextString(&s);
    while !validateString(&s) {
        s = nextString(&s);
    }
    println!("{} is {}", s, validateString(&s));
    s = nextString(&s);
    while !validateString(&s) {
        s = nextString(&s);
    }
    println!("{} is {}", s, validateString(&s));


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nextChar() {
        assert_eq!('b', nextChar('a'));
        assert_eq!('j', nextChar('h'));
        assert_eq!('j', nextChar('i'));
        assert_eq!('p', nextChar('n'));
        assert_eq!('p', nextChar('o'));
        assert_eq!('m', nextChar('k'));
        assert_eq!('m', nextChar('l'));
        assert_eq!('a', nextChar('z'));
    }

    #[test]
    fn test_StateMachine() {
        // aabcdd
        let mut s = StateMachine::init('a');
        s.tick('a');
        assert_eq!(s.state, Status::Dub);
        s.tick('b');
        assert_eq!(s.state, Status::DubSeq1);
        s.tick('c');
        assert_eq!(s.state, Status::DubSeq);
        s.tick('d');
        assert_eq!(s.state, Status::DubSeq);
        s.tick('d');
        assert_eq!(s.state, Status::Success);
        s.tick('i');
        assert_eq!(s.state, Status::Fail);
    }

    #[test]
    fn test_validateString() {
        assert!(validateString("aabcdd"));
        assert!(validateString("aabdefgg"));
        assert!(!validateString("aaabcdef"));
        assert!(!validateString("ddefghikk"));
        assert!(!validateString("hxbxwxba"));
    }

    #[test]
    fn test_nextString() {
        assert_eq!("abc", nextString("abb"));
        assert_eq!("abj", nextString("abh"));
        assert_eq!("aca", nextString("abz"));
        assert_eq!("baa", nextString("azz"));
        
    }
}
