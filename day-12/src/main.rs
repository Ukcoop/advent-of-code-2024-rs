// https://adventofcode.com/2024/day/12

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use utils::get_csv_data;

pub fn get_map(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut map: Vec<Vec<String>> = get_csv_data(path, false)?;
    let mut border: Vec<String> = Vec::new();

    let row_len: usize = map[0][0].split("").map(String::from).collect::<Vec<String>>().len();
    for _ in 0..row_len {
        border.push(".".to_string());
    }

    for i in 0..map.len() {
        let mut row: Vec<String> = map[i][0].split("").map(String::from).collect();

        let end = row.len() - 1;
        row[0] = ".".to_string();
        row[end] = ".".to_string();

        map[i] = row;
    }

    map.push(border);
    map.insert(0, map[map.len() - 1].clone());

    return Ok(map);
}

fn get_fences(map: &[Vec<String>]) -> Vec<Vec<u32>> {
    let mut fences: Vec<Vec<u32>> = Vec::new();

    for i in 0..map.len() {
        let mut row: Vec<u32> = Vec::new();

        for j in 0..map[i].len() {
            if map[i][j] == *"." {
                row.push(0);
                continue;
            }

            let mut check_spots: Vec<(usize, usize)> = Vec::new();
            let mut fence_number: u32 = 0;

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

            fence_number += 4 - check_spots.len() as u32;

            for (spot_i, spot_j) in check_spots {
                if map[i][j] != map[spot_i][spot_j] {
                    fence_number += 1;
                }
            }

            row.push(fence_number);
        }

        fences.push(row);
    }

    return fences;
}
/* this function only works in the test case

fn get_sides(map: &[Vec<String>]) -> u32 {
    let mut count: u32 = 0;
    let mut detecting_edge: bool = false;
    let mut prev = ".".to_string();

    for i in 0..(map.len() - 1) {
        for j in 0..map[i].len() {
            if map[i][j] != map[i + 1][j] {
                if detecting_edge && map[i][j] != prev {
                    count += 1;
                    detecting_edge = false;
                } else {
                    detecting_edge = true;
                    prev = map[i][j].clone();
                }
            } else if detecting_edge {
                count += 1;
                detecting_edge = false;
            }
        }
    }

    prev = ".".to_string();
    detecting_edge = false;

    for j in 0..(map.len() - 1) {
        for i in 0..map[j].len() {
            if map[i][j] != map[i][j + 1] {
                if detecting_edge && map[i][j] != prev {
                    count += 1;
                    detecting_edge = false;
                } else {
                    detecting_edge = true;
                    prev = map[i][j].clone();
                }
            } else if detecting_edge {
                count += 1;
                detecting_edge = false;
            }
        }
    }

    return count;
}
*/

/* this is my horrific attempt on solving part 2, there is way to manny variables to ajust

fn get_sides(map: &[Vec<String>]) -> u32 {
    let mut count: u32 = 0;

    for i in 0..(map.len() - 2) {
        for j in 0..(map[i].len() - 2) {
            if map[i + 1][j + 1] == *"." { continue; }
            let corner_a = map[i + 1][j] == map[i + 1][j + 1] && map[i][j + 1] == map[i + 1][j + 1];
            let corner_b = map[i][j + 1] == map[i + 1][j + 1] && map[i + 1][j + 2] == map[i + 1][j + 1];
            let corner_c = map[i + 1][j + 2] == map[i + 1][j + 1] && map[i + 2][j + 1] == map[i + 1][j + 1];
            let corner_d = map[i + 2][j + 1] == map[i + 1][j + 1] && map[i + 1][j] == map[i + 1][j + 1];

            let double_a = corner_a && map[i][j] != map[i + 1][j + 1];
            let double_b = corner_b && map[i][j + 2] != map[i + 1][j + 1];
            let double_c = corner_c && map[i + 2][j + 2] != map[i + 1][j + 1];
            let double_d = corner_d && map[i + 2][j] != map[i + 1][j + 1];

            let kite_a = corner_a && map[i][j] == map[i + 1][j + 1] && map[i + 2][j + 2] == map[i + 1][j + 1] &&
                map[i + 1][j + 2] != map[i + 1][j + 1] && map[i + 2][j + 1] != map[i + 1][j + 1];

            let kite_b = corner_b && map[i][j + 2] == map[i + 1][j + 1] && map[i + 2][j] == map[i + 1][j + 1] &&
                map[i + 1][j] != map[i + 1][j + 1] && map[i + 2][j + 1] != map[i + 1][j + 1];

            let kite_c = corner_c && map[i + 2][j + 2] == map[i + 1][j + 1] && map[i][j] == map[i + 1][j + 1] &&
                map[i + 1][j] != map[i + 1][j + 1] && map[i][j + 1] != map[i + 1][j + 1];

            let kite_d = corner_d && map[i + 2][j] == map[i + 1][j + 1] && map[i][j + 2] == map[i + 1][j + 1] &&
                map[i][j + 1] != map[i + 1][j + 1] && map[i + 1][j + 2] != map[i + 1][j + 1];

            let not_kite_a = corner_a && map[i][j] == map[i + 1][j + 1] && map[i + 2][j + 2] == map[i + 1][j + 1] &&
                (map[i + 1][j + 2] == map[i + 1][j + 1] || map[i + 2][j + 1] == map[i + 1][j + 1]);

            let not_kite_b = corner_b && map[i][j + 2] == map[i + 1][j + 1] && map[i + 2][j] == map[i + 1][j + 1] &&
                (map[i + 1][j] == map[i + 1][j + 1] || map[i + 2][j + 1] == map[i + 1][j + 1]);

            let not_kite_c = corner_c && map[i + 2][j + 2] == map[i + 1][j + 1] && map[i][j] == map[i + 1][j + 1] &&
                (map[i + 1][j] == map[i + 1][j + 1] || map[i][j + 1] == map[i + 1][j + 1]);

            let not_kite_d = corner_d && map[i + 2][j] == map[i + 1][j + 1] && map[i][j + 2] == map[i + 1][j + 1] &&
                (map[i][j + 1] == map[i + 1][j + 1] || map[i + 1][j + 2] == map[i + 1][j + 1]);

            let H_a = map[i][j] == map[i][j + 1] && map[i][j + 2] == map[i][j + 1];
            let H_b = map[i + 1][j] == map[i + 1][j + 1] && map[i + 1][j + 2] == map[i + 1][j + 1];
            let H_c = map[i + 2][j] == map[i + 2][j + 1] && map[i + 2][j + 2] == map[i + 2][j + 1];

            let H_not_a = map[i][j] != map[i][j + 1] && map[i][j + 2] != map[i][j + 1];
            let H_not_b = map[i + 1][j] != map[i + 1][j + 1] && map[i + 1][j + 2] != map[i + 1][j + 1];
            let H_not_c = map[i + 2][j] != map[i + 1][j + 1] && map[i + 2][j + 2] != map[i + 1][j + 1];

            let V_a = map[i][j] == map[i + 1][j] && map[i + 2][j] == map[i + 1][j];
            let V_b = map[i][j + 1] == map[i + 1][j + 1] && map[i + 2][j + 1] == map[i + 1][j + 1];
            let V_c = map[i][j + 2] == map[i + 1][j + 2] && map[i + 2][j + 2] == map[i + 1][j + 2];

            let V_not_a = map[i][j] != map[i + 1][j] && map[i + 2][j] != map[i + 1][j];
            let V_not_b = map[i][j + 1] != map[i + 1][j + 1] && map[i + 2][j + 1] != map[i + 1][j + 1];
            let V_not_c = map[i][j + 2] != map[i + 1][j + 1] && map[i + 2][j + 2] != map[i + 1][j + 1];

            let is_wall = (H_a && H_b) || (H_c && H_b) || (V_a && V_b) || (V_c && V_b);

            let is_corner = corner_a || corner_b || corner_c || corner_d;
            let is_double = double_a || double_b || double_c || double_d;

            let is_kite = kite_a || kite_b || kite_c || kite_d;
            let is_not_kite = not_kite_a || not_kite_b || not_kite_c || not_kite_d;

            let is_dot = (H_a && !H_b && H_c) && (V_a && !V_b && V_c);

            let is_nub = (H_a && !H_b && H_c) || (H_a && !H_b && !H_c) || (!H_a && !H_b && H_c) ||
                (V_a && !V_b && V_c) || (V_a && !V_b && !V_c)  || (!V_a && !V_b && V_c);

            let is_h = (V_a && V_not_b && V_c) || (H_a && H_not_b && H_c);

            let is_plus = V_b && H_b && H_not_a && H_not_c && V_not_a && V_not_c;

            let is_t = (V_a && H_b && V_not_b && V_not_c) || (V_c && H_b && V_not_b && V_not_c) ||
                (H_a && V_b && H_not_b && H_not_c) || (H_c && V_b && H_not_b && H_not_c);

            let is_not_t = (V_a && H_b && (!V_not_b || !V_not_c)) || (V_c && H_b && (!V_not_b || !V_not_c)) ||
                (H_a && V_b && (!H_not_b || !H_not_c)) || (H_c && V_b && (!H_not_b || !H_not_c));

            let is_inv_corner = (H_a && H_b && V_b && V_c) || (H_a && H_b && V_a && V_b) || (H_b && H_c && V_b && V_c) || (H_b && H_c && V_a && V_b);

            //if is_wall && is_corner { continue; }

            if is_dot {
                count += 4;
                println!("dot");
            } else if is_kite {
                count += 3;
                println!("kite");
            } else if is_t {
                count += 2;
            } else if is_double && !is_not_t && !is_not_kite && !is_plus {
                count += 2;
                println!("double");
            } else if is_corner && !is_wall /*&& !is_not_t*/ && !is_not_kite && !is_plus {
                count += 1;
                println!("corner");
            } else if is_inv_corner {
                count += 1;
                println!("inv corner");
            } else if is_nub && !is_not_t {
                count += 2;
                println!("nub");
            } else if is_h {
                count += 3;
                println!("h");
            } else { /*continue;*/ }

            println!("{}, {}, {}", map[i][j], map[i][j + 1], map[i][j + 2]);
            println!("{}, {}, {}", map[i + 1][j], map[i + 1][j + 1], map[i + 1][j + 2]);
            println!("{}, {}, {}", map[i + 2][j], map[i + 2][j + 1], map[i + 2][j + 2]);
            println!("---");
        }
    }

    return count;
}
*/

fn get_unique_regions(map: &[Vec<String>]) -> Vec<Vec<Vec<String>>> {
    let rows = map.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<Vec<Vec<String>>> = Vec::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn flood_fill(
        map: &[Vec<String>],
        visited: &mut HashSet<(usize, usize)>,
        start: (usize, usize),
        crop_type: &str,
        rows: usize,
        cols: usize,
        directions: &[(isize, isize)],
    ) -> Vec<(usize, usize)> {
        let mut stack = vec![start];
        let mut region_cells = Vec::new();

        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            region_cells.push((x, y));

            for &(dx, dy) in directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if nx < rows && ny < cols && !visited.contains(&(nx, ny)) && map[nx][ny] == crop_type {
                        stack.push((nx, ny));
                    }
                }
            }
        }

        return region_cells;
    }

    for x in 0..rows {
        for y in 0..cols {
            if !visited.contains(&(x, y)) {
                let crop_type = &map[x][y];
                let region_cells = flood_fill(map, &mut visited, (x, y), crop_type, rows, cols, &directions);

                let mut region = vec![vec![".".to_string(); cols]; rows];
                for &(rx, ry) in &region_cells {
                    region[rx][ry] = crop_type.clone();
                }
                regions.push(region);
            }
        }
    }

    return regions;
}

fn get_data_for_crop_types(map: &[Vec<String>]) -> HashMap<String, (u32, u32)> {
    let mut data: HashMap<String, (u32, u32)> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == *"." {
                continue;
            }

            if let std::collections::hash_map::Entry::Vacant(e) = data.entry(map[i][j].clone()) {
                e.insert((0, 0));
            }

            let (count, fence_count) = data[&map[i][j]];

            data.insert(map[i][j].clone(), (count + 1, fence_count + get_fences(map)[i][j]));
        }
    }

    return data;
}

pub fn get_cost_of_fencing(maps: Vec<Vec<Vec<String>>>) -> u32 {
    return maps
        .par_iter()
        .map(|map| {
            let data: HashMap<String, (u32, u32)> = get_data_for_crop_types(map);
            data.iter()
                .map(|(_, &(count, fence_count))| count * fence_count)
                .sum::<u32>()
        })
        .sum();
}

fn main() {
    let map = match get_map("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve map. {}", e);
            return;
        }
    };

    let cost = get_cost_of_fencing(get_unique_regions(&map));
    println!("cost: {}", cost);
    /*
        let cost_with_discount = get_cost_of_fencing(get_unique_regions(&map), true);
        println!(
            "cost with discount (witch aparently is not right): {}",
            cost_with_discount
        );
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cost_of_fencing() {
        let map = match get_map("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve map. {}", e);
                return;
            }
        };

        let cost = get_cost_of_fencing(get_unique_regions(&map));
        assert_eq!(cost, 1930);
    }
    /*
        #[test]
        fn test_get_cost_of_fencing_with_bulk_discount() {
            let map = match get_map("data/test.csv") {
                Ok(result) => result,
                Err(e) => {
                    println!("Error: Failed to retrieve map. {}", e);
                    return;
                }
            };

            let cost = get_cost_of_fencing(get_unique_regions(&map), true);
            assert_eq!(cost, 1206);
        }
    */
}
