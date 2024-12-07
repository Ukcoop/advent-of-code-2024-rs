// https://adventofcode.com/2024/day/5

use std::collections::HashMap;
use std::error::Error;

use utils::get_csv_data;

fn get_rules(path: &str) -> Result<HashMap<String, bool>, Box<dyn Error>> {
    let mut rules = HashMap::new();

    let rules_vec: Vec<Vec<u32>> = get_csv_data(path, false)?;

    for rule in rules_vec {
        rules.insert(format!("{}|{}", rule[0], rule[1]), true);
    }

    return Ok(rules);
}

fn is_in_order(rules: &HashMap<String, bool>, update: &[u32]) -> bool {
    for i in 0..update.len() {
        for j in 0..update.len() {
            if i == j {
                continue;
            }
            let possible_rule = if i > j {
                format!("{}|{}", update[i], update[j])
            } else {
                format!("{}|{}", update[j], update[i])
            };

            if rules.contains_key(&possible_rule) {
                return false;
            }
        }
    }

    return true;
}

// i'm proud of this! i adapted an algorithm to custom sorting rules!
fn sort_update(rules: &HashMap<String, bool>, update: &mut Vec<u32>) {
    if update.len() <= 1 {
        return;
    }

    let pivot = update.pop().unwrap_or(0);
    let mut less_than_pivot: Vec<u32> = Vec::new();
    let mut greater_than_pivot: Vec<u32> = Vec::new();

    for &x in update.iter() {
        let possible_rule = format!("{}|{}", x, pivot);
        if rules.contains_key(&possible_rule) {
            less_than_pivot.push(x);
        } else {
            greater_than_pivot.push(x);
        }
    }

    sort_update(rules, &mut less_than_pivot);
    sort_update(rules, &mut greater_than_pivot);

    update.clear();
    update.extend(less_than_pivot);
    update.push(pivot);
    update.extend(greater_than_pivot);
}

pub fn count_of_middle_numbers(rules: &HashMap<String, bool>, updates: &mut [Vec<u32>]) -> (u32, u32) {
    let mut sorted_count: u32 = 0;
    let mut unsorted_count: u32 = 0;

    for update in updates {
        if is_in_order(rules, update) {
            sorted_count += update[update.len() / 2];
        } else {
            sort_update(rules, update);
            unsorted_count += update[update.len() / 2];
        }
    }

    return (sorted_count, unsorted_count);
}

fn main() {
    let rules = match get_rules("data/input/rules.csv") {
        Ok(rules) => rules,
        Err(e) => {
            println!("Error: Failed to retrieve rules data. {}", e);
            return;
        }
    };

    let mut updates = match get_csv_data("data/input/updates.csv", false) {
        Ok(updates) => updates,
        Err(e) => {
            println!("Error: Failed to retrieve updates data. {}", e);
            return;
        }
    };

    let (sorted_count, unsorted_count) = count_of_middle_numbers(&rules, &mut updates);
    println!("sorted count of middle numbers: {}", sorted_count);
    println!("unsorted count of middle numbers: {}", unsorted_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_of_middle_numbers() {
        let rules: HashMap<String, bool> = match get_rules("data/test/rules.csv") {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve rules data. {}", e);
            }
        };

        let mut updates: Vec<Vec<u32>> = match get_csv_data("data/test/updates.csv", false) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve updates data. {}", e);
            }
        };

        let (sorted_count, unsorted_count) = count_of_middle_numbers(&rules, &mut updates);
        assert_eq!(sorted_count, 143);
        assert_eq!(unsorted_count, 123);
    }
}
