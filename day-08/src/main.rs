// https://adventofcode.com/2024/day/8

use std::collections::HashMap;
use std::convert::From;
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

/* this is a function i used to print the map to varify that the algorithm was finding the right
* antinodes

fn print_map(map: &Vec<Vec<String>>) {
    for row in map {
        println!("{}", row.join(""));
    }
}
*/

fn copy_map(map: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut map_copy: Vec<Vec<String>> = Vec::new();

    for row in map {
        map_copy.push(row.clone());
    }

    return map_copy;
}

fn in_range(map: &[Vec<String>], new_i: i32, new_j: i32) -> bool {
    if new_i < 0 || new_j < 0 {
        return false;
    }
    if new_i >= map.len() as i32 || new_j >= map[0].len() as i32 {
        return false;
    }
    return true;
}

fn count_antinodes(mut map: Vec<Vec<String>>, antenna: &String, i_index: &usize, j_index: &usize) -> u32 {
    let mut count: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let (i_i32, j_i32): (i32, i32) = (i as i32, j as i32);
            let (i_index_i32, j_index_i32): (i32, i32) = (*i_index as i32, *j_index as i32);

            if i_i32 == i_index_i32 && j_i32 == j_index_i32 {
                continue;
            }
            if map[i][j] != *antenna {
                continue;
            }

            let (delta_i, delta_j): (i32, i32) = (i_index_i32 - i_i32, j_index_i32 - j_i32);
            let (new_i, new_j): (i32, i32) = (i_i32 + (delta_i * 2), j_i32 + (delta_j * 2));

            if !in_range(&map, new_i, new_j) {
                continue;
            }

            if map[new_i as usize][new_j as usize] == *"." {
                map[new_i as usize][new_j as usize] = "#".to_string();
            }
        }
    }

    for row in map {
        for col in row {
            if col == *"#" {
                count += 1;
            }
        }
    }

    return count;
}

fn count_antinodes_in_line(
    mut map: Vec<Vec<String>>,
    antenna_on_antinode: &mut HashMap<(usize, usize), bool>,
    antenna: &String,
    i_index: &usize,
    j_index: &usize,
) -> (Vec<Vec<String>>, u32) {
    let mut count: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let (i_i32, j_i32): (i32, i32) = (i as i32, j as i32);
            let (i_index_i32, j_index_i32): (i32, i32) = (*i_index as i32, *j_index as i32);

            if i_i32 == i_index_i32 && j_i32 == j_index_i32 {
                continue;
            }
            if map[i][j] != *antenna {
                continue;
            }

            let mut multiply_by: i32 = 1;
            let (delta_i, delta_j): (i32, i32) = (i_index_i32 - i_i32, j_index_i32 - j_i32);
            let (mut new_i, mut new_j): (i32, i32) = (i_i32 + (delta_i * multiply_by), j_i32 + (delta_j * multiply_by));

            if !in_range(&map, new_i, new_j) {
                continue;
            }

            let (mut new_neg_i, mut new_neg_j): (i32, i32) =
                (i_i32 - (delta_i * multiply_by), j_i32 - (delta_j * multiply_by));
            while in_range(&map, new_i, new_j) {
                if map[new_i as usize][new_j as usize] == *"." {
                    map[new_i as usize][new_j as usize] = "#".to_string();
                } else if map[new_i as usize][new_j as usize] == *antenna {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        antenna_on_antinode.entry((new_i as usize, new_j as usize))
                    {
                        count += 1;
                        e.insert(true);
                    }
                }

                multiply_by += 1;
                (new_i, new_j) = (i_i32 + (delta_i * multiply_by), j_i32 + (delta_j * multiply_by));
            }

            multiply_by = 1;
            while in_range(&map, new_neg_i, new_neg_j) {
                if map[new_neg_i as usize][new_neg_j as usize] == *"." {
                    map[new_neg_i as usize][new_neg_j as usize] = "#".to_string();
                } else if map[new_neg_i as usize][new_neg_j as usize] == *antenna {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        antenna_on_antinode.entry((new_neg_i as usize, new_neg_j as usize))
                    {
                        count += 1;
                        e.insert(true);
                    }
                }

                multiply_by += 1;
                (new_neg_i, new_neg_j) = (i_i32 - (delta_i * multiply_by), j_i32 - (delta_j * multiply_by));
            }
        }
    }

    return (map, count);
}

pub fn get_count_of_all_antinodes(map: &Vec<Vec<String>>) -> u32 {
    let mut count: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let antenna = &map[i][j];

            if *antenna != *"." && *antenna != *"#" {
                count += count_antinodes(copy_map(map), antenna, &i, &j);
            }
        }
    }

    return count;
}

pub fn get_count_of_all_antinodes_in_line(mut map: Vec<Vec<String>>) -> u32 {
    let mut antenna_on_antinode: HashMap<(usize, usize), bool> = HashMap::new();
    let mut count: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let antenna = &map[i][j];

            if *antenna != *"." && *antenna != *"#" {
                let new_count;
                (map, new_count) = count_antinodes_in_line(copy_map(&map), &mut antenna_on_antinode, antenna, &i, &j);
                count += new_count;
            }
        }
    }

    for row in map {
        for col in row {
            if *col == *"#" {
                count += 1;
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

    let count_of_antinodes = get_count_of_all_antinodes(&map);
    println!("total antinodes: {}", count_of_antinodes);

    let count_of_antinodes_in_line = get_count_of_all_antinodes_in_line(copy_map(&map));
    println!("total antinodes in line: {}", count_of_antinodes_in_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count_of_all_antinodes() {
        let map = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let count = get_count_of_all_antinodes(&map);
        assert_eq!(count, 14);
    }

    #[test]
    fn test_get_count_of_all_antinodes_in_line() {
        let map = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let count = get_count_of_all_antinodes_in_line(copy_map(&map));
        assert_eq!(count, 34);
    }
}
