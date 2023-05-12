use chrono::prelude::*;
use std::{io::{self, Read, Write, Error}, fs};
use serde::{Serialize, Deserialize};
use toml_edit::Document;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    ev_id: usize,
    ev_date: NaiveDate,
    ev_time: NaiveTime,
    ev_description: String,
    ev_color: String,
}

pub fn new_event() -> Result<String, Error> {
    println!("Adding new event: ");

    let mut entry_date = String::new();
    let mut entry_time = String::new();
    let mut entry_description = String::new();
    let mut entry_color = String::new();

    println!("Type the date (dd/mm/aa): ");
    io::stdin().read_line(&mut entry_date)?;
    let date = NaiveDate::parse_from_str(&mut entry_date.trim(), "%d/%m/%y").expect("Parsing error!");

    println!("Type the time (hh:mm): ");
    io::stdin().read_line(&mut entry_time)?; 
    let time = NaiveTime::parse_from_str(&mut entry_time.trim(), "%H:%M").expect("Parsing error!");
    
    println!("Type the description: ");
    io::stdin().read_line(&mut entry_description)?;
    let description: String = entry_description.trim().to_string();
    
    println!("Type the color for the entry (#RRGGBB): ");
    io::stdin().read_line(&mut entry_color)?;
    let color = entry_color.trim().to_string();
    let id = number_of_events()? + 1;

    let event = Event {
        ev_id: id,
        ev_date: date,
        ev_time: time,
        ev_description: description,
        ev_color: color,
    };

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("events.toml")?;

    let insertion = toml::to_string(&event).unwrap();
    write!(file, "[event.{}]\n", id)?;
    write!(file, "{}", insertion)?;

    return Ok("Success adding new event!".to_string())
}

pub fn number_of_events() -> Result<usize, Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("events.toml")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut num_events: usize = contents.matches("\n").count();
    num_events = (num_events-1)/6;

    return Ok(num_events);
}

fn hex_to_rgb(hex: &str) -> String {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    format!("{};{};{}", r, g, b)
}

pub fn see_all_events() -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("events.toml")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let num_lines: usize = contents.matches("\n").count();
    let num_events: usize = (num_lines-1)/6;

    let mut i_str: String;
    let mut dp_id: String;
    let mut dp_date: String;
    let mut dp_time: String;
    let mut dp_description: String;
    let mut dp_color: String;
    let mut dp_string: String = String::from("");
 
    let document: Document = contents.parse().unwrap();
    for i in 1..num_events+1 {
        i_str = i.to_string().trim().to_string();
        dp_id =             document["event"][&i_str]["ev_id"]          .to_string();
        dp_date =           document["event"][&i_str]["ev_date"]        .to_string();
        dp_time =           document["event"][&i_str]["ev_time"]        .to_string();
        dp_description =    document["event"][&i_str]["ev_description"] .to_string();
        dp_color =          document["event"][&i_str]["ev_color"]       .to_string();

        dp_string = dp_string + &dp_id + ": " + &dp_description + "\n" + &dp_date + ", at " + &dp_time + "\n"; 

        print!("{}\x1B[38;2;{}m{}\x1B[0m", "\n", hex_to_rgb(&dp_color[2..9]), &dp_string);
        dp_string.clear();
    }

    return Ok(());
}

fn main() -> Result<(), Error> {
    let mut flag: bool = true;
    while flag {
        println!("Commands available: ");
        println!("+ Add new event.");
        println!("c See all events.");
        println!("q Quit.");

        let mut mode = String::new();
        io::stdin().read_line(&mut mode)?;

        match mode.trim() {
            "+" => println!("{}", new_event()?),
            "q" => flag = false,
            "c" => see_all_events()?,
             _  => println!("Invalid mode: {}", mode),
        }
    }

    return Ok(());
}
