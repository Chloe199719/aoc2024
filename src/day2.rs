#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut states = vec![];
    for line in lines {
        let state = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        states.push(state);
    }
    let safe_states = states
        .iter()
        .map(|state| {
            let mut safe = true;

            let mut current_state = State::NOTSET;
            let mut laststate = state[0];
            for a in 0..state.len() {
                if safe == false {
                    break;
                }
                if current_state == State::NOTSET {
                    current_state = State::FIRST;
                    laststate = state[a];
                } else if current_state == State::FIRST {
                    if state[a] > laststate {
                        current_state = State::INCREASING;
                        laststate = state[a];
                    } else if state[a] < laststate {
                        current_state = State::DECREASING;
                    }
                } else if current_state == State::INCREASING {
                    if state[a] - laststate < 0 {
                        safe = false;
                    }
                    laststate = state[a];
                } else if current_state == State::DECREASING {
                    if state[a] - laststate > 0 {
                        safe = false;
                    }
                    laststate = state[a];
                }
                let current = state[a];
                if a == 0 && a + 2 < state.len() {
                    let right1 = state[a + 1];
                    let right2 = state[a + 2];
                    if right1 - current > 3 || right1 - current < -3 {
                        safe = false;
                    }
                    if right1 - current == 0 {
                        safe = false;
                    }
                    if right2 - right1 > 3 || right2 - right1 < -3 {
                        safe = false;
                    }
                    if right2 - right1 == 0 {
                        safe = false;
                    }
                } else if a == 1 {
                    let right1 = state[a + 1];
                    let right2 = state[a + 2];
                    let left1 = state[a - 1];
                    if right1 - current > 3 || right1 - current < -3 {
                        safe = false;
                    }
                    if right1 - current == 0 {
                        safe = false;
                    }
                    if right2 - right1 > 3 || right2 - right1 < -3 {
                        safe = false;
                    }
                    if right2 - right1 == 0 {
                        safe = false;
                    }
                    if current - left1 > 3 || current - left1 < -3 {
                        safe = false;
                    }
                    if current - left1 == 0 {
                        safe = false;
                    }
                } else if a == state.len() - 1 {
                    let left1 = state[a - 1];
                    let left2 = state[a - 2];
                    if current - left1 > 3 || current - left1 < -3 {
                        safe = false;
                    }
                    if current - left1 == 0 {
                        safe = false;
                    }
                    if left1 - left2 > 3 || left1 - left2 < -3 {
                        safe = false;
                    }
                    if left1 - left2 == 0 {
                        safe = false;
                    }
                } else if a == state.len() - 2 {
                    let left1 = state[a - 1];
                    let left2 = state[a - 2];
                    let right1 = state[a + 1];
                    if current - left1 > 3 || current - left1 < -3 {
                        safe = false;
                    }
                    if current - left1 == 0 {
                        safe = false;
                    }
                    if left1 - left2 > 3 || left1 - left2 < -3 {
                        safe = false;
                    }
                    if left1 - left2 == 0 {
                        safe = false;
                    }
                    if right1 - current > 3 || right1 - current < -3 {
                        safe = false;
                    }
                    if right1 - current == 0 {
                        safe = false;
                    }
                } else {
                    let left1 = state[a - 1];
                    let left2 = state[a - 2];
                    let right1 = state[a + 1];
                    let right2 = state[a + 2];
                    if current - left1 > 3 || current - left1 < -3 {
                        safe = false;
                    }
                    if current - left1 == 0 {
                        safe = false;
                    }
                    if left1 - left2 > 3 || left1 - left2 < -3 {
                        safe = false;
                    }
                    if left1 - left2 == 0 {
                        safe = false;
                    }
                    if right1 - current > 3 || right1 - current < -3 {
                        safe = false;
                    }
                    if right1 - current == 0 {
                        safe = false;
                    }
                    if right2 - right1 > 3 || right2 - right1 < -3 {
                        safe = false;
                    }
                    if right2 - right1 == 0 {
                        safe = false;
                    }
                }
            }
            safe
        })
        .collect::<Vec<bool>>();

    safe_states.iter().filter(|&x| *x == true).count()
}
#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut states = vec![];
    for line in lines {
        let state = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        states.push(state);
    }
    let safe_count = states
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    safe_count
}
#[derive(Debug, PartialEq, Eq)]
enum State {
    FIRST,
    NOTSET,
    INCREASING,
    DECREASING,
}
fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        // A single level or empty report could be considered trivially safe.
        return true;
    }

    // Check if the sequence is strictly increasing or strictly decreasing
    // Also verify differences are between 1 and 3.
    let mut increasing = true;
    let mut decreasing = true;

    for w in levels.windows(2) {
        let diff = w[1] - w[0];
        if diff <= 0 {
            increasing = false;
        }
        if diff >= 0 {
            decreasing = false;
        }
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            // If any difference is out of range, it's not safe
            return false;
        }
    }

    // Return true if at least one direction (all-increasing or all-decreasing) holds
    increasing || decreasing
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    // First, check if it's already safe as is
    if is_safe(levels) {
        return true;
    }

    // If not, try removing one level at each possible position
    // and see if the resulting sequence is safe.
    for i in 0..levels.len() {
        let mut new_levels = Vec::with_capacity(levels.len() - 1);
        new_levels.extend_from_slice(&levels[0..i]);
        new_levels.extend_from_slice(&levels[i + 1..]);
        if is_safe(&new_levels) {
            return true;
        }
    }
    false
}
