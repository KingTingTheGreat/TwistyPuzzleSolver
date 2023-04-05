// use std::ptr::null;
use std::collections::{HashMap, HashSet};

/*
returns boolean indicating whether the state is valid
this means it is a string of length 54 and contains 9 of each color
*/
pub fn is_valid_state(state: String) -> bool {
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

/*
takes a string representing the state of the cube and returns a 2D vector of characters
*/
fn string_to_array(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if i % 9 == 0 && i != 0 {
            output.push(row);
            row = Vec::new();
        }
        row.push(c);
    }
    output.push(row);
    output
}

struct TwistyCube {
    state: String,
    depth: i32, 
    cube: Vec<Vec<char>>,
}

impl TwistyCube {
    fn new() -> TwistyCube {
        TwistyCube {
            state: String::from("UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB"),
            depth: 0,
            cube: string_to_array("UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB"),
        }
    }
    
    fn get_state(&self) -> &str {
        &self.state
    }
    
    fn get_depth(&self) -> i32 {
        self.depth
    }
    
    fn set_state(&mut self, new_state: String) {
        if !is_valid_state(new_state.clone()) {
            println!("Invalid state");
            return;
        }
        self.state = new_state;
        self.cube = string_to_array(&self.state);
    }

    fn set_depth(&mut self, new_depth: i32) {
        self.depth = new_depth;
    }
    fn do_move(&mut self, input:char) {
        match input {
            'U' => {
                move_u(1);
                // rotate top face clockwise
                // rotate top face clockwise
            }
            'L' => {
                move_l(1);
                // rotate left face clockwise
            }
            'F' => {
                move_f(1);
                // rotate front face clockwise
            }
            'R' => {
                move_r(1);
                // rotate right face clockwise
            }
            'B' => {
                move_b(1);
                // rotate back face clockwise
            }
            'D' => {
                move_d(1);
                // rotate bottom face clockwise
            }
            'u' => {
                move_u(3);
                // rotate top face counter-clockwise
            }
            'l' => {
                move_l(3);
                // rotate left face counter-clockwise
            }
            'f' => {
                move_f(3);
                // rotate front face counter-clockwise
            }
            'r' => {
                move_r(3);
                // rotate right face counter-clockwise
            }
            'b' => {
                move_b(3);
                // rotate back face counter-clockwise
            }
            'd' => {
                move_d(3);
                // rotate bottom face counter-clockwise
            }
            '0' => {
                move_u(2);
                // rotate top face twice
            }
            '1' => {
                move_l(2);
                // rotate left face twice
            }
            '2' => {
                move_f(2);
                // rotate front face twice
            }
            '3' => {
                move_r(2);
                // rotate right face twice
            }
            '4' => {
                move_b(2);
                // rotate back face twice
            }
            '5' => {
                move_d(2);
                // rotate bottom face twice
            }
            _ => {
                println!("Invalid move: {}", input);
            }        
        }
    }
    fn move_u(&mut self, num_turns: i32) {
        // rotate top face clockwise
    }
    fn move_l(&mut self, num_turns: i32) {
        // rotate left face clockwise
    }
    fn move_f(&mut self, num_turns: i32) {
        // rotate front face clockwise
    }
    fn move_r(&mut self, num_turns: i32) {
        // rotate right face clockwise
    }
    fn move_b(&mut self, num_turns: i32) {
        // rotate back face clockwise
    }
    fn move_d(&mut self, num_turns: i32) {
        // rotate bottom face clockwise
    }


}




// moving the cube
fn do_move(input:char, cube:TwistyCube) {
    match input {
        'U' => {
            // rotate top face clockwise
            // rotate top face clockwise
        }
        'L' => {
            // rotate left face clockwise
        }
        'F' => {
            // rotate front face clockwise
        }
        'R' => {
            // rotate right face clockwise
        }
        'B' => {
            // rotate back face clockwise
        }
        'D' => {
            // rotate bottom face clockwise
        }
        'u' => {
            // rotate top face counter-clockwise
        }
        'l' => {
            // rotate left face counter-clockwise
        }
        'f' => {
            // rotate front face counter-clockwise
        }
        'r' => {
            // rotate right face counter-clockwise
        }
        'b' => {
            // rotate back face counter-clockwise
        }
        'd' => {
            // rotate bottom face counter-clockwise
        }
        '0' => {
            // rotate top face twice
        }
        '1' => {
            // rotate left face twice
        }
        '2' => {
            // rotate front face twice
        }
        '3' => {
            // rotate right face twice
        }
        '4' => {
            // rotate back face twice
        }
        '5' => {
            // rotate bottom face twice
        }
        _ => {
            println!("Invalid move: {}", input);
        }        
    }
}

fn move_u(num_turns: i32) {
    // rotate top face clockwise
}
fn move_l(num_turns: i32) {
    // rotate left face clockwise
}
fn move_f(num_turns: i32) {
    // rotate front face clockwise
}
fn move_r(num_turns: i32) {
    // rotate right face clockwise
}
fn move_b(num_turns: i32) {
    // rotate back face clockwise
}
fn move_d(num_turns: i32) {
    // rotate bottom face clockwise
}


// fn main() {
//     // let mut cube = create_cube();
//     // println!("The initial state of the cube: {}", cube.get_state());
// }