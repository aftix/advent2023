use advent2023::{parser::day3::parse_input, types::Schematic};
use std::collections::HashMap;
use std::io;

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().map_while(Result::ok).collect();
    let lines: Vec<_> = lines.iter().map(String::as_str).collect();

    let (_, ((width, _), items)) = parse_input(&lines).expect("Failed to parse input");

    let linear_to_rect = |idx: usize| (idx % width, idx / width);

    let mut coordinate_map: HashMap<(usize, usize), &Schematic> = HashMap::new();
    items.iter().for_each(|schem| {
        let span = schem.span();
        for idx in span.0..span.1 {
            coordinate_map.insert(linear_to_rect(idx), schem);
        }
    });

    // Assumes that idx is the location of a '*'
    // Returns 0 in the "not a gear" case
    let get_gear_ratio = |idx: usize| {
        let (x, y) = linear_to_rect(idx);
        let mut already_found = vec![];

        let adjacent_numbers: Vec<_> = [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ]
        .iter()
        .map(|coords| {
            if let Some(&schem) = coordinate_map.get(coords) {
                match schem {
                    Schematic::Number(num, span) => {
                        let num = *num;
                        if !already_found.contains(span) {
                            already_found.push(*span);
                            Some(num)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

        if adjacent_numbers.len() != 2 {
            0
        } else {
            adjacent_numbers[0] * adjacent_numbers[1]
        }
    };

    let sum: i64 = items
        .iter()
        .filter(|schem| matches!(schem, Schematic::Symbol('*', _)))
        .map(|symb| get_gear_ratio(symb.span().0))
        .sum();
    println!("Sum is {sum}");
}
