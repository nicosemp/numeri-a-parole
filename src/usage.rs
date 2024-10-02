pub fn print_usage() {
    println!("Usage: cargo r [number]\n");
    println!("\t$ cargo r 20");
    println!("\t=> venti\n");
}

pub fn print_too_many_args_warning() {
    println!("\nWarn! Too many arguments: anything more than 1 is discarded.\n");
}
