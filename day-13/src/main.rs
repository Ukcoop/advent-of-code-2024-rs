// https://adventofcode.com/2024/day/13

use rayon::prelude::*;
use std::error::Error;

use utils::get_csv_data;

#[derive(Clone)]
pub struct Mechine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

pub fn get_mechines(path: &str) -> Result<Vec<Mechine>, Box<dyn Error>> {
    let input: Vec<Vec<String>> = get_csv_data(path, false)?;
    let mut mechines: Vec<Mechine> = Vec::new();
    let mut index: usize = 0;

    while index < input.len() {
        let a_x = input[index][0].split("+").map(String::from).collect::<Vec<String>>()[1].clone();
        let a_y = input[index][1].split("+").map(String::from).collect::<Vec<String>>()[1].clone();

        let b_x = input[index + 1][0]
            .split("+")
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .clone();
        let b_y = input[index + 1][1]
            .split("+")
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .clone();

        let prize_x = input[index + 2][0]
            .split("=")
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .clone();
        let prize_y = input[index + 2][1]
            .split("=")
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .clone();

        let new_mechine = Mechine {
            button_a: (a_x.parse::<u64>().unwrap_or(0), a_y.parse::<u64>().unwrap_or(0)),
            button_b: (b_x.parse::<u64>().unwrap_or(0), b_y.parse::<u64>().unwrap_or(0)),
            prize: (prize_x.parse::<u64>().unwrap_or(0), prize_y.parse::<u64>().unwrap_or(0)),
        };

        mechines.push(new_mechine);
        index += 3;
    }

    return Ok(mechines);
}

pub fn fix_precision_errors(mechines: &Vec<Mechine>) -> Vec<Mechine> {
    let mut new_mechines: Vec<Mechine> = Vec::new();

    for mechine in mechines {
        let mut new_mechine = mechine.clone();

        let (mut x, mut y) = new_mechine.prize;
        x += 10000000000000;
        y += 10000000000000;

        new_mechine.prize = (x, y);
        new_mechines.push(new_mechine);
    }

    return new_mechines;
}

fn get_x_and_y(a_x: u64, a_y: u64, b_x: u64, b_y: u64, prize_x: u64, prize_y: u64) -> Option<(u64, u64, u64)> {
    (0..=100)
        .into_par_iter()
        .filter_map(|x| {
            (0..=100).find_map(|y| {
                let total_x = x * a_x + y * b_x;
                let total_y = x * a_y + y * b_y;

                if total_x == prize_x && total_y == prize_y {
                    let cost = x * 3 + y;
                    Some((x, y, cost))
                } else {
                    None
                }
            })
        })
        .reduce_with(|a, b| if a.2 < b.2 { a } else { b })
}

/* here us my attempt on useing the genetic algorithm to find the large problems

use rand::{Rng, rngs::StdRng, SeedableRng};
//use rayon::prelude::*;

#[derive(Clone)]
struct Individual {
    x: u64,
    y: u64,
}

fn get_population_range(prize_x: u64, prize_y: u64) -> u64 {
    // Use the maximum of prize_x and prize_y to determine the scaling factor
    let prize_size = prize_x.max(prize_y);

    // Scale factor based on the logarithm of the prize value, with a minimum range of 100
    let scale_factor = (prize_size as f64).log(10.0).round() as u64;
    scale_factor.max(100) // Ensure the scaling factor has a minimum value of 100
}

// The genetic algorithm function with the same interface as get_x_and_y.
fn get_x_and_y(
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize_x: u64,
    prize_y: u64,
) -> Option<(u64, u64, u64)> {
    let population_range = get_population_range(prize_x, prize_y);
    let a_x = a_x as i64;
    let a_y = a_y as i64;
    let b_x = b_x as i64;
    let b_y = b_y as i64;
    let prize_x = prize_x as i64;
    let prize_y = prize_y as i64;

    fn fitness(individual: &Individual, a_x: i64, a_y: i64, b_x: i64, b_y: i64, prize_x: i64, prize_y: i64) -> u64 {
        let numerator_y = prize_x - a_x * individual.x as i64;
        if numerator_y % b_x != 0 {
            return u64::MAX; // Invalid solution, return high cost
        }
        let y = numerator_y / b_x;
        if y < 0 {
            return u64::MAX; // Invalid solution
        }

        let total_x = individual.x * a_x as u64 + y as u64 * b_x as u64;
        let total_y = individual.x * a_y as u64 + y as u64 * b_y as u64;

        let deviation_x = (total_x as i64 - prize_x).abs();
        let deviation_y = (total_y as i64 - prize_y).abs();

        let tolerance_x = if prize_x < 100 { 1 } else { (prize_x / 1000).max(1) };
        let tolerance_y = if prize_y < 100 { 1 } else { (prize_y / 1000).max(1) };

        if deviation_x <= tolerance_x && deviation_y <= tolerance_y {
            (individual.x * 3 + y as u64) + (deviation_x + deviation_y) as u64 // Include deviations as penalties
        } else {
            u64::MAX // Invalid solution
        }
    }

    let population_size = 1000;

    // Initial population
    let mut rng = StdRng::seed_from_u64(42);
    let mut population: Vec<Individual> = (0..population_size)
        .map(|_| Individual {
            x: rng.gen_range(0..=population_range),
            y: rng.gen_range(0..=population_range),
        })
        .collect();

    let mut best_solution: Option<(u64, u64, u64)> = None;
    let mut stagnation_count = 0;

    // Fixed mutation factor (not based on prize_x or prize_y)
    let base_mutation_factor = 10;
    let min_mutation_factor = 1;

    // GA loop
    loop {
        // Sort the population by fitness using Rayon parallelism
        population.par_sort_by_key(|ind| fitness(ind, a_x, a_y, b_x, b_y, prize_x, prize_y));

        // Check if the best solution is valid
        let best = &population[0];
        let best_cost = fitness(best, a_x, a_y, b_x, b_y, prize_x, prize_y);

        if best_cost != u64::MAX {
            let new_solution = Some((best.x, best.y, best_cost));
            if Some((best.x, best.y, best_cost)) != best_solution {
                best_solution = new_solution;
                stagnation_count = 0;
            } else {
                stagnation_count += 1;
            }
        }

        if stagnation_count >= 20 {
            break;
        }

        // Selection: Keep the best half of the population
        let elite_size = population_size / 10;
        let elite = population.split_at_mut(elite_size).0.to_vec();

        // Dynamically adjust mutation factor based on the cost
        let mut mutation_factor = base_mutation_factor;
        if best_cost < 500 {
            // Reduce mutation factor as the cost approaches the minimum
            mutation_factor = (mutation_factor as f64 * (best_cost as f64 / 500.0)).round() as i64;
        }
        mutation_factor = mutation_factor.max(min_mutation_factor); // Ensure minimum mutation factor

        // Crossover and mutation
        let new_population: Vec<Individual> = elite.par_iter().map(|parent| {
            let mut child = parent.clone();
            let mut rng = StdRng::from_entropy(); // Create a new RNG for each thread
            if rng.gen_bool(0.5) { // Mutation with a fixed probability
                let mutation_offset = rng.gen_range(0..mutation_factor);
                let direction = if rng.gen_bool(0.5) { 1 } else { -1 };
                child.x = (child.x as i64 + rng.gen_range(-mutation_factor..=mutation_factor) as i64).max(0) as u64;
                child.y = (child.y as i64 + rng.gen_range(-mutation_factor..=mutation_factor) as i64).max(0) as u64;
            }
            child
        }).collect();

        // Replace the old population with the new one
        population = elite.into_iter().chain(new_population.into_iter()).collect();
    }

    best_solution // Return the best found solution
}
*/

fn lowest_cost_for_mechine(mechine: &Mechine) -> u64 {
    let (a_x, a_y) = mechine.button_a;
    let (b_x, b_y) = mechine.button_b;
    let (prize_x, prize_y) = mechine.prize;

    if let Some((_, _, cost)) = get_x_and_y(a_x, a_y, b_x, b_y, prize_x, prize_y) {
        cost
    } else {
        0
    }
}

pub fn get_cost_for_all_prizes(mechines: &Vec<Mechine>) -> u64 {
    let mut cost: u64 = 0;

    for mechine in mechines {
        cost += lowest_cost_for_mechine(mechine);
    }

    return cost;
}

fn main() {
    let mechines: Vec<Mechine> = match get_mechines("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve mechines. {}", e);
            return;
        }
    };

    let cost = get_cost_for_all_prizes(&mechines);
    println!("cost for all prizes: {}", cost);
    /*
        let cost_with_precision_fix = get_cost_for_all_prizes(&fix_precision_errors(&mechines));
        println!("cost for all prizes (with precision fix): {}", cost_with_precision_fix);
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cost_for_all_prizes() {
        let mechines: Vec<Mechine> = match get_mechines("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve mechines. {}", e);
                return;
            }
        };

        let result = get_cost_for_all_prizes(&mechines);
        assert_eq!(result, 480);
    }
}
