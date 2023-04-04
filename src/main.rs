use std::io::stdin;
use std::collections::{HashMap, HashSet};

/*
returns boolean indicating whether the state is valid
this means it is a string of length 54 and contains 9 of each color
 */
fn is_valid_state(state: String) -> bool {
    // for a 3x3x3 cube
    let total_num_colors = 54;
    let quantity_per_color = 9;
    if state.len() != total_num_colors {
        println!("Invalid state length: {}", state.len());
        return false;
    }
    // set of valid colors
    let valid_colors:HashSet<char>= HashSet::from_iter(['W', 'O', 'G', 'R', 'B', 'Y'].iter().cloned());
    // check that there are 9 of each color
    let mut colors:HashMap<char, i32> = HashMap::new();
    // add quantity of each color to hashmap
    for color in state.chars() {
        // println!("char: {}", color);  // printing each input character
        if colors.contains_key(&color) {
            colors.insert(color, colors[&color] + 1);    
        }
        else {
            colors.insert(color, 1);
        }
    }
    // check that each color is valid and has quantity of 9
    for color in colors.keys() {
        if !valid_colors.contains(&color) {
            println!("Invalid color: {}", color);
            return false;
        }
        if colors[color] != quantity_per_color {
            println!("Invalid quantity of color {}: {}", color, colors[color]);
            return false;
        }
    }
    return true;
}

fn main() {
    let mut user_input = String::new();
    println!("Enter the current state of the cube (WOGRBY):");
    stdin().read_line(&mut user_input).unwrap();
    // remove whitespace characters
    let initial_state = user_input.trim().to_string();
    // check if state is valid
    if !is_valid_state(initial_state.clone()) {
        println!("Invalid state");
        return;
    }
    println!("The initial state of the cube: {}", initial_state);
}
