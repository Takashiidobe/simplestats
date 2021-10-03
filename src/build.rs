use man::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let path = "simplestats.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("simplestats")
        .about("Prints simple statistics on a line delimited file with numbers.")
        .arg(Arg::new("path"))
        .example(
            Example::new()
                .text("Small Example")
                .command("cat numbers.txt")
                .output(
                    "
                    1 
                    2 
                    3
                    4
                    5
                    6
                    7
                    8
                    9
                    10
                ",
                ),
        )
        .example(
            Example::new()
                .text("Running the program")
                .command("simplestats numbers.txt")
                .output(
                    r#"
    +-----+-----+-------+-------+------+--------------------+
    | max | min | total | count | mean | stddev             |
    +-----+-----+-------+-------+------+--------------------+
    | 10  | 1   | 55    | 10    | 5.5  | 2.8722813232690143 |
    +-----+-----+-------+-------+------+--------------------+"#,
                ),
        )
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}
