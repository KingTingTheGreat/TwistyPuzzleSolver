mod twisty_cube;
use std::io::stdin;

fn main() {
    let mut user_input = String::new();
    println!("Enter the current state of the cube (WOGRBY):");
    stdin().read_line(&mut user_input).unwrap();
    // remove whitespace characters
    let initial_state = user_input.trim().to_string();
    // check if state is valid
    if !twisty_cube::is_valid_state(3, initial_state.clone()) {
        println!("Invalid state");
        return;
    }
    println!("The initial state of the cube: {}", initial_state);
}
