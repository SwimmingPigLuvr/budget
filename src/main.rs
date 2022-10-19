use owo_colors::OwoColorize;
use std::fs::{File, self};
use std::io::prelude::*;

// BUDGET APP

// TO LEARN: everything

// ✅ write to a file 
// write to file without deleting contents
// read from file
// parse input as expenses & earnings
// 

fn main() -> std::io::Result<()> {
    

    

    println!("\n\n{}", ("BUDGET APP").green());
    println!(
        "{}",
        ("-----------------------------------------------")
            .green()
            .dimmed()
    );
    println!(
        "{} {} {} {} {} {} {} {}",
        ("Input").green().dimmed(),
        ("+").green(),
        ("or").green().dimmed(),
        ("-").red(),
        ("the").green().dimmed(),
        ("dollar amount").cyan(),
        ("followed by").green().dimmed(),
        ("item").cyan()
    );
    println!(
        "{}\n",
        ("-----------------------------------------------")
            .green()
            .dimmed()
    );
    
    let mut budget = File::create("budget.txt")?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cannot read input are you sure you typed that in correctly?");
    let item = input.trim();
    fs::write("budget.txt", item)?;

    Ok(())
}
