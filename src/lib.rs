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

fn split_triplets(num: u128) -> Vec<usize> {
    let num_str = num.to_string();
    let reversed: String = num_str.chars().rev().collect();
    let chunks: Vec<usize> = reversed
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            let chunk_str: String = chunk.iter().rev().map(|&b| b as char).collect();
            chunk_str.parse::<usize>().unwrap()
        })
        .collect();
    chunks
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

pub fn loop_numbers(min: u128, max: u128) {
    let mut max_len: usize = 0;
    let mut longest_number = String::new();

    for number in min..=max {
        if number == 0 {
            println!("{number} = zero");
            continue;
        }

        let triplets = split_triplets(number);

        let mut triplet_words: Vec<String> = vec![];

        for (i, triplet) in triplets.iter().enumerate() {
            let triplet_word = triplet_to_word(*triplet, i);
            triplet_words.push(triplet_word);
        }

        let triplet_words_reversed: Vec<String> = triplet_words.into_iter().rev().collect();
        let number_word = triplet_words_reversed.concat();

        println!("{number} = {number_word}");

        // Save longest number
        if min != max && number_word.len() > max_len {
            max_len = number_word.len();
            longest_number = number_word;
        }
    }

    if min != max {
        println!("\nLongest number (first encountered): {longest_number}");
    }
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

    #[test]
    fn test_loop_numbers() {
        // This function prints output, so you might want to capture the output and test it.
        // For simplicity, we are just calling it here.
        loop_numbers(1, 2);
    }
}
