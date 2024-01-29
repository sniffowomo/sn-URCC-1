/*
main.rs which will call the panty.rs
- to make this function accessible in main you need to have the pub keyword in the beginning
*/

mod panty;
mod fetish;

fn main() {
    println!("Hello, world!");
    
    // Calling functions from panty.rs 
    panty::stink::smellpanty();
    panty::stink::ass_pussy();

    // Calling functions from fetish.rs
    fetish::scat::scat();
}
