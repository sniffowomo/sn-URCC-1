/*
Main entry point into all the excercises that will be done
 */

// Libraries to be used
use colored::*;
mod wok;

// Main entry point into all the excercises that will be done
fn main() {
    println!("
{}
{}
{}
{}
    ", "Hello, world!".
    blue(), "---------------".green(),
    "The following are going to be the solutions to the excercise which are stored in folder wk".yellow(),
    "---------------".green());

    // Solution  files will be here
    ex1();
}

fn ex1() {
    println!("{}", "Excercise 1".red());
    wok::e1::e1();
}
