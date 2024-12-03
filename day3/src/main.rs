use utils::get_csv_data;
fn new_toggle(old_toggle: bool, test_string: String) -> bool {
    let do_length = test_string.split("do()").last().unwrap().len();
    let dont_length = test_string.split("don't()").last().unwrap().len();
    if do_length == dont_length { return old_toggle; }
    if do_length < dont_length { return true; }
    return false;
}


pub fn total_in_line(corrupted_string: String, mul_toggle: bool) -> u32 {
    let mut total: u32 = 0;
    let mut mul_toggled = true;

    let first_pass = corrupted_string
        .split("mul")
        .skip(1)
        .collect::<Vec<&str>>();

    for part in first_pass {
        let previous_toggle: bool = mul_toggled;
        mul_toggled = new_toggle(mul_toggled.clone(), part.to_string());
        if !previous_toggle && mul_toggle { continue; }

        let mut mul = part.split(")").next().unwrap_or(""); // (100,100 
        if mul.len() > 8 || mul.len() < 4 { continue; } // between (1,1 and (100,100 
        mul = mul.split("(").nth(1).unwrap_or(""); // 100,100
        
        let numbers = mul.split(",").collect::<Vec<&str>>(); // ["100", "100"]
        if numbers.len() == 2 {
            if let (Ok(a), Ok(b)) = (numbers[0].parse::<u32>(), numbers[1].parse::<u32>()) {
                total += a * b;
            }
        }
    }
    
    return total;
}


fn main() {
    let corrupted_code_segements: Vec<Vec<String>> = get_csv_data("data/input.csv", false);
    let mut full_corrupted_code: String = "".to_string();

    for code in corrupted_code_segements {
        full_corrupted_code += &code.join(",").to_string();
    }
    
    let total_without_toggle = total_in_line(full_corrupted_code.clone(), false);
    println!("without toggle: {}", total_without_toggle);
    
    let total_with_toggle = total_in_line(full_corrupted_code.clone(), true);
    println!("with toggle: {}", total_with_toggle);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_total_in_line() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = total_in_line(input.to_string(), false);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_total_in_line_with_toggle() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = total_in_line(input.to_string(), true);
        assert_eq!(result, 48);
    }
}
