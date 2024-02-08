use std::{fs::read_to_string, path::Path};

use crate::dea::DEA;

pub fn read_dea<T: AsRef<Path>>(path: T) -> Result<DEA<String, String>, String> {
    let contents = read_to_string(path).map_err(|e| e.to_string())?;
    parse_dea(contents)
}

fn parse_dea(contents: String) -> Result<DEA<String, String>, String>{
    let mut lines = contents.lines();
    let first_line = lines.next().ok_or("Not enough lines")?;
    let second_line = lines.next().ok_or("Not enough lines")?;
    let transitions: Vec<(String, String, String)> = lines.map(|line| {
        let colon_split: Vec<&str> = line.split(":").collect();
        let letter = colon_split.get(1).ok_or("No colon".to_string())?.trim().to_string();
        let splits: Vec<&str> = colon_split[0].split_ascii_whitespace().collect();
        Ok((splits[0].trim().to_string(), splits[1].trim().to_string(), letter))
    }).collect::<Result<Vec<(String, String, String)>, String>>()?;

    let start_state = first_line.to_string();
    let final_states: Vec<String> = second_line.split_ascii_whitespace().map(|x| x.to_string()).collect();

    Ok(DEA::from_transitions(start_state, final_states, transitions))
}