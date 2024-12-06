#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut vector1 = vec![];
    let mut vector2 = vec![];
    for line in lines {
        let numbers = line.split_ascii_whitespace().collect::<Vec<&str>>();
        vector1.push(numbers[0].parse::<i32>().unwrap());
        vector2.push(numbers[1].parse::<i32>().unwrap());
    }
    vector1.sort_by(|a, b| b.partial_cmp(a).unwrap());
    vector2.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut total = 0;
    for i in 0..vector1.len() {
        if vector1[i] > vector2[i] {
            total += vector1[i] - vector2[i];
        } else {
            total += vector2[i] - vector1[i];
        }
    }
    total as usize
}
#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut vector1 = vec![];
    let mut vector2 = vec![];
    for line in lines {
        let numbers = line.split_ascii_whitespace().collect::<Vec<&str>>();
        vector1.push(numbers[0].parse::<i32>().unwrap());
        vector2.push(numbers[1].parse::<i32>().unwrap());
    }
    vector1.sort_by(|a, b| b.partial_cmp(a).unwrap());
    vector2.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut total = 0;
    for i in 0..vector1.len() {
        let a = vector1[i];
        let _b = vector2[i];
        let howmany_of_a = vector2.iter().filter(|&x| *x == a).count();

        total += a * howmany_of_a as i32;
    }
    total as usize
}
