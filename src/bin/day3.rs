use advent2023::{parser::day3::parse_input, types::Schematic};
use std::collections::{HashMap, HashSet};
use std::io;

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
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

    let mut already_added = HashSet::new();
    let mut get_adjacent_numbers = |idx: usize| {
        let (x, y) = linear_to_rect(idx);
        [
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
                    Schematic::Number(num, _) => {
                        let num = *num;
                        if already_added.insert(*schem) {
                            num
                        } else {
                            0
                        }
                    }
                    _ => 0,
                }
            } else {
                0
            }
        })
        .sum::<i64>()
    };

    let sum: i64 = items
        .iter()
        .filter(|schem| schem.is_symbol())
        .map(|symb| get_adjacent_numbers(symb.span().0))
        .sum();
    println!("Sum is {sum}");
}
