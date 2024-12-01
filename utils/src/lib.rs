pub fn sort(mut list: Vec<u32>) -> Vec<u32> {
    if list.len() <= 1 {
        return list;
    }

    let pivot = list.pop().unwrap();

    let mut less_than_pivot = Vec::new();
    let mut greater_than_pivot = Vec::new();

    for x in list {
        if x <= pivot {
            less_than_pivot.push(x);
        } else {
            greater_than_pivot.push(x);
        }
    }

    let mut sorted = sort(less_than_pivot);
    sorted.push(pivot);
    sorted.extend(sort(greater_than_pivot));
    
    return sorted;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let list = vec![34, 7, 23, 32, 5, 62];
        let result = sort(list);
        assert_eq!(result, vec![5, 7, 23, 32, 34, 62]);
    }
}
