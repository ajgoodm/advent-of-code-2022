use std::collections::HashSet;

use shared::input::AocBufReader;

fn get_message_start_idx(message: &String, n_distinct: usize) -> usize {
    let message_len: usize = message.len();
    for char_idx in n_distinct..message_len {
        if message[char_idx - n_distinct..char_idx]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == n_distinct
        {
            return char_idx;
        }
    }
    panic!("Something wrong with the message!");
}

fn main() {
    let input = AocBufReader::from_string("inputs/part_1.txt")
        .next()
        .unwrap();
    println!("{}", get_message_start_idx(&input, 4));
    println!("{}", get_message_start_idx(&input, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_start_idx() {
        assert_eq!(
            get_message_start_idx(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
            11usize
        );
        assert_eq!(
            get_message_start_idx(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
            26usize
        );
    }
}
