use clap::Parser;

use rusty_romans::output::{Output, parse_value_to_integer};


/// Provide either an integer value or a roman numeral to convert to the other
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Value to be parsed
    #[arg(short, long)]
    value: String,
}

fn main() {
    let args = Cli::parse();

    let mut output = Output::init();
    

    let mut validated_input = match parse_value_to_integer(&args.value) {
        Ok(int) => output.validate_integer_input(int),
        Err(_) => output.validate_roman_numeral_input(&args.value),
    };

    

    let converted_value = match validated_input.clone().output {
        
        Ok(out) => match parse_value_to_integer(&out) {
            Ok(i) => validated_input.convert_integer(&i),
            Err(_) => validated_input.convert_roman_numeral(&out),
        },
        Err(e) => {
            validated_input.output = Err(e);
            &mut validated_input
        },
    };

    if let Err(e) = converted_value.output {
        println!("Error: {e}")
    }

    println!("Input {} is converted to {:?}.", &args.value, converted_value.clone().output.unwrap())

}





