use clap::Parser;

use roman_numerals::output::Output;



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

    let mut ouptut = Output::init();

    let o = match parse_value_to_integer(&args.value) {
        Ok(int) => ouptut.convert_integer(&int),
        Err(_) => ouptut.convert_roman_numeral(&args.value),
    };

    println!("Input {} is converted to {}.", &args.value, o.output)

}

fn parse_value_to_integer(value: &str) -> Result<i64, std::num::ParseIntError> {

    value.parse::<i64>()

}





