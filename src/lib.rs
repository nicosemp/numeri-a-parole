use std::env;

mod constants;
mod usage;

/// Read command line arguments
pub fn read_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "help" {
        usage::print_usage();
        std::process::exit(1);
    }

    if args.len() > 3 {
        usage::print_too_many_args_warning();
    }

    if args.len() == 3 {
        println!("2 ARGUMENTS NOT YET SUPPORTED!");
        std::process::exit(1);
    }

    args[1].clone() //, args[2].clone())
}

pub fn get_number_sign(number_str: &str) -> (String, bool) {
    let first_char = &number_str[..1];
    if first_char == "-" {
        let number_without_sign = number_str.chars().skip(1).collect();
        return (number_without_sign, true);
    }

    (number_str.to_string(), false)
}

pub fn parse_string_to_numbers(number_str: &str) -> Vec<u128> {
    let mut number_str = number_str;
    let mut number_parts: Vec<u128> = vec![];

    while !number_str.is_empty() {
        let split_pos = if number_str.len() > 30 {
            number_str.len() - 30
        } else {
            0
        };

        let last_thirty_chars = &number_str[split_pos..];

        let number_part = match last_thirty_chars.trim().parse::<u128>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error! Number is not valid: {e}");
                std::process::exit(1);
            }
        };

        number_parts.push(number_part);

        number_str = &number_str[..split_pos];
    }

    number_parts
}

fn split_triplets(num: u128) -> Vec<usize> {
    let mut remaining = num;
    let mut triplets: Vec<usize> = vec![];

    while remaining > 0 {
        let triplet: usize = (remaining % 1000).try_into().unwrap();
        triplets.push(triplet);
        remaining /= 1000;
    }

    triplets
}

fn elaborate_hundreds_digit(hundreds_digit: usize, tens_digit: usize) -> String {
    if hundreds_digit == 0 {
        return "".to_string();
    }

    let mut hundred_word = "".to_owned();
    if hundreds_digit != 1 {
        hundred_word.push_str(constants::UNDER_TWENTY[hundreds_digit]);
    };
    hundred_word.push_str("cent");
    if tens_digit != 8 {
        hundred_word.push_str("o")
    };

    return hundred_word;
}

fn elaborate_tens_digit(tens_digit: usize, units_digit: usize) -> String {
    let word = constants::TENS[tens_digit];
    if units_digit == 1 || units_digit == 8 {
        return (&word[..word.len() - 1]).to_string();
    } else {
        return word.to_string();
    }
}

fn elaborate_units_word(units_digit: usize) -> String {
    if units_digit == 0 {
        return "".to_string();
    };

    return constants::UNDER_TWENTY[units_digit].to_string();
}

fn triplet_to_word(triplet: usize, triplet_index: usize) -> String {
    let mut words: Vec<&str> = vec![];

    // Magnitude for 'mila', 'milioni', 'miliardi', ...
    let magnitude_ending_char = if triplet == 1 {
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
        2 => [" milion", magnitude_ending_char].concat(),
        3 => [" miliard", magnitude_ending_char].concat(),
        4 => [" bilion", magnitude_ending_char].concat(),
        5 => [" biliard", magnitude_ending_char].concat(),
        6 => [" trilion", magnitude_ending_char].concat(),
        7 => [" triliard", magnitude_ending_char].concat(),
        8 => [" quadrilion", magnitude_ending_char].concat(),
        9 => [" quadriliard", magnitude_ending_char].concat(),
        10 => [" quintilion", magnitude_ending_char].concat(),
        11 => [" quintiliard", magnitude_ending_char].concat(),
        12 => [" sestilion", magnitude_ending_char].concat(),
        13 => [" settiliard", magnitude_ending_char].concat(),
        14 => [" ottilion", magnitude_ending_char].concat(),
        15 => [" ottiliard", magnitude_ending_char].concat(),
        16 => [" nonilion", magnitude_ending_char].concat(),
        17 => [" noniliard", magnitude_ending_char].concat(),
        18 => [" decilion", magnitude_ending_char].concat(),
        19 => [" deciliard", magnitude_ending_char].concat(),
        20 => [" undecilion", magnitude_ending_char].concat(),
        21 => [" undeciliard", magnitude_ending_char].concat(),
        22 => [" dodecilion", magnitude_ending_char].concat(),
        23 => [" dodeciliard", magnitude_ending_char].concat(),
        24 => [" tridecilion", magnitude_ending_char].concat(),
        25 => [" trideciliard", magnitude_ending_char].concat(),
        26 => [" quattordicilion", magnitude_ending_char].concat(),
        27 => [" quattordiciliard", magnitude_ending_char].concat(),
        28 => [" quindicilion", magnitude_ending_char].concat(),
        29 => [" quindiciliard", magnitude_ending_char].concat(),
        30 => [" sedicilion", magnitude_ending_char].concat(),
        31 => [" sediciliard", magnitude_ending_char].concat(),
        32 => [" diciassettilion", magnitude_ending_char].concat(),
        33 => [" diciassettiliard", magnitude_ending_char].concat(),
        34 => [" diciottilion", magnitude_ending_char].concat(),
        35 => [" diciottiliard", magnitude_ending_char].concat(),
        36 => [" diciannovilion", magnitude_ending_char].concat(),
        37 => [" diciannoviliard", magnitude_ending_char].concat(),
        38 => [" ventilion", magnitude_ending_char].concat(),
        39 => [" ventiliard", magnitude_ending_char].concat(),
        _ => String::new(),
    };

    if triplet == 0 {
        return "".to_string();
    } else if triplet == 1 {
        if triplet_index == 0 {
            return "uno".to_string();
        }

        if triplet_index == 1 {
            return "mille".to_string();
        }

        let mut word = "un".to_owned();
        word.push_str(&magnitude);
        return word;
    }

    let hundreds_digit = triplet / 100;
    let tens_digit = triplet / 10 % 10;
    let units_digit = triplet % 10;

    // Hundreds
    let hundred_word = elaborate_hundreds_digit(hundreds_digit, tens_digit);
    words.push(&hundred_word);

    // Check if the remaining number can be found in the UNDER_TWENTY array
    let two_digit_remainder = triplet % 100;
    if two_digit_remainder < 20 {
        words.push(constants::UNDER_TWENTY[two_digit_remainder]);
        words.push(&magnitude);
        return words.join("");
    };

    // Tens
    let tens_word = elaborate_tens_digit(tens_digit, units_digit);
    words.push(&tens_word);

    // Units
    let units_word = elaborate_units_word(units_digit);
    words.push(&units_word);

    // Add magnitude at the end
    words.push(&magnitude);

    return words.join("");
}

fn number_to_words(number: u128, delta: usize) -> String {
    if number == 0 {
        println!("{number} = zero");
        return "".to_string();
    }

    let triplets = split_triplets(number);

    let mut triplet_words: Vec<String> = vec![];

    for (i, triplet) in triplets.iter().enumerate() {
        let triplet_word = triplet_to_word(*triplet, i + delta);
        triplet_words.push(triplet_word);
    }

    let triplet_words_reversed: Vec<String> = triplet_words.into_iter().rev().collect();
    let number_words = triplet_words_reversed.concat();

    number_words
}

pub fn number_parts_to_words(number_parts: &Vec<u128>) -> String {
    let mut words = String::new();

    for (i, part) in number_parts.iter().enumerate() {
        let part_words = number_to_words(*part, 10 * i);
        // println!("aaa {}", part_words);
        words.insert_str(0, &part_words);
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_triplets() {
        let result = split_triplets(1234567890);
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

    // #[test]
    // fn test_loop_numbers() {
    //     // This function prints output, so you might want to capture the output and test it.
    //     // For simplicity, we are just calling it here.
    //     loop_numbers(1, 2);
    // }
}
