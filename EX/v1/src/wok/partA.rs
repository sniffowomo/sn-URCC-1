/*
First excercise
*/

use crate::tezt::cfo;
use colored::*;

pub fn e1() {
    print!(
        "{}",
        "
        Excercise 1 - Basic Variable declaration and printing
        "
        .green()
        .italic()
    );
    cfo::cfo1();
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}
