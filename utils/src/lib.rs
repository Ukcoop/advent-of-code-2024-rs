use std::str::FromStr;

use csv::ReaderBuilder;

// standard CS algorithms
pub fn sort(list: &mut Vec<u32>) -> Vec<u32> {
    if list.len() <= 1 {
        return list.clone();
    }

    let pivot = match list.pop() {
        Some(value) => value,
        None => return Vec::new(),
    };

    let mut less_than_pivot = Vec::new();
    let mut greater_than_pivot = Vec::new();

    for &x in list.iter() {
        if x <= pivot {
            less_than_pivot.push(x);
        } else {
            greater_than_pivot.push(x);
        }
    }

    let mut sorted = sort(&mut less_than_pivot);
    sorted.push(pivot);
    sorted.extend(sort(&mut greater_than_pivot));

    return sorted;
}

// data processing
pub fn get_csv_data<T>(path: &str, headers: bool) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let mut data: Vec<Vec<T>> = Vec::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(headers)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        let row: Result<Vec<T>, _> = record.iter().map(|s| s.parse::<T>()).collect();

        data.push(row?);
    }

    return Ok(data);
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
        // Handling the result from get_csv_data
        let int_csv: Vec<Vec<u32>> = match get_csv_data("data/testInt.csv", false) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve int CSV data. {}", e);
            }
        };

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

        let string_csv: Vec<Vec<String>> = match get_csv_data("data/testString.csv", true) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: Failed to retrieve string CSV data. {}", e);
            }
        };

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
