// Simplify Solution by gpt based on previous 1b_ori

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, u32> = [
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect();
}

#[derive(Debug, Default)]
struct Value(u32, String, u32);

fn find_number(chars: impl Iterator<Item = char>, start_at: usize, reverse: bool) -> Value {
    let mut data = Vec::with_capacity(5);

    for (idx, char) in chars.enumerate() {
        if let Some(digit) = char.to_digit(10) {
            let position = if reverse { start_at - idx } else { idx };
            return Value(digit, char.to_string(), position as u32);
        }

        if reverse {
            data.insert(0, char);
        } else {
            data.push(char);
        }

        let string: String = data.iter().collect();
        if !NUMBERS.keys().any(|&k| if reverse { k.ends_with(&string) } else { k.starts_with(&string) }) {
            data.clear();
            data.push(char);
            continue;
        }

        if let Some(&value) = NUMBERS.get(&*string) {
            let position = if reverse { start_at - idx + data.len() - 1 } else { idx };
            return Value(value, string.clone(), position as u32);
        }
    }

    Value::default()
}

fn main() {
    let mut num: u32 = 0;
    let documents = "one23five7
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
        if line.len() == 0{
            continue
        }
        let line_len = line.len() - 1;
        let first = find_number(line.chars(), 0, false);
        let last = find_number(line.chars().rev(), line_len, true);
        num += if first.2 == last.2 && first.1 == last.1 { first.0 } else { first.0 + last.0 };
        println!("Loop {} {:?} {:?} = {}", line, first, last, num);
    }
}
