/*
Crate name = m3
main.rs - This will have th main functions
*/

// Imports
use m3::lib;
mod func1;

// Main Code logic
fn main() {
    println!("src/main.rs - Main function");

    // This is a library subfunction
    lib::lib_func1();
    lib::lib_func2();

    // Calling panty functions
    func1::panty::panty_func1();
    func1::panty2::panty_func2();
    func1::pussy::pussy_func1();
}