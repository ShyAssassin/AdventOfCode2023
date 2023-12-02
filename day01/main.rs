const INPUT: &str = include_str!("input.txt");
static NUMBER_MAPPING: [(&'static str, u32); 9] = [
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

fn part1() -> i32 {
    let numbers: Vec<i32> = INPUT
        // Split the input into lines
        .lines()
        // Filter out characters that are not digits
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap()
        })
        // Combine first and last digit
        .map(|num: String| {
            let last = num.chars().last().unwrap();
            let first = num.chars().next().unwrap();
            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .collect();
    let sum = numbers.iter().sum::<i32>();
    return sum;
}

fn part2() -> u32 {
    let numbers: Vec<u32> = INPUT
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut nums = vec![];

            for i in 0..bytes.len() {
                if bytes[i].is_ascii_digit() {
                    nums.push((bytes[i] - b'0') as u32);
                    continue;
                }
                for (target, numeric) in &NUMBER_MAPPING {
                    if (&bytes[i..]).starts_with(&target.as_bytes()) {
                        nums.push(*numeric);
                        break;
                    }
                }
            }
            let first = nums.first().unwrap() * 10;
            let last = nums.last().unwrap();
            first + last
        })
        .collect();
    let sum = numbers.iter().sum::<u32>();
    return sum;
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
