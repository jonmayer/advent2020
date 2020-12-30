use advent;
use regex;
use std::collections;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct BagRule {
    name: String,
    contains: HashMap<String, u32>,
    contained_by: Vec<String>,
}

impl BagRule {
    fn new_from_name(name: &str) -> BagRule {
        return BagRule {
            name: name.to_owned(),
            contains: HashMap::new(),
            contained_by: Vec::new(),
        };
    }

    fn parse(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        self.name = parts[0].to_owned();
        let re_bagdesc = regex::Regex::new(r"^(\d+) (\S+\s+\S+) bag").unwrap();
        if !(parts[1] == "no other bags.") {
            for d in parts[1].split(", ") {
                let caps = re_bagdesc.captures(d).unwrap();
                let count: u32 = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let bagtype = caps.get(2).unwrap().as_str().to_owned();
                self.contains.insert(bagtype, count);
            }
        }
    }

    fn new(text: &str) -> BagRule {
        let mut br = BagRule::new_from_name("unknown");
        br.parse(text);
        return br;
    }
}

#[derive(Debug)]
struct BagGraph {
    m: HashMap<String, BagRule>,
}

impl BagGraph {
    pub fn new() -> BagGraph {
        BagGraph { m: HashMap::new() }
    }

    fn get_mut_rule(&mut self, name: &str) -> &mut BagRule {
        if !self.m.contains_key(name) {
            let br = BagRule::new_from_name(name);
            self.m.insert(name.to_owned(), br);
        }
        return self.m.get_mut(name).unwrap();
    }

    fn get_rule(&self, name: &str) -> &BagRule {
        return self.m.get(name).unwrap();
    }

    fn parse(&mut self, text: &str) {
        for line in text.lines() {
            println!("{}", line);
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let mut br = self.get_mut_rule(parts[0]).clone();
            br.parse(line);
            for key in br.contains.keys() {
                if !self.m.contains_key(key) {
                    println!("making new {}", key);
                    let br = BagRule::new_from_name(key);
                    self.m.insert(key.to_owned(), br);
                }
                self.m
                    .get_mut(key)
                    .unwrap()
                    .contained_by
                    .push(br.name.to_owned());
                println!(
                    "  {} contained by {:?}",
                    key,
                    self.m.get(key).unwrap().contained_by
                );
            }
            self.m.insert(br.name.to_owned(), br);
        }
    }

    // search up: return the list of bags that can contain the named bag.
    fn search_up(&mut self, name: &str) -> Vec<String> {
        let mut visited = collections::HashSet::<String>::new();
        let mut checklist = collections::VecDeque::<String>::new();
        checklist.push_back(name.to_owned());
        while !checklist.is_empty() {
            let name = checklist.pop_front().unwrap();
            // dbg!(&name, &checklist);
            if visited.contains(&name) {
                continue;
            }
            visited.insert(name.to_owned());
            let br = self.get_rule(&name);
            // dbg!(&br);
            for other in br.contained_by.iter() {
                if !visited.contains(other) {
                    checklist.push_back(other.to_owned());
                }
            }
        }

        return visited.into_iter().collect();
    }

    // search down: how many bags are contained by the named bag?
    fn count_down(&self, name: &str) -> u32 {
        let mut count: u32 = 0;
        // count self:
        let br = self.get_rule(name);
        count += 1;
        // count children:
        for key in br.contains.keys() {
            count += br.contains[key] * self.count_down(key);
        }
        return count;
    }
}

fn main() {
    let mut part1_result = 0;
    let mut part2_result = 0;
    let content = advent::load_input();
    let mut graph = BagGraph::new();
    graph.parse(&content);
    // dbg!(&graph);

    let containers = graph.search_up("shiny gold");
    dbg!(&containers);
    part1_result = containers.len() - 1; // don't count "shiny gold" bag itself.
    part2_result = graph.count_down("shiny gold") - 1; // same.

    dbg!(part1_result);
    dbg!(part2_result);
}
