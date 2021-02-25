use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let solution = solve(input);
    write_output(solution);
    return Ok(());
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    return Ok(buffer);
}

fn solve(input: String) -> String {
    input
}

fn write_output(output: String) {
    println!("{}", output);
}
