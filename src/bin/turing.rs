
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right
}

type State = u8;
type Value = u8;

fn main() {
    use Direction::*;

    let mut x_b = HashMap::new();
    x_b.insert((0, 0), (1, Right, 1));
    x_b.insert((1, 0), (2, Right, 1));
    x_b.insert((2, 0), (3, Right, 1));
    x_b.insert((3, 0), (4, Right, 1));
    x_b.insert((4, 0), (5, Right, 2));

    x_b.insert((0, 1), (0, Left,  0));

    x_b.insert((5, 2), (5, Right, 2));
    x_b.insert((0, 2), (0, Left,  2));

    simulate(&x_b);
}

fn simulate(instructions: &HashMap<(Value, State),(Value,Direction,State)>) {
    let mut tape = [0; 2];
    let mut head = 0;
    let mut state = 0;
    for _ in 0..16 {
        println!("{:02x}{:02x}", tape[0], tape[1]);
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
