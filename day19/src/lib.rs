use std::collections::HashMap;

#[derive(Debug)]
struct Sequence {
    rules: Vec<usize>,
}

impl Sequence {
    fn parse(text: &str) -> Sequence {
        let text = text.trim();
        Sequence {
            rules: text
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Char(u8),
    Alternates(Vec<Sequence>), // 3 5 | 5 3
}

impl Rule {
    fn parse(ruletext: &str) -> Rule {
        if ruletext.starts_with("\"") {
            return Rule::Char(ruletext.bytes().nth(1).unwrap());
        } else {
            let mut alts: Vec<Sequence> = Vec::new();
            for seqs in ruletext.split(" | ") {
                alts.push(Sequence::parse(seqs));
            }
            // alts.sort_by(|a, b| a.len().cmp(&b.len()));
            return Rule::Alternates(alts);
        }
    }
}

// Chooser is used to exhaustively search decision space.   "state" is source
// that provides up to 128 binary choices.  "count" keeps track of how many
// decisions the Chooser is being asked to make on any given run, so that we
// can abort early when the decision space is exhausted.
struct Chooser {
    state: u128,
    count: u32,
}

impl Chooser {
    fn new(i: u128) -> Chooser {
        Chooser { state: i, count: 0 }
    }

    fn choose(&mut self) -> usize {
        let choice = self.state & 0x1;
        self.state >>= 1;
        self.count += 1;
        return choice as usize;
    }

    fn exhausted(&self, i: u128) -> bool {
        return i >= (1 << self.count);
    }
}

#[derive(Debug)]
pub struct RuleMap {
    m: HashMap<usize, Rule>,
    data: Vec<u8>,
}

impl RuleMap {
    pub fn new() -> RuleMap {
        RuleMap {
            m: HashMap::new(),
            data: Vec::new(),
        }
    }

    pub fn parse(&mut self, text: &str) {
        for line in text.lines() {
            let line = line.trim();
            let parts: Vec<&str> = line.split(": ").take(2).collect();
            if let [id, ruletext] = &parts[..] {
                let rule = Rule::parse(ruletext);
                self.m.insert(id.parse::<usize>().unwrap(), rule);
            }
        }
    }

    // returns the number of bytes consumed, or 0 if no match was possible.
    fn try_sequence(&self, index: usize, s: &Sequence, chooser: &mut Chooser) -> usize {
        let mut consumed = 0usize;
        for id in &(s.rules) {
            let got = self.try_rule(index + consumed, *id, chooser);
            if got == 0 {
                consumed = 0;
                break;
            }
            consumed += got;
        }
        return consumed;
    }

    // returns the number of bytes consumed, or 0 if no match was possible.
    fn try_rule(&self, index: usize, rule_id: usize, chooser: &mut Chooser) -> usize {
        if index >= self.data.len() {
            return 0;
        }
        let rule = &(self.m[&rule_id]);
        let consumed: usize = match rule {
            Rule::Char(x) => {
                if self.data[index] == *x {
                    1
                } else {
                    0
                }
            }
            Rule::Alternates(alts) => {
                let mut got = 0; // no match
                let gots: Vec<usize> = alts
                    .iter()
                    .map(|a| self.try_sequence(index, a, chooser))
                    .filter(|g| *g > 0)
                    .collect();
                if gots.len() > 1 {
                    let select = chooser.choose();
                    got = gots[select as usize];
                } else if gots.len() == 1 {
                    got = gots[0];
                }
                got
            }
        };
        return consumed;
    }

    pub fn try_match(&mut self, text: &str) -> bool {
        let text = text.trim();
        println!("Trying to match {:?}", text);
        self.data = text.as_bytes().iter().map(|&x| x as u8).collect();
        // A poor man's backtracker: every time the algorithm has to
        // make a binary choice between two options that both match,
        // it pulls a bit from chooser to select between them.  By
        // iterating through all possible pairs of decisions, I can
        // exhaustively search the decision space.
        let mut i: u128 = 0;
        loop {
            let mut chooser: Chooser = Chooser::new(i);
            let consumed = self.try_rule(0, 0, &mut chooser);
            if consumed == self.data.len() {
                return true;
            }
            i = i + 1;
            // if i is larger that the set of possible choices the
            // chooser was asked to make, terminate early.  My
            // previous code would try 1024 different chooser seeds,
            // this is much faster (typically exhausts decision space
            // after 8 tries).
            if chooser.exhausted(i) {
                println!("Gave up after {} tries ({} choices)", i, chooser.count);
                break;
            }
        }
        return false;
    }
}

pub fn count_matches(ruletext: &str, datatext: &str, part2: bool) -> usize {
    let mut rulemap = RuleMap::new();
    rulemap.parse(ruletext);
    if part2 {
        // Override 2 rules to introduce loops:
        rulemap.parse("8: 42 | 42 8\n11: 42 31 | 42 11 31");
    }

    let mut count = 0;
    for line in datatext.lines() {
        let valid = rulemap.try_match(line);
        println!("{} for {:?}", valid, line);
        if valid {
            count += 1;
        }
    }
    return count;
}
