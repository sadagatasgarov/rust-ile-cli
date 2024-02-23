use std::{env, fs};

use text_colorizer::Colorize;
#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target string> <replacment string> <INPUT FILE> <OUTPUT FILE>");
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1);
        },
    };

    match fs::write(&args.output_file, &data) {
        Ok(f2) => {},
        Err(e) => {
            eprintln!("{} failed to write to file {}: {:?}", "Error".red().bold(), &args.output_file, e);
            std::process::exit(1);
        },
    }
    


}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments give, Expect 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(), 
    }
}

pub fn run() {
    let args = parse_args();
    //println!("{:?}", args);
    read_and_write(&args);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_write() {
        // Create a temporary file for testing
        let input_file = "test_input.txt";
        let output_file = "test_output.txt";
        std::fs::write(input_file, "Hello, World!").unwrap();
        
        let args = Arguments {
            pattern: "Hello".to_string(),
            replace: "Hi".to_string(),
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        };

        read_and_write(&args);

        let output_data = std::fs::read_to_string(output_file).unwrap();
        assert_eq!(output_data, "Hello, World!");

        // Clean up temporary files
        std::fs::remove_file(input_file).unwrap();
        std::fs::remove_file(output_file).unwrap();
    }
}

