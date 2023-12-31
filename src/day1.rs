use std::fs;

pub fn solve() {
    let largest = fs::read_to_string("./input/day-1.txt")
        .expect("Failed to read day-1.txt file")
        .split("\n\n")
        .map(|item| {
            item.lines()
                .map(|e| {
                    e.trim()
                        .parse::<u32>()
                        .expect("Failed to parse into number")
                })
                .sum::<u32>()
        })
        .max()
        .expect("Failed to find max value");

    let mut v = fs::read_to_string("./input/day-1.txt")
        .expect("Failed to read day-1.txt file")
        .split("\n\n")
        .map(|item| {
            item.lines()
                .map(|e| {
                    e.trim()
                        .parse::<u32>()
                        .expect("Failed to parse into number")
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    v.sort_unstable();
    let total_top_3: u32 = v.into_iter().rev().take(3).sum();

    println!("---------------------------------");
    println!("day 1 puzzle-1 : {}", largest);
    println!("day 1 puzzle-2 : {}", total_top_3);
}
