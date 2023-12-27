use barrel::types::char;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Debug;

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

#[derive(Debug, Default)]
struct Value(u32, String, u32);

enum CharIter<'a> {
    Forward(std::str::Chars<'a>),
    Backward(std::iter::Rev<std::str::Chars<'a>>),
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CharIter::Forward(iter) => iter.next(),
            CharIter::Backward(iter) => iter.next(),
        }
    }
}

fn find_number(char_iter: CharIter, start_at: usize) -> Value {
    let mut data = Vec::with_capacity(5);
    let is_reverse = match char_iter {
        CharIter::Forward(_) => false,
        CharIter::Backward(_) => true,
    };

    for (idx, char) in char_iter.enumerate() {
        if char.is_digit(10) {
            return Value {
                0: char.to_digit(10).unwrap(),
                1: char.to_string(),
                2: if !is_reverse { idx } else { start_at - idx } as u32,
            };
        }

        if is_reverse {
            data.insert(0, char);
        } else {
            data.push(char);
        }

        let string = data.iter().collect::<String>();

        let mut has_match = false;
        for key in NUMBERS.keys() {
            if !is_reverse && key.starts_with(string.as_str())
                || is_reverse && key.ends_with(string.as_str())
            {
                has_match = true;
                break;
            };
        }

        if !has_match {
            data.clear();
            data.push(char);
            continue;
        };

        if let Some(value) = NUMBERS.get(string.as_str()) {
            return Value {
                0: *value,
                1: string,
                2: if !is_reverse {
                    idx
                } else {
                    start_at - idx + data.len() - 1
                } as u32,
            };
        }
    }
    Value::default()
}

fn main() {
    let mut num: u32 = 0;
    let documents: &str = "one23five7
eighxthree
2asix9
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
poarsche
null1null

";
    for line in documents.lines() {
        if line.len() == 0 {
            continue;
        }

        let first = find_number(CharIter::Forward(line.chars()), 0);
        let last = find_number(CharIter::Backward(line.chars().rev()), line.len() - 1);
        if first.2 == last.2 {
            num += first.0;
        } else {
            num += first.0 + last.0;
        }
        println!("Loop {} {:?} {:?} = {}", line, first, last, num);
    }
}
