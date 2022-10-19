use owo_colors::OwoColorize;
use std::fs::{File, self};
use std::io::prelude::*;
use std::io::Read;

// BUDGET APP

// TO LEARN: everything

// ✅ write to a file 
// ✅ write to file without deleting contents
// ✅ read from file
// parse input as expenses & earnings
// use a calendar crate so my inputs will be timestamped

// 10-19-22
// user experience
// type budget in terminal to open

// BUDGET APP

// hello 'username here', what would you like to do?

// select option:
// 'view expenses'
// 'view earnings'
// 'input new item'
// 'delete item'



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
    
    // input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cannot read input are you sure you typed that in correctly?");
    let item = input.trim();

    // write input to "budget.txt"
    fs::write("budget.txt", item)?;

    // read "budget.txt" to budget_string
    let mut budget_file = fs::OpenOptions::new()
        // .read(true)
        .append(true)
        .open("budget.txt")?;

    println!("{}", ("input received, enter another item").cyan());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cannot read input are you sure you typed that in correctly?");
    let item = input.trim();
    let to_bytes = input.as_bytes();
    
    // write input to "budget.txt"
    budget_file.write_all(to_bytes)?;

    Ok(())
}
