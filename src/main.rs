use text_colorizer::Colorize;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}


fn main() {
print_help();
let args: Vec<String> = env::args().skip(1).collect();

//println!("{:?}", args)

}

fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("Usage: <target string> <replacment string> <INPUT FILE> <OUTPUT FILE>");

}
