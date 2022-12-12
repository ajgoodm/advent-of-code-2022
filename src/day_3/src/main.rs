use std::collections::HashSet;

use shared::conversion::char_to_usize;
use shared::input::AocBufReader;

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", rucksack_priority_pt_1(reader));

    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("{}", rucksack_badges_pt_2(reader));
}

fn shared_character(items: String) -> char {
    let n_items: usize = items.len() / 2;

    *items[..n_items]
        .chars()
        .collect::<HashSet<char>>()
        .intersection(&items[n_items..].chars().collect::<HashSet<char>>())
        .into_iter()
        .next()
        .unwrap()
}

fn rucksack_priority_pt_1(reader: AocBufReader) -> usize {
    reader
        .into_iter()
        .map(|string| char_to_usize(shared_character(string)))
        .sum()
}

fn rucksack_badges_pt_2(mut reader: AocBufReader) -> usize {
    let mut elf_1: String;
    let mut elf_2: String;
    let mut elf_3: String;

    let mut badge_sum: usize = 0;

    while let Some(elf_1) = reader.next() {
        elf_2 = reader.next().unwrap();
        elf_3 = reader.next().unwrap();

        badge_sum += char_to_usize(
            *elf_1
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&elf_2.chars().collect::<HashSet<char>>())
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&elf_3.chars().collect::<HashSet<char>>())
                .into_iter()
                .next()
                .unwrap(),
        );
    }

    badge_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shared_character() {
        assert_eq!(
            shared_character("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()),
            'p'
        );
        assert_eq!(
            shared_character("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()),
            'L'
        );
    }

    #[test]
    fn test_char_to_usize() {
        assert_eq!(char_to_usize('A'), 27);
        assert_eq!(char_to_usize('b'), 2);
    }
}
