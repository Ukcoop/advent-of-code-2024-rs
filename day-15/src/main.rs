// https://adventofcode.com/2024/day/14

use std::error::Error;

use utils::get_csv_data;

#[derive(PartialEq, Clone, Debug)]
pub enum MapState {
    Wall,
    Box,
    Floor,
}

#[derive(PartialEq, Debug)]
pub enum Moves {
    Left,
    Up,
    Down,
    Right,
}

type MapResult = Result<(Vec<Vec<MapState>>, (usize, usize)), Box<dyn Error>>;

pub fn get_map(path: &str) -> MapResult {
    let mut map: Vec<Vec<MapState>> = Vec::new();
    let mut bot_x: usize = 0;
    let mut bot_y: usize = 0;

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
            if input_map[i][j] == *"@" {
                bot_x = i;
                bot_y = j;
                row.push(MapState::Floor);
            } else if input_map[i][j] == *"#" {
                row.push(MapState::Wall);
            } else if input_map[i][j] == *"O" {
                row.push(MapState::Box);
            } else {
                row.push(MapState::Floor);
            }
        }
        map.push(row);
    }

    Ok((map, (bot_x, bot_y)))
}

pub fn get_moves(path: &str) -> Result<Vec<Moves>, Box<dyn Error>> {
    let mut moves: Vec<Moves> = Vec::new();
    let input: Vec<Vec<String>> = get_csv_data(path, false)?;
    let moves_string: Vec<String> = input[0][0].split("").map(String::from).collect::<Vec<String>>();

    for bot_move in moves_string {
        if bot_move == *"<" {
            moves.push(Moves::Left);
        } else if bot_move == *"^" {
            moves.push(Moves::Up);
        } else if bot_move == *"v" {
            moves.push(Moves::Down);
        } else if bot_move == *"" {
        } else {
            moves.push(Moves::Right);
        }
    }

    return Ok(moves);
}

/* this function prints the map and helps with debuging

fn print_map(map: &Vec<Vec<MapState>>) {
    for row in map {
        let mut new_row: Vec<String> = Vec::new();
        for col in row {
            if *col == MapState::Wall {
                new_row.push("#".to_string());
            } else if *col == MapState::Box {
                new_row.push("O".to_string());
            } else {
                new_row.push(".".to_string());
            }
        }

        println!("{}", new_row.join(""));
    }
}
*/

fn push_boxes(map: &[Vec<MapState>], direction: &Moves, bot_x: &usize, bot_y: &usize) -> (bool, Vec<MapState>) {
    let (mut go_to_x, mut go_to_y) = (*bot_x, *bot_y);
    let mut infront_of_bot: Vec<MapState> = Vec::new();
    let mut available_spot = false;

    while map[go_to_x][go_to_y] != MapState::Wall {
        if *direction == Moves::Left {
            go_to_y -= 1;
        } else if *direction == Moves::Up {
            go_to_x -= 1;
        } else if *direction == Moves::Down {
            go_to_x += 1;
        } else if *direction == Moves::Right {
            go_to_y += 1;
        }

        if map[go_to_x][go_to_y] == MapState::Floor {
            available_spot = true;
        }
        if map[go_to_x][go_to_y] == MapState::Wall {
            break;
        }

        infront_of_bot.push(map[go_to_x][go_to_y].clone());
    }

    if infront_of_bot.is_empty() || !available_spot {
        return (false, infront_of_bot);
    }

    let mut index: usize = 0;
    while infront_of_bot[index] != MapState::Floor {
        index += 1;
    }

    infront_of_bot[index] = MapState::Box;
    infront_of_bot[0] = MapState::Floor;

    return (true, infront_of_bot);
}

pub fn simulate_robot(map: &[Vec<MapState>], moves: &Vec<Moves>, initial_x: &usize, initial_y: &usize) -> u32 {
    let mut sum_of_gps_chords: u32 = 0;

    let mut mut_map = map.to_owned();
    let (mut bot_x, mut bot_y) = (*initial_x, *initial_y);

    for bot_move in moves {
        let (mut go_to_x, mut go_to_y) = (bot_x, bot_y);

        if *bot_move == Moves::Left {
            go_to_y = bot_y - 1;
        } else if *bot_move == Moves::Up {
            go_to_x = bot_x - 1;
        } else if *bot_move == Moves::Down {
            go_to_x = bot_x + 1;
        } else if *bot_move == Moves::Right {
            go_to_y = bot_y + 1;
        }

        if mut_map[go_to_x][go_to_y] == MapState::Wall {
            continue;
        } else if mut_map[go_to_x][go_to_y] == MapState::Floor {
            bot_x = go_to_x;
            bot_y = go_to_y;
        } else {
            let (can_push, new_values) = push_boxes(&mut_map, bot_move, &bot_x, &bot_y);
            let (mut update_x, mut update_y) = (bot_x, bot_y);

            if can_push {
                for update in new_values {
                    if *bot_move == Moves::Left {
                        update_y -= 1;
                    } else if *bot_move == Moves::Up {
                        update_x -= 1;
                    } else if *bot_move == Moves::Down {
                        update_x += 1;
                    } else if *bot_move == Moves::Right {
                        update_y += 1;
                    }

                    mut_map[update_x][update_y] = update;
                }

                bot_x = go_to_x;
                bot_y = go_to_y;
            }
        }
    }

    for i in 0..mut_map.len() {
        for j in 0..mut_map[0].len() {
            if mut_map[i][j] == MapState::Box {
                sum_of_gps_chords += (100 * i as u32) + j as u32;
            }
        }
    }

    return sum_of_gps_chords;
}

fn main() {
    let (map, (initial_x, initial_y)) = match get_map("data/input/map.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve map. {}", e);
            return;
        }
    };

    let moves = match get_moves("data/input/moves.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve moves. {}", e);
            return;
        }
    };

    let result = simulate_robot(&map, &moves, &initial_x, &initial_y);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_robot() {
        let (map, (initial_x, initial_y)) = match get_map("data/test/map.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let moves = match get_moves("data/test/moves.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve moves. {}", e);
                return;
            }
        };

        let result = simulate_robot(&map, &moves, &initial_x, &initial_y);
        assert_eq!(result, 10092);
    }
}
