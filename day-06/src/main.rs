// https://adventofcode.com/2024/day/6

use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

use utils::get_csv_data;

#[derive(PartialEq, Clone)]
pub enum MapState {
    Wall,
    Explored,
    Unexplored,
}

#[derive(PartialEq)]
enum GuardDirection {
    Left,
    Up,
    Down,
    Right,
}

type MapResult = Result<(Vec<Vec<MapState>>, (usize, usize)), Box<dyn Error>>;

fn get_map(path: &str) -> MapResult {
    let mut map: Vec<Vec<MapState>> = Vec::new();
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;

    let mut input_map: Vec<Vec<String>> = get_csv_data(path, false)?;

    for i in 0..input_map.len() {
        let mut row: Vec<String> = input_map[i][0].split("").map(String::from).collect();

        if row.first() == Some(&String::from("")) {
            row.remove(0);
        }

        if row.last() == Some(&String::from("")) {
            row.pop();
        }
        input_map[i] = row;
    }

    for i in 0..input_map.len() {
        let mut row: Vec<MapState> = Vec::new();
        for j in 0..input_map.len() {
            if input_map[i][j] == *"^" {
                guard_x = i;
                guard_y = j;
                row.push(MapState::Explored);
            } else if input_map[i][j] == *"#" {
                row.push(MapState::Wall);
            } else {
                row.push(MapState::Unexplored);
            }
        }
        map.push(row);
    }

    Ok((map, (guard_x, guard_y)))
}

fn copy_map(map: &Vec<Vec<MapState>>) -> Vec<Vec<MapState>> {
    let mut map_copy: Vec<Vec<MapState>> = Vec::new();

    for row in map {
        map_copy.push(row.clone());
    }

    return map_copy;
}

pub fn get_unique_positions(
    mut map: Vec<Vec<MapState>>,
    initial_x: usize,
    initial_y: usize,
) -> (Vec<Vec<MapState>>, bool, u32) {
    let mut guard_x = initial_x;
    let mut guard_y = initial_y;
    let mut guard_direction = GuardDirection::Up;
    let mut visited: HashMap<String, bool> = HashMap::new();

    let mut explored_count = 0;
    let mut count: u32 = 0;

    if map[initial_x - 1][initial_y] == MapState::Wall {
        return (map, false, 0);
    }

    loop {
        let (mut go_to_x, mut go_to_y) = (guard_x, guard_y);

        if guard_direction == GuardDirection::Left {
            if guard_y == 0 {
                break;
            }
            go_to_y = guard_y - 1;
        } else if guard_direction == GuardDirection::Up {
            if guard_x == 0 {
                break;
            }
            go_to_x = guard_x - 1;
        } else if guard_direction == GuardDirection::Down {
            if guard_x + 1 == map.len() {
                break;
            }
            go_to_x = guard_x + 1;
        } else if guard_direction == GuardDirection::Right {
            if guard_y + 1 == map[0].len() {
                break;
            }
            go_to_y = guard_y + 1;
        }

        if map[go_to_x][go_to_y] == MapState::Wall {
            guard_direction = match guard_direction {
                GuardDirection::Left => GuardDirection::Up,
                GuardDirection::Up => GuardDirection::Right,
                GuardDirection::Right => GuardDirection::Down,
                GuardDirection::Down => GuardDirection::Left,
            };
        } else {
            guard_x = go_to_x;
            guard_y = go_to_y;
        }

        let possible_visit = format!("{}|{}", guard_x, guard_y);

        if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(possible_visit) {
            explored_count = 0;
            e.insert(true);
        } else {
            explored_count += 1;
        }

        map[guard_x][guard_y] = MapState::Explored;
        if explored_count == 5000 {
            return (map, true, 0);
        }
    }

    for row in &map {
        for col in row {
            if *col == MapState::Explored {
                count += 1;
            }
        }
    }

    return (map, false, count);
}

pub fn total_possible_loops(map: Vec<Vec<MapState>>, initial_x: usize, initial_y: usize) -> u32 {
    let (explored_map, _, _) = get_unique_positions(copy_map(&map), initial_x, initial_y);

    let count: u32 = explored_map
        .par_iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, state)| {
                    if *state == MapState::Explored {
                        let mut new_map = copy_map(&map);
                        new_map[i][j] = MapState::Wall;
                        let (_, looped, _) = get_unique_positions(new_map, initial_x, initial_y);
                        if looped {
                            Some(1)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum();

    return count;
}

fn main() {
    let (map, (initial_x, initial_y)) = match get_map("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve map data. {}", e);
            return;
        }
    };

    let (_, _, unique_positions) = get_unique_positions(copy_map(&map), initial_x, initial_y);
    println!("unique positions: {}", unique_positions);

    let possible_loops = total_possible_loops(copy_map(&map), initial_x, initial_y);
    println!("possible loops: {}", possible_loops);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_positions() {
        let (map, (initial_x, initial_y)) = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve map data in test_get_unique_positions. {}", e);
            }
        };

        let (_, looped, result) = get_unique_positions(map, initial_x, initial_y);
        assert_eq!(looped, false);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_total_possible_loops() {
        let (map, (initial_x, initial_y)) = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve map data in test_total_possible_loops. {}", e);
            }
        };

        let result = total_possible_loops(map, initial_x, initial_y);
        assert_eq!(result, 6);
    }
}
