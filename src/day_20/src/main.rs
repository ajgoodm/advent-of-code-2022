use shared::input::AocBufReader;

#[derive(Clone)]
struct Node {
    value: isize,
    id: usize,
    previous: usize,
    next: usize,
}

#[derive(Clone)]
struct LinkedRing {
    elements: Vec<Node>,
}

impl LinkedRing {
    fn len(&self) -> usize {
        self.elements.len()
    }

    fn get(&self, id: usize) -> &Node {
        self.elements.get(id).unwrap()
    }

    fn _push(&mut self, id: usize) {
        let mut subject = self.get(id).clone();
        let mut next = self.get(subject.next).clone();
        let mut previous = self.get(subject.previous).clone();
        let mut next_next = self.get(next.next).clone();

        let next_next_id = next_next.id;
        let subject_previous = previous.id;

        subject.previous = next.id;
        subject.next = next_next_id;

        next.previous = subject_previous;
        next.next = subject.id;

        previous.next = next.id;
        next_next.previous = subject.id;

        self.elements[subject.id] = subject.clone();
        self.elements[next.id] = next.clone();
        self.elements[previous.id] = previous.clone();
        self.elements[next_next.id] = next_next.clone();
    }

    fn push_n(&mut self, id: usize, n: usize) {
        for _ in 0..n {
            self._push(id);
        }
    }

    fn _pull(&mut self, id: usize) {
        let mut subject = self.get(id).clone();
        let mut previous = self.get(subject.previous).clone();
        let mut next = self.get(subject.next).clone();
        let mut previous_previous = self.get(previous.previous).clone();

        let previous_previous_id = previous.previous;
        let subject_next = subject.next;

        subject.previous = previous_previous_id;
        subject.next = previous.id;

        previous.previous = subject.id;
        previous.next = subject_next;

        next.previous = previous.id;
        previous_previous.next = subject.id;

        self.elements[subject.id] = subject.clone();
        self.elements[previous.id] = previous.clone();
        self.elements[next.id] = next.clone();
        self.elements[previous_previous.id] = previous_previous.clone();
    }

    fn pull_n(&mut self, id: usize, n: usize) {
        for _ in 0..n {
            self._pull(id);
        }
    }

    fn get_nth_value_after_zero(&self, n: usize) -> isize {
        let zero_elements = self
            .elements
            .iter()
            .filter(|e| e.value == 0)
            .collect::<Vec<&Node>>();
        assert_eq!(zero_elements.len(), 1);

        let mut cursor = zero_elements[0].id;
        for _ in 0..n {
            cursor = self.get_next_element_id(cursor);
        }
        self.elements.get(cursor).unwrap().value
    }

    fn get_next_element_id(&self, id: usize) -> usize {
        self.get(id).next
    }
}

fn parse_input_pt_1(reader: AocBufReader) -> LinkedRing {
    let values: Vec<isize> = reader
        .into_iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect();
    let n_values = values.len();
    let mut elements: Vec<Node> = vec![];
    for (idx, val) in values.into_iter().enumerate() {
        let previous: usize;
        if idx == 0 {
            previous = n_values - 1;
        } else {
            previous = idx - 1;
        }

        let next: usize;
        if idx == n_values - 1 {
            next = 0;
        } else {
            next = idx + 1;
        }

        elements.push(Node {
            value: val,
            id: idx,
            previous,
            next,
        });
    }
    LinkedRing { elements }
}

fn part_1(reader: AocBufReader) -> isize {
    let mut linear_ring = parse_input_pt_1(reader);
    for id in 0..linear_ring.len() {
        let value = linear_ring.elements.get(id).unwrap().value;
        let n_moves = value.abs() as usize;
        if value < 0 {
            linear_ring.pull_n(id, n_moves);
        } else if value > 0 {
            linear_ring.push_n(id, n_moves);
        }
    }

    linear_ring.get_nth_value_after_zero(1_000)
        + linear_ring.get_nth_value_after_zero(2_000)
        + linear_ring.get_nth_value_after_zero(3_000)
}

fn main() {
    // too low: 12962
    println!(
        "part_1: {}",
        part_1(AocBufReader::from_string("inputs/part_1.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_pt_1() {
        let mut ring = parse_input_pt_1(AocBufReader::from_string("inputs/example.txt"));
        ring.push_n(0, 1);

        let mut cursor = 0;
        for _ in 0..ring.len() + 1 {
            println!("{}", ring.get(cursor).value);
            cursor = ring.get_next_element_id(cursor);
        }
    }
}
