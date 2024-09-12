use numeri_a_parole::loop_numbers;
use numeri_a_parole::parse_input;

fn main() {
    let (min, max, is_error) = parse_input();
    if is_error {
        return;
    }

    loop_numbers(min, max);
}
