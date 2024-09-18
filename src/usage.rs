pub fn print_usage() {
    println!("Usage: ./numeri-a-parole min [max]\n");
    println!("Min only:");
    println!("\tcargo r 20");
    println!("\t=> venti\n");
    println!("Min and Max:");
    println!("\tcargo r 20 30");
    println!("\t=> venti");
    println!("\t=> ventuno");
    println!("\t=> ...");
    println!("\t=> ventinove");
    println!("\t=> trenta");
}

pub fn print_too_many_args_warning() {
    println!("\nWarn! Too many arguments: anything more than 1 is discarded.\n");
}
