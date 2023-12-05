use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_sum = 0u32;

    for file_line in input.lines() {
        let split = file_line
            .find("|")
            .expect("Could not find pattern delimiter");

        let winning_numbers = file_line[0..split]
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Could not parse value"))
            .collect::<HashSet<i32>>();

        let mut line_sum = None;

        file_line[(split + 1)..]
            .split_ascii_whitespace()
            .for_each(|value| {
                let value = value.parse::<i32>().expect("Could not parse value");
                if winning_numbers.contains(&value) {
                    line_sum = match line_sum.take() {
                        Some(line_sum) => Some(line_sum * 2),
                        None => Some(1),
                    }
                }
            });

        total_sum += line_sum.unwrap_or(0);
    }

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_of_cards = input.lines().count();
    let mut instances = vec![1u32; num_of_cards];

    for (card_idx, file_line) in input.lines().enumerate() {
        let split = file_line
            .find("|")
            .expect("Could not find pattern delimiter");

        let winning_numbers = file_line[0..split]
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Could not parse value"))
            .collect::<HashSet<i32>>();

        let number_of_matches: usize = file_line[(split + 1)..]
            .split_ascii_whitespace()
            .map(|value| {
                let value = value.parse::<i32>().expect("Could not parse value");
                if winning_numbers.contains(&value) {
                    1
                } else {
                    0
                }
            })
            .sum();

        for offset in 1..=number_of_matches {
            let new_idx = card_idx + offset;
            instances[new_idx] += instances[card_idx];
        }
    }

    Some(instances.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
