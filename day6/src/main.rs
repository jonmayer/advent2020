use advent;

const SURVEY_SIZE: usize = 26;

struct SurveySet {
    yes_count: [u32; SURVEY_SIZE],  // number of yes answers for 26 questions.
    answer_count: u32
}

impl SurveySet {
    fn new() -> SurveySet {
        SurveySet { yes_count: [0; SURVEY_SIZE], answer_count: 0 }
    }

    fn read_record(&mut self, record: &str) {
        for c in record.chars() {
            if c == '\n' { self.answer_count += 1; }
            let index = (c as usize).wrapping_sub('a' as usize);
            if index <= SURVEY_SIZE {self.yes_count[index] += 1;}
        }
        self.answer_count += 1;
    }

    fn count_nonzero_answers(&self) -> u32 {
        return self.yes_count.iter().filter(|&&x| x > 0).count() as u32;
    }

    fn count_allyes_answers(&self) -> u32 {
        return self.yes_count.iter()
            .filter(|&&x| x == self.answer_count).count() as u32;
    }
}

fn main() {
    let content = advent::load_input();

    let records: Vec<&str> = content.split("\n\n").collect();

    let mut part1_total = 0;
    let mut part2_total = 0;

    for r in records.iter() {
        let mut ss: SurveySet = SurveySet::new();
        ss.read_record(r);
        part1_total += dbg!(ss.count_nonzero_answers());
        part2_total += dbg!(ss.count_allyes_answers());
    }

    dbg!(part1_total);
    dbg!(part2_total);
}
