use std::cmp;
use std::mem::swap;

use shared::input::AocBufReader;

#[derive(Debug, PartialEq, Eq)]
enum ObjectType {
    LIST,
    ELEMENT,
}

#[derive(Debug)]
struct Object {
    object_type: ObjectType,
    element_value: usize,
    list: Vec<Object>,
}

#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    ORDERED,
    EQUAL,
    OUT_OF_ORDER,
}

impl Object {
    fn len(&self) -> usize {
        self.list.len()
    }

    fn as_string(&self) -> String {
        if self.object_type == ObjectType::ELEMENT {
            return self.element_value.to_string();
        } else {
            return format!(
                "[{}]",
                self.list
                    .iter()
                    .map(|x| Object::as_string(x))
                    .collect::<Vec<String>>()
                    .join(",")
            );
        }
    }

    fn new_val(element_value: usize) -> Object {
        Object {
            object_type: ObjectType::ELEMENT,
            element_value,
            list: vec![],
        }
    }

    fn empty_list() -> Object {
        Object {
            object_type: ObjectType::LIST,
            element_value: 0,
            list: vec![],
        }
    }

    fn list_from_val(element_value: usize) -> Object {
        Object {
            object_type: ObjectType::LIST,
            element_value,
            list: vec![Object::new_val(element_value)],
        }
    }

    fn is_ordered(left: &Object, right: &Object) -> Comparison {
        if left.object_type == ObjectType::LIST && right.object_type == ObjectType::LIST {
            for idx in 0..cmp::min(left.len(), right.len()) {
                match Object::is_ordered(&left.list[idx], &right.list[idx]) {
                    Comparison::ORDERED => return Comparison::ORDERED,
                    Comparison::EQUAL => (),
                    Comparison::OUT_OF_ORDER => return Comparison::OUT_OF_ORDER,
                }
            }
            if left.len() < right.len() {
                return Comparison::ORDERED;
            } else if left.len() > right.len() {
                return Comparison::OUT_OF_ORDER;
            } else {
                return Comparison::EQUAL;
            }
        } else if left.object_type == ObjectType::ELEMENT
            && right.object_type == ObjectType::ELEMENT
        {
            if left.element_value < right.element_value {
                return Comparison::ORDERED;
            } else if left.element_value > right.element_value {
                return Comparison::OUT_OF_ORDER;
            } else {
                return Comparison::EQUAL;
            }
        } else {
            if left.object_type == ObjectType::ELEMENT {
                return Object::is_ordered(&Object::list_from_val(left.element_value), &right);
            } else {
                return Object::is_ordered(&left, &Object::list_from_val(right.element_value));
            }
        }
    }
}

struct PacketPair {
    left: Object,
    right: Object,
}

impl PacketPair {
    fn is_ordered(&self) -> bool {
        match Object::is_ordered(&self.left, &self.right) {
            Comparison::ORDERED => true,
            Comparison::OUT_OF_ORDER => false,
            Comparison::EQUAL => panic!("Don't know what to do if packet pair is equal"),
        }
    }
}

/// start_idx is the index of an opening square bracket.
/// Finds the index of the corresponding closing square bracket.
fn _get_end_idx(packet_str: &String, mut start_idx: usize) -> usize {
    let mut n_left_brackets: usize = 1;
    let mut n_right_brackets: usize = 0;
    loop {
        start_idx += 1;
        match packet_str.chars().nth(start_idx).unwrap() {
            '[' => n_left_brackets += 1,
            ']' => n_right_brackets += 1,
            _ => (),
        }
        if n_left_brackets == n_right_brackets {
            break;
        }
    }
    start_idx
}

fn parse_packet(packet: String) -> Object {
    if packet == "[]".to_string() {
        return Object {
            object_type: ObjectType::LIST,
            element_value: 0,
            list: vec![],
        };
    }

    let mut list: Vec<Object> = Vec::new();
    let mut current_number: String = "".to_string();

    // the first character is alwas '['
    let mut cursor_idx = 1;
    loop {
        let c: char = packet.chars().nth(cursor_idx).unwrap();
        if c.is_ascii_digit() {
            current_number.push(c);
        } else if c == ',' {
            list.push(Object {
                object_type: ObjectType::ELEMENT,
                element_value: current_number.parse::<usize>().unwrap(),
                list: vec![],
            });
            current_number = "".to_string();
        } else if c == '[' {
            let end_idx: usize = _get_end_idx(&packet, cursor_idx);
            if end_idx == cursor_idx + 1 {
                list.push(Object::empty_list());
            } else {
                // recurse!
                list.push(parse_packet(packet[cursor_idx..end_idx + 1].to_string()));
            }
            cursor_idx = end_idx + 1;
        } else {
            panic!("unexpected character {}", c);
        }
        cursor_idx += 1;
        if cursor_idx >= packet.len() - 1 {
            break;
        }
    }
    if current_number != "".to_string() {
        list.push(Object {
            object_type: ObjectType::ELEMENT,
            element_value: current_number.parse::<usize>().unwrap(),
            list: vec![],
        });
    }
    Object {
        object_type: ObjectType::LIST,
        element_value: 0,
        list,
    }
}

fn parse_packet_pair(packet_1: String, packet_2: String) -> PacketPair {
    PacketPair {
        left: parse_packet(packet_1),
        right: parse_packet(packet_2),
    }
}

fn part_1(mut reader: AocBufReader) -> usize {
    let mut packet_pair_idx: usize = 1;
    let mut running_sum: usize = 0;
    loop {
        if let Some(line_1) = reader.next() {
            let line_2 = reader.next().unwrap();
            let _empty_line = reader.next();
            let packet_pair = parse_packet_pair(line_1, line_2);
            if packet_pair.is_ordered() {
                running_sum += packet_pair_idx;
            }
            packet_pair_idx += 1;
        } else {
            break;
        }
    }

    running_sum
}

fn bubble_sort(packets: &mut Vec<Object>) {
    let mut swap_occurred = true;
    while swap_occurred {
        swap_occurred = false;
        for idx in 0..(packets.len() - 1) {
            let is_ordered: bool = match Object::is_ordered(&packets[idx], &packets[idx + 1]) {
                Comparison::ORDERED => true,
                Comparison::OUT_OF_ORDER => false,
                Comparison::EQUAL => panic!("Something went wrong in bubble sort"),
            };

            if !is_ordered {
                unsafe {
                    let pa: *mut Object = &mut packets[idx];
                    let pb: *mut Object = &mut packets[idx + 1];
                    std::ptr::swap(pa, pb);
                }
                swap_occurred = true;
            }
        }
    }
}

fn part_2(reader: AocBufReader) -> usize {
    let divider_packet_string_1 = "[[2]]".to_string();
    let divider_packet_string_2 = "[[6]]".to_string();

    let mut packets: Vec<Object> = reader
        .filter(|line| line != "")
        .map(|line| parse_packet(line))
        .collect();
    packets.push(parse_packet(divider_packet_string_1.clone()));
    packets.push(parse_packet(divider_packet_string_2.clone()));
    bubble_sort(&mut packets);

    let mut packet_1_idx: usize = 0;
    let mut packet_2_idx: usize = 0;
    for (idx, packet) in packets.iter().enumerate() {
        if packet.as_string() == divider_packet_string_1 {
            packet_1_idx = idx + 1;
        }
        if packet.as_string() == divider_packet_string_2 {
            packet_2_idx = idx + 1;
        }
    }

    packet_1_idx * packet_2_idx
}

fn main() {
    //not 5938
    println!("{}", part_1(AocBufReader::from_string("inputs/part_1.txt")));
    println!("{}", part_2(AocBufReader::from_string("inputs/part_1.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let empty_packet = parse_packet("[]".to_string());
        assert_eq!(empty_packet.object_type, ObjectType::LIST);
        assert_eq!(empty_packet.list.len(), 0);

        parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string());
    }

    #[test]
    fn test_as_string() {
        let packet = parse_packet("[[1],[2,3,4]]".to_string());
        assert_eq!(packet.as_string(), "[[1],[2,3,4]]".to_string());
    }

    #[test]
    fn test_is_ordered() {
        assert_eq!(
            Object::is_ordered(
                &parse_packet("[1,1,3,1,1]".to_string()),
                &parse_packet("[1,1,5,1,1]".to_string())
            ),
            Comparison::ORDERED
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[[1],[2,3,4]]".to_string()),
                &parse_packet("[[1],4]".to_string())
            ),
            Comparison::ORDERED
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[9]".to_string()),
                &parse_packet("[[8,7,6]]".to_string())
            ),
            Comparison::OUT_OF_ORDER
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[[4,4],4,4]".to_string()),
                &parse_packet("[[4,4],4,4,4]".to_string())
            ),
            Comparison::ORDERED
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[]".to_string()),
                &parse_packet("[3]".to_string())
            ),
            Comparison::ORDERED
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[[[]]]".to_string()),
                &parse_packet("[[]]".to_string())
            ),
            Comparison::OUT_OF_ORDER
        );

        assert_eq!(
            Object::is_ordered(
                &parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string()),
                &parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string())
            ),
            Comparison::OUT_OF_ORDER
        );
    }

    #[test]
    fn test_parse_bug_case() {
        let s = "[[],[],[[2],[[],[7,10,0],2],[[6],3]],[8,10,[4]],[2,0,2,[2,[2,0],4,[1,2,7,4]],7]]"
            .to_string();
        assert_eq!(parse_packet(s.clone()).as_string(), s);
    }

    #[test]
    fn test_multi_digit() {
        Object::is_ordered(
            &parse_packet(
                "[[],[],[[2],[[],[7,10,0],2],[[6],3]],[8,10,[4]],[2,0,2,[2,[2,0],4,[1,2,7,4]],7]]"
                    .to_string(),
            ),
            &parse_packet(
                "[[4,[[5,10,9,10,8],[0,9,0,8],4,1,10],[[0,3],[3,5],10]],[8],[],[7]]".to_string(),
            ),
        );
    }
}
