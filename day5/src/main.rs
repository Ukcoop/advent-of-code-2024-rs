// https://adventofcode.com/2024/day/5

use std::collections::HashMap;
use utils::get_csv_data;

fn get_rules(path: &str) -> HashMap<String, bool> {
    let mut rules = HashMap::new();
    let rules_vec: Vec<Vec<u32>> = get_csv_data(path, false);
    
    for rule in rules_vec {
        rules.insert(format!("{}|{}", rule[0], rule[1]), true);
    }

    return rules;
}

fn is_in_order(rules: &HashMap<String, bool>, update: &[u32]) -> bool {
    for i in 0..update.len() {
        for j in 0..update.len() {
            if i == j { continue }
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
fn sort_update(rules: &HashMap<String, bool>, mut update: Vec<u32>) -> Vec<u32> {
    if update.len() <= 1 {
        return update;
    } 

    let pivot = update.pop().unwrap();

    let mut less_than_pivot: Vec<u32> = Vec::new();
    let mut greater_than_pivot: Vec<u32> = Vec::new();

    for x in update {
        let possible_rule = format!("{}|{}", x, pivot);
        if rules.contains_key(&possible_rule) {
            less_than_pivot.push(x);
        } else {
            greater_than_pivot.push(x);
        }
    }

    let mut sorted = sort_update(rules, less_than_pivot);
    sorted.push(pivot);
    sorted.extend(sort_update(rules, greater_than_pivot));

    return sorted;
}

pub fn count_of_middle_numbers(rules: &HashMap<String, bool>, updates: &[Vec<u32>]) -> (u32, u32) {
    let mut sorted_count: u32 = 0;
    let mut unsorted_count: u32 = 0;

    for update in updates {
        if is_in_order(rules, update) {
            sorted_count += update[update.len() / 2];
        } else {
            let mut sorted_update: Vec<u32> = update.to_vec();
            sorted_update = sort_update(rules, sorted_update);
            unsorted_count += sorted_update[sorted_update.len() / 2];
        }
    }

    return (sorted_count, unsorted_count);
}

fn main() {
    let rules = get_rules("data/input/rules.csv");
    let updates: Vec<Vec<u32>> = get_csv_data("data/input/updates.csv", false);

    let (sorted_count, unsorted_count) = count_of_middle_numbers(&rules, &updates);
    println!("sorted count of middle numbers: {}", sorted_count);
    println!("sorted count of middle numbers: {}", unsorted_count);    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_of_middle_numbers() {
        let rules = get_rules("data/test/rules.csv");
        let updates: Vec<Vec<u32>> = get_csv_data("data/test/updates.csv", false);

        let (sorted_count, unsorted_count) = count_of_middle_numbers(&rules, &updates);
        assert_eq!(sorted_count, 143);
        assert_eq!(unsorted_count, 123);
    }
}
