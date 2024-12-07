// https://adventofcode.com/2024/day/1

use utils::get_csv_data;
use utils::sort;

pub fn get_lists(path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    let data = get_csv_data::<u32>(path, false)?;

    for result in data {
        if result.len() >= 2 {
            list_a.push(result[0]);
            list_b.push(result[1]);
        } else {
            return Err("CSV row has fewer than 2 columns".into());
        }
    }

    return Ok((list_a, list_b));
}

pub fn minimum_distance(list_a: &mut Vec<u32>, list_b: &mut Vec<u32>) -> u32 {
    let mut dist: u32 = 0;
    *list_a = sort(list_a);
    *list_b = sort(list_b);

    for i in 0..list_a.len() {
        let mut sorted: Vec<u32> = vec![list_a[i], list_b[i]];
        sorted = sort(&mut sorted);
        dist += sorted[1] - sorted[0];
    }

    return dist;
}

pub fn similarity_score(list_a: &[u32], list_b: &[u32]) -> u32 {
    let mut similarity: u32 = 0;

    for a in list_a {
        let mut repeated = 0;

        for b in list_b {
            if a == b {
                repeated += 1;
            }
        }

        similarity += a * repeated;
    }

    return similarity;
}

fn main() {
    match get_lists("data/input.csv") {
        Ok((mut list_a, mut list_b)) => {
            let min_distance = minimum_distance(&mut list_a, &mut list_b);
            println!("minimum distance: {}", min_distance);

            let similarity = similarity_score(&list_a, &list_b);
            println!("similarity score: {}", similarity);
        }

        Err(e) => {
            println!("Error: failed to retrieve CSV data. {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_distance() {
        match get_lists("data/test.csv") {
            Ok((mut test_a, mut test_b)) => {
                let result = minimum_distance(&mut test_a, &mut test_b);
                assert_eq!(result, 11);
            }
            Err(e) => {
                println!("Error: failed to retrieve CSV data. {}", e);
            }
        }
    }

    #[test]
    fn test_similarity_score() {
        match get_lists("data/test.csv") {
            Ok((mut test_a, mut test_b)) => {
                let result = similarity_score(&mut test_a, &mut test_b);
                assert_eq!(result, 31);
            }
            Err(e) => {
                panic!("Error: failed to retrieve CSV data. {}", e);
            }
        }
    }
}
