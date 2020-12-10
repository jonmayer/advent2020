use advent;
use std::collections::HashMap;
use regex;

#[derive(Debug)]
struct BagRule<'a> {
    name: &'a str,
    contains: HashMap<&'a str, u32>,
    contained_by: Vec<&'a str>,
}

#[derive(Debug)]
struct BagGraph<'a> {
    text: &'a str,
    bagmap: HashMap<&'a str, BagRule<'a>>,
}

impl<'a> BagGraph<'a> {
    pub fn new(t: &'a str) -> BagGraph<'a> {
        BagGraph {
            text: t,
            bagmap: HashMap::new(),
        }
    }

    fn get_rule(&'a mut self, name: &'a str) -> &'a mut BagRule {
        if !self.bagmap.contains_key(name) {
            let mut br = BagRule {name: name, contains: HashMap::new(), contained_by: Vec::new()};
            self.bagmap.insert(name, br);
        }
        return self.bagmap.get_mut(name).unwrap();
    }

    fn parse(&'a mut self) -> BagGraph<'a> {
        for line in self.text.lines() {
            let mut br = BagRule::new(line);
            for key in br.contains.keys() {
                if !self.bagmap.contains_key(key) {
                    let mut br = BagRule {name: key, contains: HashMap::new(), contained_by: Vec::new()};
                    self.bagmap.insert(key, br);
                }
                self.bagmap.get_mut(key).unwrap().contained_by.push(key);
            }
            self.bagmap.insert(&br.name, br);
        }
        return *self;
    }

}


/*
fn bag_map_search<'a>(bagmap: &'a mut BagMap, name: &str) -> Vec<&'a str> {
    let mut visited = collections::HashSet::<&str>::new();
    let mut checklist = vec![name];
    while !checklist.is_empty() {
        let name = checklist.pop().unwrap();
        if visited.contains(name) { continue; }
        visited.insert(name);
        let br = bag_map_get_rule(bagmap, name);
        for other in br.contained_by.iter() {
            if visited.contains(other) { continue; }
            checklist.push(other);
        }
    }

    return visited.into_iter().collect();
}
*/

impl<'a> BagRule<'a> {
    fn new(text: &'a str) -> BagRule {
        let parts: Vec<&str> = text.split(" bags contain ").collect();
        let name = parts[0];
        let mut br = BagRule { name: name, contains: HashMap::new(),
            contained_by: Vec::new(),
        };
        let re_bagdesc = regex::Regex::new(r"^(\d+) (\S+\s+\S+) bag").unwrap();
        if !(parts[1] == "no other bags.") {
          for d in parts[1].split(", ") {
            let caps = re_bagdesc.captures(d).unwrap();
            let count: u32 = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let bagtype = caps.get(2).unwrap().as_str();
            br.contains.insert(bagtype, count);
          }
        }

        return br;
    }
}

fn main() {
    let content = advent::load_input();
    let mut graph = BagGraph::new(&content);
    graph = graph.parse();
    dbg!(&graph);

    let mut part1_result = 0;
    let mut part2_result = 0;

    dbg!(part1_result);
    dbg!(part2_result);
}
