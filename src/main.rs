use owo_colors::OwoColorize;
use std::fs::{File, self};
use std::io::prelude::*;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Result, Value, json};






#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    value: i32,
}


// build item
fn set_item(name: String, value: i32) -> Item {
    let new_item: Item = Item {
        name,
        value
    };
    new_item
}


fn main() -> Result<()> {

    println!("{}", ("enter the name and the dollar amount").green().dimmed());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cant read that shit");
    let trim = input.trim();
    let input_vec: Vec<&str> = trim.split_terminator(&['+', '-'][..]).collect();
    let name = input_vec[0];
    let value = input_vec[1].parse().unwrap();

    let item1 = set_item(name.to_owned(), value);

    println!("{} {:?}", ("first item:"), item1);

    // we have successfully taken input and created a new item struct with it
    // now we can parse this struct into json and print the json to "budget.json"
    let item1_json = serde_json::to_string(&item1)?;

    
    println!("{}", item1_json.cyan().blink());


    Ok(())

}
