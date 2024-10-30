use std::{
    fs::File,
    i32,
    io::{BufRead, BufReader},
    path::Path,
};

const RADIX: u32 = 10;

pub fn run() {
    let lines = lines_from_file("trebuchet.txt");
    let mut line_numbers: Vec<u32> = Vec::new();

    for line in lines {
        let line_str = line.as_str();
        let mut first: i32 = -1;
        let mut last: u32 = 0;

        let mut i = 0;
        while i < line_str.len() {
            if let Some(number) = starts_with_number(&line_str[i..]) {
                if first == -1 {
                    first = number as i32;
                }

                last = number;
            }

            i += 1;
        }

        line_numbers.push((first as u32 * 10) + last);
    }

    println!("{}", line_numbers.iter().sum::<u32>());
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}

fn starts_with_number(text: &str) -> Option<u32> {
    if let Some(number) = text.chars().next().unwrap().to_digit(RADIX) {
        return Some(number);
    }

    let numbers = [
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
    ];

    for (word, number) in &numbers {
        if text.starts_with(word) {
            return Some(*number);
        }
    }

    None
}
