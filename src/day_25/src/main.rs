use shared::input::AocBufReader;

fn snafu_to_decimal(s: String) -> usize {
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
        .sum::<isize>() as usize
}

fn get_n_digits(val: usize) -> usize {
    let mut n_digits: u32 = 1;
    loop {
        if 2 * 5usize.pow(n_digits) >= val {
            break;
        }
        n_digits += 1;
    }

    // digits are 0 indexed and we want to return the length
    n_digits as usize + 1
}

fn largest_value_of_length_n(n_digits: u32) -> isize {
    (0u32..n_digits).map(|x| 2 * 5isize.pow(x)).sum()
}

fn decimal_to_snafu(val: usize) -> String {
    let n_digits = get_n_digits(val);

    let mut snafu: String = String::new();
    let mut snafu_val: isize = 0;
    for exponent in (0..n_digits).rev() {
        let exponent: u32 = exponent.try_into().unwrap();
        let largest_remainder: isize;
        if exponent == 0 {
            largest_remainder = 0;
        } else {
            largest_remainder = largest_value_of_length_n(exponent);
        }

        let difference: isize = (val as isize) - snafu_val;
        if difference >= 0 {
            // need to make snafu bigger!
            if difference > 5isize.pow(exponent) + largest_remainder {
                snafu.push('2');
                snafu_val += 2 * 5isize.pow(exponent);
            } else if difference > largest_remainder {
                snafu.push('1');
                snafu_val += 5isize.pow(exponent);
            } else {
                snafu.push('0');
            }
        } else {
            // need to make snafu smaller!
            if difference < -1 * 5isize.pow(exponent) - largest_remainder {
                snafu.push('=');
                snafu_val -= 2 * 5isize.pow(exponent);
            } else if difference < -largest_remainder {
                snafu.push('-');
                snafu_val -= 5isize.pow(exponent);
            } else {
                snafu.push('0');
            }
        }
    }

    snafu
}

fn part_1(reader: AocBufReader) -> String {
    let mut running_value: usize = 0;
    for line in reader {
        running_value += snafu_to_decimal(line);
    }

    decimal_to_snafu(running_value)
}

fn main() {
    let reader = AocBufReader::from_string("inputs/part_1.txt");
    println!("part 1: {}", part_1(reader));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=".to_string()), 3usize);
        assert_eq!(snafu_to_decimal("1=11-2".to_string()), 2022usize);
        assert_eq!(
            snafu_to_decimal("1121-1110-1=0".to_string()),
            314159265usize
        );
    }

    #[test]
    fn test_get_n_digits() {
        assert_eq!(get_n_digits(1747), 6);
        assert_eq!(get_n_digits(906), 5);
        assert_eq!(get_n_digits(3), 2);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1747), "1=-0-2".to_string());
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0".to_string());
    }
}
