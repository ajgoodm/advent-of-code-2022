use shared::input::AocBufReader;

enum JetDirection {
    Left,
    Right,
}

struct JetIterator {
    _string: String,
    _len: usize,
    _idx: usize,
}

impl JetIterator {
    fn new(string: String) -> JetIterator {
        let _len = string.len();
        let _idx = 0;
        JetIterator {
            _string: string,
            _len,
            _idx,
        }
    }

    fn next(&mut self) -> JetDirection {
        let mut jet_direction: JetDirection;
        match self._string.chars().nth(self._idx).unwrap() {
            '>' => {
                jet_direction = JetDirection::Right;
            }
            '<' => {
                jet_direction = JetDirection::Left;
            }
            _ => {
                panic!("unknown character in jet string");
            }
        };
        self._idx += 1;
        if self._idx == self._len {
            self._idx = 0;
        }

        jet_direction
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RockShape {
    Horizontal,
    Plus,
    RightAngle,
    Vertical,
    Square,
}

struct RockIterator {
    _rocks: [RockShape; 5],
    _len: usize,
    _idx: usize,
}

impl RockIterator {
    fn new() -> RockIterator {
        let _rocks: [RockShape; 5] = [
            RockShape::Horizontal,
            RockShape::Plus,
            RockShape::RightAngle,
            RockShape::Vertical,
            RockShape::Square,
        ];
        let _len: usize = 5;
        let _idx: usize = 0;
        RockIterator { _rocks, _len, _idx }
    }

    fn next(&mut self) -> RockShape {
        let return_value = self._rocks[self._idx].clone();
        self._idx += 1;
        if self._idx == self._len {
            self._idx = 0;
        }
        return_value
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jet_iterator() {
        let mut reader = AocBufReader::from_string("inputs/example.txt");
        let mut jet_iterator = JetIterator::new(reader.next().unwrap());
        for _ in 0..1000 {
            jet_iterator.next();
        }
    }

    #[test]
    fn test_rock_iterator() {
        let mut rock_iterator = RockIterator::new();
        for _ in 0..1000 {
            rock_iterator.next();
        }
    }
}
