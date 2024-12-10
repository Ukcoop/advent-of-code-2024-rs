// https://adventofcode.com/2024/day/7

use rayon::prelude::*;
use std::error::Error;

use utils::get_csv_data;

pub struct Calibration {
    evaluating_to: u64,
    equation: Vec<u64>,
}

pub fn get_calibrations(path: &str) -> Result<Vec<Calibration>, Box<dyn Error>> {
    let calibration_strings: Vec<Vec<String>> = get_csv_data(path, false)?;
    let mut calibrations: Vec<Calibration> = Vec::new();

    for calibration_string in calibration_strings {
        let parts: Vec<&str> = calibration_string[0].split(":").collect();

        if parts.len() != 2 {
            return Err("Expected exactly two parts, but got one. (get_calibrations)".into());
        }

        let (evaluating_to, equation) = (parts[0], parts[1]);

        let calibration = Calibration {
            evaluating_to: evaluating_to.parse::<u64>().unwrap_or(1),
            equation: equation
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect(),
        };

        calibrations.push(calibration);
    }

    return Ok(calibrations);
}

fn concatinate_numbers(a: &u64, b: &u64) -> u64 {
    let number_string = format!("{}{}", a, b);
    return number_string.parse::<u64>().unwrap_or(0);
}

fn possible_equations_for_calibration(
    calibration: &Calibration,
    compare: u64,
    mut count: u64,
    concatenation: &bool,
) -> u64 {
    if calibration.equation.len() > 1 {
        let mut new_equation = calibration.equation.clone();
        new_equation.remove(0);

        let new_calibration = Calibration {
            evaluating_to: calibration.evaluating_to,
            equation: new_equation,
        };

        count = possible_equations_for_calibration(
            &new_calibration,
            compare + calibration.equation[0],
            count,
            concatenation,
        );
        count = possible_equations_for_calibration(
            &new_calibration,
            compare * calibration.equation[0],
            count,
            concatenation,
        );
        if *concatenation {
            count = possible_equations_for_calibration(
                &new_calibration,
                concatinate_numbers(&compare, &calibration.equation[0]),
                count,
                concatenation,
            );
        }

        return count;
    } else {
        if compare + calibration.equation[0] == calibration.evaluating_to {
            count += 1
        }
        if compare * calibration.equation[0] == calibration.evaluating_to {
            count += 1
        }
        if *concatenation && concatinate_numbers(&compare, &calibration.equation[0]) == calibration.evaluating_to {
            count += 1;
        }

        return count;
    }
}

pub fn total_calibration_result(calibrations: &Vec<Calibration>, concatenation: bool) -> u64 {
    return calibrations
        .par_iter()
        .map(|calibration| {
            let possible = possible_equations_for_calibration(calibration, 0, 0, &concatenation);
            if possible > 0 {
                calibration.evaluating_to
            } else {
                0
            }
        })
        .sum();
}

fn main() {
    let calibrations = match get_calibrations("data/input.csv") {
        Ok(result) => result,
        Err(e) => {
            println!("Error: Failed to retrieve calibrations. {}", e);
            return;
        }
    };

    let calibration_result = total_calibration_result(&calibrations, false);
    println!("calibration result: {}", calibration_result);

    let calibration_result_with_concatenation = total_calibration_result(&calibrations, true);
    println!(
        "calibration result with concatenation: {}",
        calibration_result_with_concatenation
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_calibration_result() {
        let calibrations = match get_calibrations("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve calibrations. {}", e);
                return;
            }
        };

        let result = total_calibration_result(&calibrations, false);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_total_calibration_result_with_concatenation() {
        let calibrations = match get_calibrations("data/test.csv") {
            Ok(result) => result,
            Err(e) => {
                println!("Error: Failed to retrieve calibrations. {}", e);
                return;
            }
        };

        let result = total_calibration_result(&calibrations, true);
        assert_eq!(result, 11387);
    }
}
