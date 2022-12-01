use shared::input::AocBufReader;

fn parse_input(aoc_reader: AocBufReader) -> Vec<Vec<usize>> {
    let empty_line = "".to_string();

    let mut calories: usize;
    let mut manifests: Vec<Vec<usize>> = Vec::new();
    let mut elf_food: Vec<usize> = Vec::new();
    for line in aoc_reader {
        if line == empty_line {
            manifests.push(elf_food);
            elf_food = Vec::new();
        } else {
            calories = line.parse::<usize>().unwrap();
            elf_food.push(calories);
        }
    }

    manifests
}

fn part_1(manifests: Vec<Vec<usize>>) -> usize {
    manifests
        .iter()
        .map(|elf_food| elf_food.iter().sum())
        .max()
        .unwrap()
}

struct TopThree {
    first: usize,
    second: usize,
    third: usize,
}

impl TopThree {
    pub fn new() -> TopThree {
        TopThree {
            first: 0,
            second: 0,
            third: 0,
        }
    }

    pub fn sum(&self) -> usize {
        self.first + self.second + self.third
    }

    pub fn maybe_replace(&mut self, val: usize) {
        if val >= self.first {
            self.replace_first(val);
        } else if val >= self.second {
            self.replace_second(val);
        } else if val >= self.third {
            self.replace_third(val);
        }
    }

    fn replace_third(&mut self, val: usize) {
        self.third = val;
    }

    fn replace_second(&mut self, val: usize) {
        self.replace_third(self.second);
        self.second = val;
    }

    fn replace_first(&mut self, val: usize) {
        self.replace_second(self.first);
        self.first = val;
    }
}

fn part_2(manifests: Vec<Vec<usize>>) -> usize {
    let mut top_three = TopThree::new();
    for manifest in manifests {
        top_three.maybe_replace(manifest.into_iter().sum::<usize>())
    }
    top_three.sum()
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let manifests = parse_input(reader);
    println!("{}", part_1(manifests));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    let manifests = parse_input(reader);
    println!("{}", part_2(manifests));
}
