#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;
use std::collections::HashMap;

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse<'a>(text: &'a str) -> Passport {
    let mut p = Passport::new();
    let kvs: Vec<&str> = text.split_whitespace().collect();
    for kv in kvs {
        let parts: Vec<&str> = kv.split(":").collect();
        p.insert(parts[0], parts[1]);
    }
    return p;
}

fn ppvalid(p: &Passport) -> bool {
    /*
    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID) -- optional
    */
    return
       p.contains_key("byr") &&
       p.contains_key("iyr") &&
       p.contains_key("eyr") &&
       p.contains_key("hgt") &&
       p.contains_key("hcl") &&
       p.contains_key("ecl") &&
       p.contains_key("pid");
}

// passport validator for part 2
fn yearinrange(yr: &str, min: u32, max: u32) -> bool {
    lazy_static! {
        static ref re_year: Regex = Regex::new(r"^\d{4}$").unwrap();
    }
    if !re_year.is_match(yr) { return false };
    let value = yr.parse::<u32>().unwrap();
    if (value < min) { return false; }
    if (value > max) { return false; }
    return true;
}

fn ppvalid2(p: &Passport) -> bool {
    /*
     * byr (Birth Year) - four digits; at least 1920 and at most 2002.
     * iyr (Issue Year) - four digits; at least 2010 and at most 2020.
     * eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
     * hgt (Height) - a number followed by either cm or in:
     * If cm, the number must be at least 150 and at most 193.
     * If in, the number must be at least 59 and at most 76.
     * hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
     * ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
     * pid (Passport ID) - a nine-digit number, including leading zeroes.
     * cid (Country ID) - ignored, missing or not.
     */
   if !(p.contains_key("byr") &&
        p.contains_key("iyr") &&
        p.contains_key("eyr") &&
        p.contains_key("hgt") &&
        p.contains_key("hcl") &&
        p.contains_key("ecl") &&
        p.contains_key("pid")) { return false; }
   if !yearinrange(p["byr"], 1920, 2002) { return false; }
   if !yearinrange(p["iyr"], 2010, 2020) { return false; }
   if !yearinrange(p["eyr"], 2020, 2030) { return false; }

   lazy_static! {
       static ref re_hgt: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
       static ref re_hcl: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
       static ref re_ecl: Regex = Regex::new(
           r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
       static ref re_pid: Regex = Regex::new(
           r"^\d{9}$").unwrap();
   }
   if !re_hcl.is_match(p["hcl"]) { return false; }
   if !re_ecl.is_match(p["ecl"]) { return false; }
   if !re_pid.is_match(p["pid"]) { return false; }

   let parts = re_hgt.captures(p["hgt"]);
   if parts.is_none() { return false; }
   let parts = parts.unwrap();
   let value = parts.get(1).unwrap().as_str().parse::<u32>().unwrap();
   if parts.get(2).unwrap().as_str() == "in" {
       if (value < 59) || (value > 76) { return false; }
   } else {
       if (value < 150) || (value > 193) { return false; }
   }

   return true; 
}

fn main() {
    // let inputfile = "example.txt";
    let inputfile = "input.txt";

    let contents = fs::read_to_string(inputfile)
                 .expect("Something went wrong reading the file");
    let records: Vec<&str> = contents.split("\n\n")
        .map(|x| x.trim()).collect();
    let passports: Vec<Passport> = records.iter()
        .map(|x| parse(x)).collect();
    let valids: Vec<bool> = passports.iter().map(|x| ppvalid(&x)).collect();
    dbg!(records.len());
    // dbg!(&valids);
    dbg!(valids.iter().filter(|x| **x).count());
    let p2_valids: Vec<bool> = passports.iter().map(|x| ppvalid2(&x)).collect();
    dbg!(p2_valids.iter().filter(|x| **x).count());
}
