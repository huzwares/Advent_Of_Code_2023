use std::fs;
fn main() {
    // part 1
    let input = fs::read_to_string("input.txt").unwrap();
    let mut part_1 = 0;
    for l in input.lines() {
        let (_, data) = l.split_at(10);
        let tmp = data.trim().split('|').collect::<Vec<_>>();
        let winning_numbers = tmp[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let scratched_numbers = tmp[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let wins = scratched_numbers
            .iter()
            .filter(|&n| winning_numbers.contains(n))
            .count();
        if wins > 0 {
            part_1 += 2_i32.pow((wins - 1) as u32);
        }
    }
    println!("{}", part_1);
}
