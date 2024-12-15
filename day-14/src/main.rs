// https://adventofcode.com/2024/day/14

use std::error::Error;

use utils::get_csv_data;

#[derive(Debug, Clone)]
pub struct Robot {
    pos_x: i32,
    pos_y: i32,
    delta_x: i32,
    delta_y: i32,
}

pub fn get_mechines(path: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    let input: Vec<Vec<String>> = get_csv_data(path, false)?;
    let mut robots: Vec<Robot> = Vec::new();

    for row in input {
        let pos_x = row[0].split("=").map(String::from).collect::<Vec<String>>()[1].clone();
        let pos_y = row[1].split(" ").map(String::from).collect::<Vec<String>>()[0].clone();
        let delta_x = row[1].split("=").map(String::from).collect::<Vec<String>>()[1].clone();
        let delta_y = row[2].clone();

        let robot = Robot {
            pos_x: pos_x.parse::<i32>().unwrap_or(0),
            pos_y: pos_y.parse::<i32>().unwrap_or(0),
            delta_x: delta_x.parse::<i32>().unwrap_or(0),
            delta_y: delta_y.parse::<i32>().unwrap_or(0),
        };

        robots.push(robot);
    }

    return Ok(robots);
}

pub fn simulate_robots(robots: &[Robot], time: u32, map_x: i32, map_y: i32) -> u32 {
    let mut mut_robots = robots.to_owned();

    for _ in 0..time {
        for i in 0..mut_robots.len() {
            mut_robots[i].pos_x += mut_robots[i].delta_x;
            mut_robots[i].pos_y += mut_robots[i].delta_y;

            if mut_robots[i].pos_x >= map_x {
                mut_robots[i].pos_x -= map_x;
            }
            if mut_robots[i].pos_x < 0 {
                mut_robots[i].pos_x += map_x;
            }

            if mut_robots[i].pos_y >= map_y {
                mut_robots[i].pos_y -= map_y;
            }
            if mut_robots[i].pos_y < 0 {
                mut_robots[i].pos_y += map_y;
            }
        }
    }

    let mid_x = map_x / 2;
    let mid_y = map_y / 2;

    let mut quadrent_1: u32 = 0;
    let mut quadrent_2: u32 = 0;
    let mut quadrent_3: u32 = 0;
    let mut quadrent_4: u32 = 0;

    for i in 0..mut_robots.len() {
        if mut_robots[i].pos_x < mid_x && mut_robots[i].pos_y < mid_y {
            quadrent_1 += 1;
        }
        if mut_robots[i].pos_x > mid_x && mut_robots[i].pos_y < mid_y {
            quadrent_2 += 1;
        }
        if mut_robots[i].pos_x < mid_x && mut_robots[i].pos_y > mid_y {
            quadrent_3 += 1;
        }
        if mut_robots[i].pos_x > mid_x && mut_robots[i].pos_y > mid_y {
            quadrent_4 += 1;
        }
    }

    return quadrent_1.max(1) * quadrent_2.max(1) * quadrent_3.max(1) * quadrent_4.max(1);
}

fn main() {
    let robots: Vec<Robot> = match get_mechines("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve robots. {}", e);
            return;
        }
    };

    let result = simulate_robots(&robots, 100, 101, 103);
    println!("{:#?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_robots() {
        let robots: Vec<Robot> = match get_mechines("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve robots. {}", e);
                return;
            }
        };

        let result = simulate_robots(&robots, 100, 11, 7);
        assert_eq!(result, 12);
    }
}
