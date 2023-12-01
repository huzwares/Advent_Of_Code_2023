use std::fs;

fn main() {
    let sppeled_digit = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut sp_d = sppeled_digit
            .iter()
            .enumerate()
            .filter(|(_i, &sn)| line.find(sn).is_some())
            .map(|(_i, &sn)| (line.find(sn).unwrap(), sn.to_string()))
            .collect::<Vec<_>>();

        sp_d.append(
            &mut sppeled_digit
                .iter()
                .enumerate()
                .filter(|(_i, &sn)| line.rfind(sn).is_some())
                .map(|(_i, &sn)| (line.rfind(sn).unwrap(), sn.to_string()))
                .collect::<Vec<_>>(),
        );
        sp_d.sort();
        sp_d.dedup();
        let mut numbers = line
            .chars()
            .enumerate()
            .filter(|(_i, c)| c.is_ascii_digit())
            .map(|(i, c)| (i, c.to_string()))
            .collect::<Vec<_>>();
        numbers.append(&mut sp_d);
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        let b = numbers
            .iter()
            .map(|(_i, n)| match n.as_str() {
                "zero" => "0".to_string(),
                "one" => "1".to_string(),
                "two" => "2".to_string(),
                "three" => "3".to_string(),
                "four" => "4".to_string(),
                "five" => "5".to_string(),
                "six" => "6".to_string(),
                "seven" => "7".to_string(),
                "eight" => "8".to_string(),
                "nine" => "9".to_string(),
                x => x.to_string(),
            })
            .collect::<Vec<_>>();
        let number = match b.len() {
            x if x > 1 => {
                format!("{}{}", b.first().unwrap(), b.last().unwrap())
            }
            1 => format!("{0}{0}", b.first().unwrap()),
            _ => "0".to_string(),
        };
        let number: u32 = number.parse().unwrap();
        sum += number;
    }
    println!("{}", sum);
}
