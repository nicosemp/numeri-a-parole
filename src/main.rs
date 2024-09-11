use std::env;

mod constants;
mod usage;

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

fn elaborate_tens_digit(tens_digit: usize, two_digit_remainder: usize) -> String {
    let remainder = two_digit_remainder % 10;
    let word = constants::TENS[tens_digit];
    if remainder == 1 || remainder == 8 {
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

    // Suffix for 'mila', 'milioni', 'miliardi', ...
    let word_suffix_end = if triplet == 1 {
        if triplet_index % 2 == 0 {
            "e"
        } else {
            "o"
        }
    } else {
        "i"
    };
    let word_suffix = match triplet_index {
        1 => "mila".to_string(),
        2 => [" milion", word_suffix_end, " "].concat(),
        3 => [" miliard", word_suffix_end, " "].concat(),
        4 => [" bilion", word_suffix_end, " "].concat(),
        5 => [" biliard", word_suffix_end, " "].concat(),
        6 => [" trilion", word_suffix_end, " "].concat(),
        7 => [" triliard", word_suffix_end, " "].concat(),
        8 => [" quadrilion", word_suffix_end, " "].concat(),
        9 => [" quadriliard", word_suffix_end, " "].concat(),
        10 => [" quintilion", word_suffix_end, " "].concat(),
        11 => [" quintiliard", word_suffix_end, " "].concat(),
        12 => [" sestilion", word_suffix_end, " "].concat(),
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

        word.push_str(&word_suffix);
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
        words.push(&word_suffix);
        return words.join("");
    };

    // Tens
    let tens_word = elaborate_tens_digit(tens_digit, two_digit_remainder);
    words.push(&tens_word);

    // Units
    let units_word = elaborate_units_word(units_digit);
    words.push(&units_word);

    // Add suffix at the end
    words.push(&word_suffix);

    return words.join("");
}

fn main() {
    println!("{}", u128::MAX);

    // Read command line arguments
    let args: Vec<String> = env::args().collect();

    // TODO: Test too many/few arguments
    // TODO: Test invalid input (only digits 0-9 should be allowed)
    // TODO: Test numbers too large
    // TODO: Test min greater than max
    // TODO: Test negative numbers
    if args.len() == 1 || (args.len() == 2 && args[1] == "help") {
        usage::print_usage();
        return;
    }

    let min = (&args[1]).to_string();
    let max = match args.len() {
        x if x > 2 => (&args[2]).to_string(),
        _ => min.clone(),
    };

    if min == max {
        println!("number: {}", min);
    } else {
        println!("min: {}", min);
        println!("max: {}", max);
    }

    let min = min.trim().parse::<u128>().unwrap_or(0);
    let max = max.trim().parse::<u128>().unwrap_or(0);

    let mut max_len: usize = 0;
    let mut longest_number = String::new();

    for number in min..=max {
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
        if number_word.len() > max_len {
            max_len = number_word.len();
            longest_number = number_word;
        }
    }

    println!("Longest number: {}", longest_number);
}
