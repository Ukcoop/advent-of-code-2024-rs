use std::fmt::Debug;
use std::str::FromStr;

use csv::ReaderBuilder;

// standard CS algorithms
pub fn sort(list: &mut Vec<u32>) -> Vec<u32> {
    if list.len() <= 1 {
        return list.to_vec();
    }

    // this unwrap is here because in certian it will not panic
    let pivot = list.pop().unwrap();

    let less_than_pivot: &mut Vec<u32> = &mut Vec::new();
    let greater_than_pivot: &mut Vec<u32> = &mut Vec::new();

    for x in list {
        if *x <= pivot {
            less_than_pivot.push(*x);
        } else {
            greater_than_pivot.push(*x);
        }
    }

    let mut sorted = sort(less_than_pivot);
    sorted.push(pivot);
    sorted.extend(sort(greater_than_pivot));

    return sorted;
}

// data processing
pub fn get_csv_data<T>(path: &str, headers: bool) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut data: Vec<Vec<T>> = Vec::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(headers)
        .flexible(true)
        .from_path(path)
        .expect("Failed to open CSV file");

    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        let row = record
            .iter()
            .map(|s| s.parse::<T>().expect("Failed to parse value"))
            .collect();
        data.push(row);
    }

    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    // standard CS algorithms
    #[test]
    fn test_sort() {
        let mut list_vec = vec![34, 7, 23, 32, 5, 62];
        let list: &mut Vec<u32> = &mut list_vec;
        let result = sort(list);
        assert_eq!(result, vec![5, 7, 23, 32, 34, 62]);
    }

    // data processing
    #[test]
    fn test_get_csv_data() {
        let int_csv: Vec<Vec<u32>> = get_csv_data("data/testInt.csv", false);
        assert_eq!(
            int_csv,
            vec![
                vec![1, 2, 3, 4, 5],
                vec![2, 3, 4, 5, 1],
                vec![3, 4, 5, 1, 2],
                vec![4, 5, 1, 2, 3],
                vec![5, 1, 2, 3, 4],
            ]
        );

        let string_csv: Vec<Vec<String>> = get_csv_data("data/testString.csv", true);
        assert_eq!(
            string_csv,
            vec![
                vec!["jhon doe", "anonymity"],
                vec!["alexander", "steam deck"],
                vec!["bob", "lawn mower"],
                vec!["alice", "baking sheets"]
            ]
        );
    }
}
