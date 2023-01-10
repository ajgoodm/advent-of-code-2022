fn snafu_to_decimal(s: String) -> isize {
    s.chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| match c {
            '2' => 2 * 5isize.pow(idx.try_into().unwrap()),
            '1' => 5isize.pow(idx.try_into().unwrap()),
            '0' => 0,
            '-' => -1 * 5isize.pow(idx.try_into().unwrap()),
            '=' => -2 * 5isize.pow(idx.try_into().unwrap()),
            _ => panic!("unexpected char {}", c),
        })
        .sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=".to_string()), 3isize);
        assert_eq!(snafu_to_decimal("1=11-2".to_string()), 2022isize);
        assert_eq!(
            snafu_to_decimal("1121-1110-1=0".to_string()),
            314159265isize
        );
    }
}
