// https://adventofcode.com/2024/day/9

use rayon::prelude::*;

use utils::get_csv_data;

#[derive(Clone)]
struct OpenBlock {
    index: usize,
    length: usize,
}

fn disk_from_disk_map(disk_map: &[String]) -> Vec<String> {
    let mut disk: Vec<String> = Vec::new();

    for i in 0..disk_map.len() {
        let digit: u32 = disk_map[i].parse::<u32>().unwrap_or(0);

        for _ in 0..digit {
            if (i - 1) % 2 == 0 {
                disk.push(format!("{}", (i - 1) / 2));
            } else {
                disk.push(".".to_string());
            }
        }
    }

    return disk;
}

fn compress_disk(mut disk: Vec<String>) -> Vec<String> {
    let mut available_spots: Vec<usize> = Vec::new();

    for i in 0..disk.len() {
        if disk[i] == *"." {
            available_spots.push(i);
        }
    }

    for i in (0..disk.len()).rev() {
        if disk[i] != *"." {
            if let Some(j) = available_spots
                .par_iter()
                .enumerate()
                .filter(|&(_, &spot)| spot < i)
                .min_by_key(|&(_, &spot)| spot)
                .map(|(index, _)| index)
            {
                disk[available_spots[j]] = disk[i].clone();
                disk[i] = ".".to_string();

                available_spots[j] = i;
            }
        }
    }

    return disk;
}

fn compress_disk_in_blocks(mut disk: Vec<String>) -> Vec<String> {
    let mut available_spots: Vec<usize> = Vec::new();
    let mut available_block_spots: Vec<OpenBlock> = Vec::new();
    let mut spot_length: usize = 1;

    for i in 0..disk.len() {
        if disk[i] == *"." {
            available_spots.push(i);
        }
    }

    for i in 0..(available_spots.len() - 1) {
        if (available_spots[i] + 1) == available_spots[i + 1] {
            spot_length += 1;
        } else {
            let block_spot = OpenBlock {
                index: available_spots[i] - (spot_length - 1),
                length: spot_length,
            };

            available_block_spots.push(block_spot);
            spot_length = 1;
        }
    }

    let mut i: usize = disk.len() - 1;
    while i > 0 {
        if disk[i] != *"." {
            let mut block_length: usize = 0;

            while i >= block_length && disk[i - block_length] == disk[i] {
                block_length += 1;
            }

            if let Some(j) = available_block_spots
                .par_iter()
                .enumerate()
                .filter(|&(_, block)| block.index < i && block.length >= block_length)
                .min_by_key(|&(_, block)| block.index)
                .map(|(index, _)| index)
            {
                let new_index = available_block_spots[j].index + block_length - 1;
                for k in 0..block_length {
                    disk[new_index - k] = disk[i - k].clone();
                    disk[i - k] = ".".to_string();
                }

                if available_block_spots[j].length > block_length {
                    available_block_spots[j].index += block_length;
                    available_block_spots[j].length -= block_length;
                } else {
                    available_block_spots.remove(j);
                }

                let new_block = OpenBlock {
                    index: i,
                    length: block_length,
                };

                available_block_spots.push(new_block);
            }

            if i > block_length {
                i -= block_length;
            } else {
                i = 0;
            }
        } else {
            i -= 1;
        }
    }

    return disk;
}

fn get_cheksum_of_disk(disk: Vec<String>) -> u64 {
    let mut checksum: u64 = 0;

    for i in 0..disk.len() {
        if disk[i] != *"." {
            checksum += (i as u64) * disk[i].parse::<u64>().unwrap_or(0);
        }
    }

    return checksum;
}

fn main() {
    let mut input: Vec<String> = match get_csv_data("data/input.csv", false) {
        Ok(result) => result[0].clone(),
        Err(e) => {
            println!("Error: Failed to retrieve disk map. {}", e);
            return;
        }
    };

    input = input[0].split("").map(String::from).collect();
    let checksum_of_disk = get_cheksum_of_disk(compress_disk(disk_from_disk_map(&input)));
    println!("checksum of disk: {:?}", checksum_of_disk);

    let checksum_of_disk_in_blocks = get_cheksum_of_disk(compress_disk_in_blocks(disk_from_disk_map(&input)));
    println!(
        "checksum of disk (compressed in blocks): {:?}",
        checksum_of_disk_in_blocks
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cheksum_of_disk() {
        let mut input: Vec<String> = match get_csv_data("data/test.csv", false) {
            Ok(result) => result[0].clone(),
            Err(e) => {
                println!("Error: Failed to retrieve disk map. {}", e);
                return;
            }
        };

        input = input[0].split("").map(String::from).collect();
        let result = get_cheksum_of_disk(compress_disk(disk_from_disk_map(&input)));

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_get_cheksum_of_disk_in_blocks() {
        let mut input: Vec<String> = match get_csv_data("data/test.csv", false) {
            Ok(result) => result[0].clone(),
            Err(e) => {
                println!("Error: Failed to retrieve disk map. {}", e);
                return;
            }
        };

        input = input[0].split("").map(String::from).collect();
        let result = get_cheksum_of_disk(compress_disk_in_blocks(disk_from_disk_map(&input)));

        assert_eq!(result, 2858);
    }
}
