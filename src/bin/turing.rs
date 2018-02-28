/// Following the Destroy All Software screencast [here](https://www.destroyallsoftware.com/screencasts/catalog/computing-by-changing)

#[macro_use]
extern crate quicli;

use std::collections::HashMap;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The .turning file to read
    file: String,

    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right
}

type State = u8;
type Value = u8;

type Instructions = HashMap<(Value, State),(Value,Direction,State)>;

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;

    info!("Parsing file");
    let instructions = parse(&content)?;

    info!("Simulating file");
    simulate(&instructions);
});

const NUM_ITERATIONS: u8 = 24;

fn simulate(instructions: &Instructions) {
    let mut tape = [0; 16];
    let mut head = 0;
    let mut state = 0;
    for _ in 0..NUM_ITERATIONS {
        for value in tape.iter() {
            print!("{:02x}", value);
        }
        println!("");
        for _ in 0..head {
            print!("  ");
        }
        println!("^^");

        let (new_value, head_dir, new_state) = instructions[&(tape[head], state)];
        tape[head] = new_value;
        head = match head_dir {
            Direction::Left  => head - 1,
            Direction::Right => head + 1,
        };
        state = new_state;
    }
}

fn parse(text: &str) -> Result<Instructions> {
    let mut instructions = Instructions::new();
    for line in text.lines() {
        let line = match line.find('#') {
            Some(n) => &line[0..n],
            None => &line,
        };
        if line.trim() == "" || &line.trim()[0..1]=="#" {
            continue;
        }
        let sides = line.split("->").collect::<Vec<_>>();
        ensure!(sides.len() == 2, "No `->` in line");
        let left = sides[0].split(",").collect::<Vec<_>>();
        let right = sides[1].split(",").collect::<Vec<_>>();
        ensure!(left.len() == 2, "Wrong number of items on left side");
        ensure!(right.len() == 3, "Wrong number of items on right side");
        let value_match: u8 = left[0].trim().parse()?;
        let state_match: u8 = left[1].trim().parse()?;
        let new_value: u8 = right[0].trim().parse()?;
        let direction: char = right[1].trim().to_lowercase().chars().next().unwrap();
        let new_state: u8 = right[2].trim().parse()?;

        ensure!(direction == 'l' || direction == 'r', "Wrong number of items on left side");
        let direction = match direction {
            'l' => Direction::Left,
            'r' => Direction::Right,
            _ => panic!(),
        };

        instructions.insert((value_match, state_match), (new_value, direction, new_state));
    }
    Ok(instructions)
}
