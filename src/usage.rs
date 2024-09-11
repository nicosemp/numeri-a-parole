pub fn print_usage() {
    println!("{}", "Usage: cargo r min [max]");
    println!("{}", "Examples:");
    println!("{}", "\tcargo r 20");
    println!("{}", "\t=> Prints 'venti'");
    println!("{}", "");
    println!("{}", "\tcargo r 20 30");
    println!(
        "{}",
        "\t=> Prints all the numbers from 'venti' (20) to 'trenta' (30) included."
    );
}
