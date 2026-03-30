use std::env;

mod constants;
mod usage;

pub fn parse_input() -> (u128, u128, bool) {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "help" {
        usage::print_usage();
        return (0, 0, true);
    }

    if args.len() > 3 {
        usage::print_too_many_args_warning();
    }

    let min = (&args[1]).to_string();
    let max = match args.len() {
        x if x > 2 => (&args[2]).to_string(),
        _ => min.clone(),
    };

    let min = match min.trim().parse::<u128>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error! Min is not valid: {e}");
            return (0, 0, true);
        }
    };
    let max = match max.trim().parse::<u128>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error! Max is not valid: {e}");
            return (0, 0, true);
        }
    };

    if min > max {
        println!("Error! Min must be less or equal than Max.");
        return (0, 0, true);
    }

    if min == max {
        println!("number: {min}\n");
    } else {
        println!("min: {min}");
        println!("max: {max}\n");
    }

    (min, max, false)
}

fn split_triplets(num: u128) -> impl Iterator<Item = usize> {
    let mut remaining = num;
    std::iter::from_fn(move || {
        if remaining > 0 {
            let triplet = (remaining % 1000) as usize;
            remaining /= 1000;
            Some(triplet)
        } else {
            None
        }
    })
}

fn elaborate_hundreds_digit(hundreds_digit: usize, tens_digit: usize) -> String {
    match hundreds_digit {
        0 => String::new(),
        1 => {
            if tens_digit == 8 {
                "cent".into()
            } else {
                "cento".into()
            }
        }
        n => format!(
            "{}cent{}",
            constants::UNDER_TWENTY[n],
            if tens_digit == 8 { "" } else { "o" }
        ),
    }
}

fn elaborate_tens_digit(tens_digit: usize, units_digit: usize) -> String {
    let word = constants::TENS[tens_digit];
    match units_digit {
        1 | 8 => word[..word.len() - 1].to_string(),
        _ => word.to_string(),
    }
}

fn elaborate_units_word(units_digit: usize) -> String {
    match units_digit {
        0 => String::new(),
        n => constants::UNDER_TWENTY[n].to_string(),
    }
}

fn triplet_to_word(triplet: usize, triplet_index: usize) -> String {
    // Magnitude for 'mila', 'milioni', 'miliardi', ...
    let magnitude_ending = if triplet == 1 {
        if triplet_index % 2 == 0 {
            "e "
        } else {
            "o "
        }
    } else {
        "i "
    };
    let magnitude = match triplet_index {
        1 => "mila".to_string(),
        2 => format!(" milion{magnitude_ending}"),
        3 => format!(" miliard{magnitude_ending}"),
        4 => format!(" bilion{magnitude_ending}"),
        5 => format!(" biliard{magnitude_ending}"),
        6 => format!(" trilion{magnitude_ending}"),
        7 => format!(" triliard{magnitude_ending}"),
        8 => format!(" quadrilion{magnitude_ending}"),
        9 => format!(" quadriliard{magnitude_ending}"),
        10 => format!(" quintilion{magnitude_ending}"),
        11 => format!(" quintiliard{magnitude_ending}"),
        12 => format!(" sestilion{magnitude_ending}"),
        _ => String::new(),
    };

    match triplet {
        0 => String::new(),
        1 => match triplet_index {
            0 => "uno".to_string(),
            1 => "mille".to_string(),
            _ => format!("un{magnitude}"),
        },
        _ => {
            let hundreds_digit = triplet / 100;
            let tens_digit = triplet / 10 % 10;
            let units_digit = triplet % 10;
            let two_digit_remainder = triplet % 100;

            let body = if two_digit_remainder < 20 {
                format!(
                    "{}{}",
                    elaborate_hundreds_digit(hundreds_digit, tens_digit),
                    constants::UNDER_TWENTY[two_digit_remainder],
                )
            } else {
                format!(
                    "{}{}{}",
                    elaborate_hundreds_digit(hundreds_digit, tens_digit),
                    elaborate_tens_digit(tens_digit, units_digit),
                    elaborate_units_word(units_digit),
                )
            };

            format!("{body}{magnitude}")
        }
    }
}

pub fn loop_numbers(min: u128, max: u128) {
    let mut max_len: usize = 0;
    let mut longest_numbers: Vec<String> = vec![];

    for number in min..=max {
        let number_word = if number == 0 {
            "zero".to_string()
        } else {
            split_triplets(number)
                .enumerate()
                .map(|(i, t)| triplet_to_word(t, i))
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>()
        };

        println!("{number} = {number_word}");

        if min != max {
            match number_word.len().cmp(&max_len) {
                std::cmp::Ordering::Greater => {
                    max_len = number_word.len();
                    longest_numbers = vec![number_word];
                }
                std::cmp::Ordering::Equal => longest_numbers.push(number_word),
                std::cmp::Ordering::Less => {}
            }
        }
    }

    if min != max {
        println!(
            "\nLongest numbers ({} found, length {}):",
            longest_numbers.len(),
            max_len
        );
        for word in &longest_numbers {
            println!("  {word}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_triplets() {
        let result: Vec<usize> = split_triplets(1234567890).collect();
        assert_eq!(result, vec![890, 567, 234, 1]);
    }

    #[test]
    fn test_elaborate_hundreds_digit() {
        let result = elaborate_hundreds_digit(1, 2);
        assert_eq!(result, "cento");
    }
    #[test]
    fn test_elaborate_hundreds_digit_eight() {
        let result = elaborate_hundreds_digit(1, 8);
        assert_eq!(result, "cent");
    }

    #[test]
    fn test_elaborate_tens_digit() {
        let result = elaborate_tens_digit(2, 5);
        assert_eq!(result, "venti");
    }

    #[test]
    fn test_elaborate_tens_digit_one() {
        let result = elaborate_tens_digit(2, 1);
        assert_eq!(result, "vent");
    }

    #[test]
    fn test_elaborate_tens_digit_eigth() {
        let result = elaborate_tens_digit(2, 8);
        assert_eq!(result, "vent");
    }

    #[test]
    fn test_elaborate_units_word() {
        let result = elaborate_units_word(5);
        assert_eq!(result, "cinque");
    }

    #[test]
    fn test_triplet_to_word() {
        let result = triplet_to_word(123, 0);
        assert_eq!(result, "centoventitre");
    }

    #[test]
    fn test_triplet_to_word_zero() {
        let result = triplet_to_word(0, 0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_triplet_to_word_188_mrd() {
        let result = triplet_to_word(188, 3);
        assert_eq!(result, "centottantotto miliardi ");
    }

    #[test]
    fn test_loop_numbers() {
        // This function prints output, so you might want to capture the output and test it.
        // For simplicity, we are just calling it here.
        loop_numbers(1, 2);
    }
}
