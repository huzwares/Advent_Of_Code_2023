use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // part 1
    let mut part_1 = 0;
    for l in input.lines() {
        let (game, info) = l.split_once(':').unwrap();
        let tmp = info.replace(';', ",");
        let cubes = tmp.split(',').collect::<Vec<&str>>();
        let mut flag = true;
        for info in cubes {
            let info = info.split_whitespace().collect::<Vec<_>>();
            let count: i32 = info[0].parse().unwrap();
            let color = info[1];
            let max = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => panic!(),
            };
            if count > max {
                flag = false;
                break;
            }
        }
        if flag {
            let game_number = game.split_whitespace().collect::<Vec<_>>();
            part_1 += game_number[1].parse::<i32>().unwrap();
        }
    }
    println!("{part_1}");

    // part 2
    let mut part_2 = 0;
    for l in input.lines() {
        let (_game, info) = l.split_once(':').unwrap();
        let tmp = info.replace(';', ",");
        let cubes = tmp.split(',').collect::<Vec<&str>>();
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for info in cubes {
            let info = info.split_whitespace().collect::<Vec<_>>();
            let count: i32 = info[0].parse().unwrap();
            let color = info[1];
            match color {
                "red" => {
                    if count > max_red {
                        max_red = count;
                    }
                }
                "green" => {
                    if count > max_green {
                        max_green = count;
                    }
                }
                "blue" => {
                    if count > max_blue {
                        max_blue = count;
                    }
                }
                _ => panic!(),
            };
        }
        part_2 += max_red * max_green * max_blue;
    }

    println!("{part_2}");
}
