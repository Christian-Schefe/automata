use std::{collections::HashSet, env::args, io::stdin};

use dea_parser::read_dea;

use crate::dea::DEA;

mod dea;
mod dea_parser;
mod nea;

fn main() {
    match main_result() {
        Ok(()) => (),
        Err(err_str) => println!("Error: {}", err_str),
    }
}

fn main_result() -> Result<(), String> {
    let combiner = |states: HashSet<String>| {
        let mut v: Vec<String> = states.into_iter().collect();
        v.sort();
        v.join("")
    };

    let arg = read_arg()?;
    let mut dea = read_dea(arg)?;
    println!("In: {}\n", dea);
    println!("Complete: {}\n", dea.completed("TRAP"));
    println!("Min: {}\n", dea.completed("TRAP").minimized(combiner));
    read_word(|x| {
        println!("{}", dea.accepts(x.split_ascii_whitespace()));
    })?;
    Ok(())
}

fn read_word<U, T: Fn(String) -> U>(callback: T) -> Result<(), String> {
    let stdin = stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).map_err(|x| x.to_string())?;
        buf = buf.trim().to_string();
        if buf == "exit" {
            break;
        }
        callback(buf);
    }
    Ok(())
}

fn read_arg() -> Result<String, String> {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        Err("Too many arguments.".to_string())
    } else {
        args.into_iter()
            .nth(1)
            .ok_or("Not enough arguments.".to_string())
    }
}
