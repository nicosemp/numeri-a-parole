pub fn print_usage() {
    println!("{}", "Usage: cargo r min [max]");
    println!("{}", "Examples:");
    println!("{}", "\tcargo r 20");
    println!("{}", "\t=> venti");
    println!("{}", "");
    println!("{}", "\tcargo r 20 30");
    println!("{}", "\t=> venti");
    println!("{}", "\t=> ventuno");
    println!("{}", "\t=> ...");
    println!("{}", "\t=> ventinove");
    println!("{}", "\t=> trenta");
}
