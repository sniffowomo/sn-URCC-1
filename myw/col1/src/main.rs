/*
Making this for testing color basically 
*/

// Calling the colored crate
use colored::*;


fn main() {
    println!("{}","Fuck all nights".red().bold());
    print!("
{}
{}
{}
    ", "Hello".green().bold(), "World".green().bold().blink(), "!!!".green().bold());
}

