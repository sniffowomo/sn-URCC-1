/*
Teting out the cfonto here
*/
use cfonts::*;

pub fn cfo1() {
    say(Options {
        text: String::from("Excercise 1"),
        font: Fonts::FontPallet,
        colors: vec![Colors::Magenta, Colors::GreenBright],
        ..Options::default()
    })
}
