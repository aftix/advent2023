use std::io;

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().map_while(Result::ok).collect();
    let input: Vec<_> = lines.iter().map(String::as_str).collect();
    println!("Sum is {}", advent2023::day3p2(&input));
}
