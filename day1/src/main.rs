// https://adventofcode.com/2024/day/1

use csv::ReaderBuilder;
use utils::*;

pub fn get_lists(path: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        let row: Vec<u32> = record.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        list_a.push(row[0]);
        list_b.push(row[1]);
    }

    return (list_a, list_b);
}

pub fn minimum_distance(mut list_a: Vec<u32>, mut list_b: Vec<u32>) -> u32 {
    let mut dist: u32 = 0;
    list_a = sort(list_a);
    list_b = sort(list_b);

    for i in 0..list_a.len() {
        let sorted: Vec<u32> = sort(vec![list_a[i], list_b[i]]);
        dist += sorted[1] - sorted[0];
    }

    return dist;
}

pub fn similarity_score(mut list_a: Vec<u32>, mut list_b: Vec<u32>) -> u32 {
    let mut similarity: u32 = 0;
    list_a = sort(list_a);
    list_b = sort(list_b);

    for i in 0..list_a.len() {
        let mut repeated = 0;        
        
        for j in 0..list_b.len() {
            if list_a[i] == list_b[j] {
                repeated += 1;
            }
        }

        similarity += list_a[i] * repeated;
    }

    return similarity;
}

fn main() {
    let list_a;
    let list_b;
    (list_a, list_b) = get_lists("data/input.csv");

    let min_distance = minimum_distance(list_a.clone(), list_b.clone());
    println!("minimum distance: {}", min_distance);

    let similarity = similarity_score(list_a, list_b);
    println!("similarity_score: {}", similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_distance() {
        let test_a;
        let test_b;
        (test_a, test_b) = get_lists("data/test.csv");

        let result = minimum_distance(test_a, test_b);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_similarity_score() {
        let test_a;
        let test_b;
        (test_a, test_b) = get_lists("data/test.csv");

        let result = similarity_score(test_a, test_b);
        assert_eq!(result, 31);
    }
}
