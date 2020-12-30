#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;

// part 1 evaluator, quick and dirty.
fn eval_iterator<I>(ch_iter: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    let mut value = 0i64;
    let mut op: char = '+';
    loop {
        let ch = ch_iter.next();
        if ch.is_none() {
            break;
        }
        let ch = ch.unwrap();
        let mut rhs: i64 = (ch as i64) - ('0' as i64);
        if ch == '(' {
            rhs = eval_iterator(ch_iter);
        }
        match ch {
            '0'..='9' | '(' => match op {
                '+' => value += rhs,
                '-' => value -= rhs,
                '*' => value *= rhs,
                _ => panic!("Unknown op {}", op),
            },
            ')' => break,
            '+' | '-' | '*' => op = ch,
            ' ' => (),
            _ => panic!("Unknown token: {}", ch),
        }
    } // loop
    return value;
}

fn eval(expr: &str) -> i64 {
    let x = eval_iterator(&mut expr.chars());
    println!("{} = {}", expr, x);
    return x;
}

// part 2 evaluator
//
// I'm not proud (well, I am a bit), but I used regular expressions to implement
// the different operations.  I know I'm supposed to tokenize the input stream,
// build a syntax tree, and then evaluate, but I decided to optimize for programmer
// time on this one.

// Here are a bunch of structs that implement the regex::Replace trait, for use
// with Regex::replace calls below.
struct SubgroupReplacer;
impl regex::Replacer for SubgroupReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let value = p2_eval(caps.get(1).unwrap().as_str()); // recurse.
        dst.push_str(&(value.to_string()));
    }
}

struct MultReplacer;
impl regex::Replacer for MultReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let lhs = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let rhs = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let value = lhs * rhs;
        dst.push_str(&(value.to_string()));
    }
}

struct AddReplacer;
impl regex::Replacer for AddReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let lhs = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let rhs = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let value = lhs + rhs;
        dst.push_str(&(value.to_string()));
    }
}

fn p2_eval(expr: &str) -> i64 {
    lazy_static! {
        static ref RE_SUBEXPR: Regex = Regex::new(r"\(([0-9 \+\*]+)\)").unwrap();
        static ref RE_ADD: Regex = Regex::new(r"(\d+)\s*\+\s*(\d+)").unwrap();
        static ref RE_MULT: Regex = Regex::new(r"(\d+)\s*\*\s*(\d+)").unwrap();
    }
    let mut e: String = expr.to_owned();

    // Unfortunately, Regex::Replace returns std::borrow::Cow<String>, so we
    // would need to perform a string comparison to see if a change was made.
    // It was simpler just to search for operator tokens.
    while e.contains("(") {
        e = RE_SUBEXPR.replace(&e, SubgroupReplacer {}).to_string();
    }
    while e.contains("+") {
        e = RE_ADD.replace(&e, AddReplacer {}).to_string();
    }
    while e.contains("*") {
        e = RE_MULT.replace(&e, MultReplacer {}).to_string();
    }
    return e.parse::<i64>().unwrap();
}

fn main() {
    part1();
    part2();
}

fn part1() {
    // Quick test cases:
    assert_eq!(eval("1 + 1"), 2);
    assert_eq!(eval("2 * 3"), 6);
    assert_eq!(eval("1 + 2 * 4"), 12);
    assert_eq!(eval("1 + (2 * 4)"), 9);
    assert_eq!(eval("1 + ((2 * 4))"), 9);
    assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let p1_result: i64 = contents.lines().map(|x| eval(x)).sum();
    dbg!(p1_result);
}

fn part2() {
    // Quick test cases:
    assert_eq!(p2_eval("1 + 1"), 2);
    assert_eq!(p2_eval("2 * 3"), 6);
    assert_eq!(p2_eval("1 + 2 * 4"), 12);
    assert_eq!(p2_eval("3 * 2 + 4"), 18);
    assert_eq!(p2_eval("(3 * 2) + 4"), 10);
    assert_eq!(p2_eval("1 + (2 * 4)"), 9);
    assert_eq!(p2_eval("1 + ((2 * 4))"), 9);
    assert_eq!(p2_eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        p2_eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let p2_result: i64 = contents.lines().map(|x| p2_eval(x)).sum();
    dbg!(p2_result);
}
