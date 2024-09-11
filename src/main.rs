use std::env;

const TENTHS: [&str; 10] = [
    "",
    "",
    "venti",
    "trenta",
    "quaranta",
    "cinquanta",
    "sessanta",
    "settanta",
    "ottanta",
    "novanta",
];

const UNDER_TWENTY: [&str; 20] = [
    "zero",
    "uno",
    "due",
    "tre",
    "quattro",
    "cinque",
    "sei",
    "sette",
    "otto",
    "nove",
    "dieci",
    "undici",
    "dodici",
    "tredici",
    "quattordici",
    "quindici",
    "sedici",
    "diciassette",
    "diciotto",
    "diciannove",
];

fn input_to_number(input: String) -> usize {
    // Convert the input to a number
    let number: usize = input.trim().parse::<usize>().unwrap_or(0);

    return number;
}

fn print_numbers_words(mut num: usize) -> Vec<&'static str> {
    let mut words: Vec<&'static str> = vec![];

    while num > 0 {
        if num < UNDER_TWENTY.len() {
            words.push(UNDER_TWENTY[num]);
            break;
        } else if num < 100 {
            let tenth = num / 10;
            let remainder = num % 10;
            let word = TENTHS[tenth];

            // Elisione della vocale finale se l'unità è 1 o 8
            words.push(if remainder == 1 || remainder == 8 {
                &word[..word.len() - 1]
            } else {
                word
            });

            num = remainder;
        } else if num < 1000 {
            let hundreds = num / 100;
            let remainder = num % 100;

            if hundreds > 1 {
                words.push(UNDER_TWENTY[hundreds]);
            }

            // Elisione della 'o' con decine che iniziano per 8: "cent" + "ottanta" = centottanta
            words.push("cent");
            if !remainder.to_string().starts_with("8") {
                words.push("o")
            }

            num = remainder;
        }
    }
    words
}

fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    let min = (&args[1]).to_string();
    let max = (&args[2]).to_string();
    let min = input_to_number(min);
    let max = input_to_number(max);
    println!("You entered: {}", min);
    println!("You entered: {}", max);

    // Prompt the user to enter a number
    // println!("Please enter a number: ");
    // let input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap_or(0);

    // let number = input_to_number(input);

    let mut max_len: usize = 0;
    let mut longest_number = String::new();

    for number in min..=max {
        let answer = print_numbers_words(number);
        let number_word = answer.concat();
        println!("{}", answer.concat());
        if number_word.len() > max_len {
            max_len = number_word.len();
            longest_number = number_word;
        }
    }

    println!("{}", longest_number);
}
