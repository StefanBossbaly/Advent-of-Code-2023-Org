advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    fn find_first_digit<I: Iterator<Item = char>>(mut chars: I) -> u32 {
        chars
            .find(|value| value.is_numeric())
            .expect("Could not find digit")
            .to_string()
            .parse::<u32>()
            .expect("Could not convert to digit")
    }

    let sum = input
        .lines()
        .map(|line| {
            let first_digit = find_first_digit(line.chars());
            let second_digit = find_first_digit(line.chars().rev());
            (first_digit * 10) + second_digit
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    const DIGITS: [(&'static str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    const DIGITS_REV: [(&'static str, u32); 9] = [
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ];

    fn find_first_digit(chars: &[char], digits: &[(&'static str, u32); 9]) -> u32 {
        for idx in 0..chars.len() {
            let current_char = chars[idx];

            match current_char.is_numeric() {
                true => {
                    return current_char
                        .to_string()
                        .parse::<u32>()
                        .expect("Could not convert to digit")
                }
                false => {
                    let maybe_digit = digits.iter().find_map(|(name, value)| {
                        if chars[idx..].starts_with(&name.chars().collect::<Vec<char>>()) {
                            Some(*value)
                        } else {
                            None
                        }
                    });

                    if let Some(digit) = maybe_digit {
                        return digit;
                    }
                }
            }
        }

        panic!("Could not find digit!");
    }

    let sum = input
        .lines()
        .map(|line| {
            let first_digit = find_first_digit(&line.chars().collect::<Vec<char>>(), &DIGITS);
            let second_digit =
                find_first_digit(&line.chars().rev().collect::<Vec<char>>(), &DIGITS_REV);
            (first_digit * 10) + second_digit
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
