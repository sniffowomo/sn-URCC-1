/*
Testing out module system
*/

//Imports 
use m2::lib;
mod src_d1;

// Code 
fn main() {
    println!("\n Main() - Main Function in src/main.rs");
    lib::li1();
    lib::li2();
    lib::li3();
    src_d1::f1::f1func();
    src_d1::f2::f2func();
}
