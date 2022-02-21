use std::collections::VecDeque;

use std::{ io::{ self, Write }, process};

fn main() {
    let mut folds = 0;
    let mut adds = 0;
    let mut steps = VecDeque::new();

    let mut layer_order = VecDeque::new();
    println!("- Please enter how many layers you want: ");
    
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut user_input = input.trim().parse::<i32>().unwrap_or_else(|_| {eprintln!("- Entered input is not Integer!");drop(input);process::exit(1);});

    println!("- The input entered by User is {}.",user_input);

    while user_input != 1 {
        layer_order.push_front(user_input);
        if user_input % 2==0 {
            user_input = user_input / 2;
            folds = folds + 1;
            steps.push_front("Fold".to_string());
        } else {
            user_input = user_input - 1;
            adds = adds + 1;
            steps.push_front("Add".to_string());
        }
    }
    layer_order.push_front(1);

    println!("Folds: {}, Adds: {}", folds, adds);
    println!("{:?}", layer_order);
    println!("{:?}", steps);
}