// https://adventofcode.com/2024/day/2

use utils::get_csv_data;
use utils::sort;

fn is_safe(report: Vec<u32>) -> bool {
    let mut flip: bool = false;

    for i in 0..(report.len() - 1) {
        if i == 0 {
            flip = report[i] > report[i + 1];
        }
        if report[i] == report[i + 1] {
            return false;
        }

        if i > 0 && flip != (report[i] > report[i + 1]) {
            return false;
        }

        let mut sorted_vec: Vec<u32> = vec![report[i], report[i + 1]];
        let sorted: &mut Vec<u32> = &mut sorted_vec;
        *sorted = sort(sorted);
        if sorted[1] - sorted[0] > 3 {
            return false;
        }
    }

    return true;
}

pub fn reactors_safe(reports: &Vec<Vec<u32>>, dampener_enabled: bool) -> u32 {
    let mut safe_reactors: u32 = 0;

    for report in reports {
        let safe = is_safe(report.clone());

        if safe {
            safe_reactors += 1;
        } else if dampener_enabled {
            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                let safe_after_removal = is_safe(modified_report);

                if safe_after_removal {
                    safe_reactors += 1;
                    break;
                }
            }
        }
    }

    return safe_reactors;
}

fn main() {
    let reports = get_csv_data("data/input.csv", false);
    let safe_reactors = reactors_safe(&reports, false);
    println!("save reactors: {}", safe_reactors);

    let safe_reactors_with_dampener = reactors_safe(&reports, true);
    println!("save reactors with dampener: {}", safe_reactors_with_dampener);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactors_safe() {
        let reports = get_csv_data("data/test.csv", false);
        let result = reactors_safe(&reports, false);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_reactors_safe_with_dampener() {
        let reports = get_csv_data("data/test.csv", false);
        let result = reactors_safe(&reports, true);
        assert_eq!(result, 4);
    }
}
