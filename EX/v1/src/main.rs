/*
Main entry point into all the excercises that will be done
 */

// Libraries to be used
use colored::*;
mod tezt;
mod wok; // Declaring this makes it visible

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
    wok::partA::e1();
}
