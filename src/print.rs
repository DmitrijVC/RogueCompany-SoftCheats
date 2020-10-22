/*

    Copied from old project.

*/


#![allow(unused_variables, dead_code)]

use ansi_term::Colour;


pub fn log(msg: String) {
    println!(" {} {}",
             Colour::RGB(67, 207, 12).paint("[LOG]"),
             Colour::RGB(176, 176, 176).paint(msg)
    );
}

pub fn error(msg: String) {
    println!(" {} {}",
             Colour::RGB(184, 29, 18).paint("[ERROR]"),
             Colour::RGB(176, 176, 176).paint(msg)
    );
}

pub fn warn(msg: String) {
    println!(" {} {}",
             Colour::RGB(219, 124, 0).paint("[WARN]"),
             Colour::RGB(176, 176, 176).paint(msg)
    );
}

pub fn info(msg: String) {
    println!(" {} {}",
             Colour::RGB(67, 207, 12).paint("[INFO]"),
             Colour::RGB(176, 176, 176).paint(msg)
    );
}
