// https://adventofcode.com/2024/day/10

use std::collections::HashMap;
use std::error::Error;

use utils::get_csv_data;

pub fn get_map(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut map: Vec<Vec<String>> = get_csv_data(path, false)?;

    for i in 0..map.len() {
        let mut row: Vec<String> = map[i][0].split("").map(String::from).collect();
        if row.first() == Some(&String::from("")) {
            row.remove(0);
        }

        if row.last() == Some(&String::from("")) {
            row.pop();
        }

        map[i] = row;
    }

    return Ok(map);
}

fn count_trail_paths(map: &Vec<Vec<String>>, i_start: &usize, j_start: &usize) -> Vec<(bool, (usize, usize))> {
    let mut found_9: Vec<(bool, (usize, usize))> = Vec::new();
    let mut i = *i_start;
    let mut j = *j_start;

    loop {
        if map[i][j] == *"9" {
            return vec![(true, (i, j))];
        }

        let digit = map[i][j].parse::<u32>().unwrap_or(0);
        let mut check_spots: Vec<(usize, usize)> = Vec::new();
        let mut can_go_to: Vec<(usize, usize)> = Vec::new();

        if j != 0 {
            check_spots.push((i, j - 1))
        }

        if i != 0 {
            check_spots.push((i - 1, j));
        }

        if i != map.len() - 1 {
            check_spots.push((i + 1, j));
        }

        if j != map[0].len() - 1 {
            check_spots.push((i, j + 1));
        }

        for (spot_i, spot_j) in check_spots {
            if digit + 1 == map[spot_i][spot_j].parse::<u32>().unwrap_or(10) {
                can_go_to.push((spot_i, spot_j));
            }
        }

        if can_go_to.is_empty() {
            return vec![(false, (0, 0))];
        }

        if can_go_to.len() == 1 {
            (i, j) = can_go_to[0];
        } else {
            for (spot_i, spot_j) in can_go_to {
                let found = count_trail_paths(map, &spot_i, &spot_j);

                for spot in found {
                    found_9.push(spot);
                }
            }

            return found_9;
        }
    }
}

pub fn get_count_of_all_paths(map: &Vec<Vec<String>>, count_mutiple_routes: bool) -> u32 {
    let mut count: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == *"0" {
                let mut unique_positions: HashMap<(usize, usize), bool> = HashMap::new();
                let found = count_trail_paths(map, &i, &j);

                for (valid, position) in found {
                    if valid {
                        if count_mutiple_routes {
                            count += 1;
                        } else if let std::collections::hash_map::Entry::Vacant(e) = unique_positions.entry(position) {
                            count += 1;
                            e.insert(true);
                        }
                    }
                }
            }
        }
    }

    return count;
}

fn main() {
    let map = match get_map("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve map. {}", e);
            return;
        }
    };

    let count_of_all_unique_paths = get_count_of_all_paths(&map, false);
    println!("all unique paths: {}", count_of_all_unique_paths);

    let count_of_all_possible_paths = get_count_of_all_paths(&map, true);
    println!("all possible paths: {}", count_of_all_possible_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count_of_all_unique_paths() {
        let map = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let result = get_count_of_all_paths(&map, false);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_get_count_of_all_possible_paths() {
        let map = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let result = get_count_of_all_paths(&map, true);
        assert_eq!(result, 81);
    }
}
