// lib
use colored::*;
use std::io::{self};

pub fn help() -> Result<(), io::Error> {
    // say hello
    say_welcome().unwrap();
    Ok(())
}

// - welcome func
pub fn say_welcome() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-> welcome to nylang, is the interplitor written in {}",
        "rust".color(blue_color_res.unwrap_or(Color::Green)).bold(),
    );
    println!(
        "-! '{}' to excute program",
        "nylang run <filename>.nyl".color(blue_color_res.unwrap_or(Color::Green)).bold(),
    );
    println!(
        "-! '{}' to parse program",
        "nylang parse <filename>.nyl".color(blue_color_res.unwrap_or(Color::Green)).bold(),
    );
    println!(
        "-! '{}' to ast program",
        "nylang ast <filename>.nyl".color(blue_color_res.unwrap_or(Color::Green)).bold(),
    );
    Ok(())
}