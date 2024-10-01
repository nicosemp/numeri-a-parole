use numeri_a_parole::get_number_sign;
use numeri_a_parole::number_parts_to_words;
use numeri_a_parole::parse_string_to_numbers;
use numeri_a_parole::read_args;

fn main() {
    let number_str = read_args();

    let (number_without_sign, is_negative) = get_number_sign(&number_str);

    let number_parts = parse_string_to_numbers(&number_without_sign);

    let number_words = number_parts_to_words(&number_parts);

    println!("{} {number_words}", if is_negative { "meno" } else { "" });
}
