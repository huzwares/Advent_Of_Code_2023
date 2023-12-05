use regex::Regex;
use std::fs;

fn main() {
    // part 1
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().enumerate().collect::<Vec<_>>();
    let mut part_1 = 0;
    let re = Regex::new(r"[0-9]+").unwrap();
    let not_symbol = ['.', '\n', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let numbers = lines
        .iter()
        .map(|(i, l)| (i, re.find_iter(l)))
        .collect::<Vec<_>>();
    for (&i, m) in numbers {
        for matched in m.into_iter() {
            let begin_at_zero = if matched.start() == 0 { 0 } else { 1 };
            let finish_at_end = if matched.end() == lines[i].1.len() {
                0
            } else {
                1
            };
            if (!not_symbol.contains(
                &lines[i]
                    .1
                    .chars()
                    .nth(matched.start() - begin_at_zero)
                    .unwrap(),
            )) || (!not_symbol.contains(
                &lines[i]
                    .1
                    .chars()
                    .nth(matched.end() + finish_at_end - 1)
                    .unwrap(),
            )) || (i != 0
                && lines[i - 1]
                    .1
                    .chars()
                    .skip(matched.start() - begin_at_zero)
                    .take(matched.len() + begin_at_zero + finish_at_end)
                    .filter(|c| !c.is_ascii_digit())
                    .any(|c| !not_symbol.contains(&c)))
                || (i != lines.len() - 1
                    && lines[i + 1]
                        .1
                        .chars()
                        .skip(matched.start() - begin_at_zero)
                        .take(matched.len() + begin_at_zero + finish_at_end)
                        .filter(|c| !c.is_ascii_digit())
                        .any(|c| !not_symbol.contains(&c)))
            {
                part_1 += matched.as_str().parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", part_1);

    // part 2
    let mut part_2 = 0;
    let all_gears = lines
        .clone()
        .into_iter()
        .map(|(line_number, line)| {
            (
                line_number,
                line.trim()
                    .chars()
                    .enumerate()
                    .filter(|(_i, c)| c == &'*')
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    for (line_number, g) in all_gears {
        for (index, _g) in g {
            let mut adj = vec![];

            if index != 0
                && lines[line_number]
                    .1
                    .chars()
                    .nth(index - 1)
                    .unwrap()
                    .is_ascii_digit()
            {
                let tmp = re.find_iter(lines[line_number].1);
                let nmb = tmp
                    .into_iter()
                    .filter(|m| m.end() == index)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>()[0]
                    .parse::<i128>()
                    .unwrap();
                adj.push(nmb);
            }
            if index != 140
                && lines[line_number]
                    .1
                    .chars()
                    .nth(index + 1)
                    .unwrap()
                    .is_ascii_digit()
            {
                let tmp = re.find_iter(lines[line_number].1);
                let nmb = tmp
                    .into_iter()
                    .filter(|m| m.start() == index + 1)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>()[0]
                    .parse::<i128>()
                    .unwrap();
                adj.push(nmb);
            }

            if line_number != 0 {
                if lines[line_number - 1]
                    .1
                    .chars()
                    .nth(index)
                    .unwrap()
                    .is_ascii_digit()
                {
                    let tmp = re.find_iter(lines[line_number - 1].1);

                    let nmb = tmp
                        .into_iter()
                        .filter(|m| index <= m.end() && index >= m.start())
                        .map(|m| m.as_str())
                        .collect::<Vec<_>>()[0]
                        .parse::<i128>()
                        .unwrap();
                    adj.push(nmb);
                } else {
                    if index != 0
                        && lines[line_number - 1]
                            .1
                            .chars()
                            .nth(index - 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        let tmp = re.find_iter(lines[line_number - 1].1);
                        let nmb = tmp
                            .into_iter()
                            .filter(|m| m.end() == index)
                            .map(|m| m.as_str())
                            .collect::<Vec<_>>()[0]
                            .parse::<i128>()
                            .unwrap();
                        adj.push(nmb);
                    }

                    if index != 140
                        && lines[line_number - 1]
                            .1
                            .chars()
                            .nth(index + 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        let tmp = re.find_iter(lines[line_number - 1].1);
                        let nmb = tmp
                            .into_iter()
                            .filter(|m| m.start() == index + 1)
                            .map(|m| m.as_str())
                            .collect::<Vec<_>>()[0]
                            .parse::<i128>()
                            .unwrap();
                        adj.push(nmb);
                    }
                }
            }

            if line_number != lines.len() - 1 {
                if lines[line_number + 1]
                    .1
                    .chars()
                    .nth(index)
                    .unwrap()
                    .is_ascii_digit()
                {
                    let tmp = re.find_iter(lines[line_number + 1].1);
                    let nmb = tmp
                        .into_iter()
                        .filter(|m| index <= m.end() && index >= m.start())
                        .map(|m| m.as_str())
                        .collect::<Vec<_>>()[0]
                        .parse::<i128>()
                        .unwrap();
                    adj.push(nmb);
                } else {
                    if index != 0
                        && lines[line_number + 1]
                            .1
                            .chars()
                            .nth(index - 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        let tmp = re.find_iter(lines[line_number + 1].1);
                        let nmb = tmp
                            .into_iter()
                            .filter(|m| m.end() == index)
                            .map(|m| m.as_str())
                            .collect::<Vec<_>>()[0]
                            .parse::<i128>()
                            .unwrap();
                        adj.push(nmb);
                    }

                    if index != 140
                        && lines[line_number + 1]
                            .1
                            .chars()
                            .nth(index + 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        let tmp = re.find_iter(lines[line_number + 1].1);
                        let nmb = tmp
                            .into_iter()
                            .filter(|m| m.start() == index + 1)
                            .map(|m| m.as_str())
                            .collect::<Vec<_>>()[0]
                            .parse::<i128>()
                            .unwrap();
                        adj.push(nmb);
                    }
                }
            }

            if adj.len() == 2 {
                part_2 += adj[0] * adj[1];
            }
        }
    }

    println!("{part_2}");
}
