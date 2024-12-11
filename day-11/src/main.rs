// https://adventofcode.com/2024/day/9

use std::collections::HashMap;

use utils::get_csv_data;

fn apply_change_rules(rocks: &mut HashMap<u64, u64>) {
    let mut new_rocks: HashMap<u64, u64> = HashMap::new();

    for (&rock_value, &count) in rocks.iter() {
        let rock_string = rock_value.to_string();

        if rock_value == 0 {
            *new_rocks.entry(1).or_insert(0) += count;
        } else if rock_string.len() % 2 == 0 {
            let (left, right) = rock_string.split_at(rock_string.len() / 2);
            let left_value = left.parse::<u64>().unwrap_or(0);
            let right_value = right.parse::<u64>().unwrap_or(0);
            *new_rocks.entry(left_value).or_insert(0) += count;
            *new_rocks.entry(right_value).or_insert(0) += count;
        } else {
            *new_rocks.entry(rock_value * 2024).or_insert(0) += count;
        }
    }

    *rocks = new_rocks;
}

pub fn get_count_of_rocks_after_blinks(rocks: Vec<u64>, blinks: u32) -> u64 {
    let mut rock_counts: HashMap<u64, u64> = HashMap::new();

    for rock in rocks {
        *rock_counts.entry(rock).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        apply_change_rules(&mut rock_counts);
    }

    rock_counts.values().copied().sum::<u64>()
}

fn main() {
    let rocks: Vec<u64> = match get_csv_data("data/input.csv", false) {
        Ok(result) => result[0].clone(),
        Err(e) => {
            println!("Error: Failed to retrieve rocks. {}", e);
            return;
        }
    };

    let blinks_25 = get_count_of_rocks_after_blinks(rocks.clone(), 25);
    println!("after 25 blinks: {}", blinks_25);

    let blinks_75 = get_count_of_rocks_after_blinks(rocks, 75);
    println!("after 75 blinks: {}", blinks_75);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_count_of_rocks_after_25_blinks() {
        let rocks: Vec<u64> = match get_csv_data("data/test.csv", false) {
            Ok(result) => result[0].clone(),
            Err(e) => {
                println!("Error: Failed to retrieve rocks. {}", e);
                return;
            }
        };

        let result = get_count_of_rocks_after_blinks(rocks, 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_get_count_of_rocks_after_75_blinks() {
        let rocks: Vec<u64> = match get_csv_data("data/test.csv", false) {
            Ok(result) => result[0].clone(),
            Err(e) => {
                println!("Error: Failed to retrieve rocks. {}", e);
                return;
            }
        };

        let result = get_count_of_rocks_after_blinks(rocks, 75);
        assert_eq!(result, 65601038650482);
    }
}
