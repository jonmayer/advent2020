use advent;

struct SurveySet {
    answer_count: [u32; 26],
    entries: u32
}

impl SurveySet {
    fn new() -> SurveySet {
        SurveySet { answer_count: [0; 26], entries: 0 }
    }

    fn read_record(&mut self, record: &str) {
        for c in record.chars() {
            if c == '\n' { self.entries += 1; }
            let index = (c as usize).wrapping_sub('a' as usize);
            if index <= 26 {self.answer_count[index] += 1;}
        }
        self.entries += 1;
    }

    fn count_nonzero_answers(&self) -> u32 {
        return self.answer_count.iter().filter(|&&x| x > 0).count() as u32;
    }

    fn count_allyes_answers(&self) -> u32 {
        return self.answer_count.iter()
            .filter(|&&x| x == self.entries).count() as u32;
    }
}

fn main() {
    let content = advent::load_input();

    let records: Vec<&str> = content.split("\n\n").collect();

    {
        println!("part 1");
        let mut total = 0;
        for r in records.iter() {
            let mut ss: SurveySet = SurveySet::new();
            ss.read_record(r);
            total += dbg!(ss.count_nonzero_answers());
        }
        dbg!(total);
    }

    {
        println!("part 2");
        let mut total = 0;
        for r in records.iter() {
            let mut ss: SurveySet = SurveySet::new();
            ss.read_record(r);
            total += dbg!(ss.count_allyes_answers());
        }
        dbg!(total);
    }
}
