#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let regex = regex::Regex::new(r"mul\(\d{1,3},\s*\d{1,3}\)").unwrap();
    let all_matches = regex.find_iter(input);

    let matches = all_matches.map(|f| f.as_str()).collect::<Vec<_>>();
    let split = matches
        .iter()
        .map(|f| {
            f.split(|a| a == ',')
                .map(|n| n.chars().filter(|c| c.is_digit(10)).collect::<String>())
                .collect::<Vec<String>>()
                .iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    for i in split {
        total += i[0] * i[1];
    }
    println!("{:?}", total);
    total as usize
}
#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let regex = regex::Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d{1,3}),\s*(\d{1,3})\))").unwrap();

    // Initial state: mul instructions are enabled
    let mut enabled = true;
    let mut result_sum = 0;

    // Iterate over matches in the input
    for cap in regex.captures_iter(input) {
        if let Some(_) = cap.get(1) {
            // do() instruction
            enabled = true;
        } else if let Some(_) = cap.get(2) {
            // don't() instruction
            enabled = false;
        } else if let Some(_) = cap.get(3) {
            // mul(x, y) instruction
            if enabled {
                let x: i32 = cap[4].parse().unwrap();
                let y: i32 = cap[5].parse().unwrap();
                result_sum += x * y;
            }
        }
    }

    result_sum as usize
}
