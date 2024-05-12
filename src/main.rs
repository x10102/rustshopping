use std::{error::Error, process};
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
use std::path::Path;
use std::fs::File;
use inline_colorization::*;
use clearscreen;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ListItem {
    name: String,
    count: u16,
    bought: bool
}

impl ListItem {
    fn to_string(&self) -> String {
        format!("{name} [{count}] - {status}{color_reset}", 
            name=self.name, 
            count=self.count, 
            status=if self.bought {format!("{color_green}BOUGHT")} else {format!("{color_yellow}NOT BOUGHT")})
    }
}

fn export_json(filename: String, list: &Vec<ListItem>) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(&filename);
    {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, list)?;
    }
    Ok(())
}

fn import_json(filename: String) -> Result<Vec<ListItem>, Box<dyn Error>> {
    let file_path = Path::new(&filename);
    {
        let file = File::open(file_path)?;
        let file_reader = BufReader::new(file);
        Ok(serde_json::from_reader(file_reader)?)
    }
}

fn cli_action(args: Vec<&str>, list: &mut Vec<ListItem>) {

    match args[..] {
        ["add", name, n] => {
            match n.parse::<u16>() {
                Err(_) => println!("Invalid input"),
                Ok(count) => list.push(ListItem { name: name.to_string(), count, bought: false })
            }
        },

        ["buy", index] => {
            if list.len() == 0 {
                println!("{color_red}Nothing to buy{color_reset}");
                return;
            }
            match index.parse::<usize>() {
                Err(_) => println!("{color_red}Invalid index{color_reset}"),
                Ok(i) if i > list.len() - 1 => println!("{color_red}Invalid index{color_reset}"),
                Ok(i) => list[i].bought = true
            }
            if list.iter().all(|item| item.bought) {
                print_list(list);
                println!("{color_green}You bought everything you needed!{color_reset}");
                process::exit(0);
            };
        },

        ["remove", index] => {
            if list.len() == 0 {
                println!("{color_red}Nothing to remove{color_reset}");
                return;
            }
            match index.parse::<usize>() {
                Err(_) => println!("Invalid index"),
                Ok(i) if i > list.len() - 1 => println!("{color_red}Invalid index{color_reset}"),
                Ok(i) => drop(list.remove(i))   // Drop the result since the other arms return ()
            }
        },

        ["list"] => return,

        ["clear"] => clearscreen::clear().unwrap(), // Just crash if we can't clear the screen i guess lol

        ["export", filename] => {
            match export_json(filename.to_owned(), &list) {
                Ok(()) => println!("{color_yellow}List has been exported to {filename}{color_reset}"),
                Err(e) => println!("{color_red}Export failed: {e}{color_reset}")
            }
        },

        ["import", filename] => {
            match import_json(filename.to_owned()) {
                Ok(imported) => *list = imported,
                Err(e) => println!("{color_red}Import failed: {e}{color_reset}")
            }
        }

        ["exit", ..] => process::exit(0),

        _ => println!("{color_red}Invalid command{color_reset}")
    }
}

fn print_list(list: &Vec<ListItem>) {
    for (index, item) in list.iter().enumerate() {
        println!("{index}: {}", item.to_string())
    }
}

fn main() -> ! { // main doesn't return since we exit from cli_action
    let mut the_list = vec![
        ListItem{name: "Milk".into(), count: 1, bought: false}, 
        ListItem{name: "Eggs".into(), count: 10, bought: false}, 
        ListItem{name: "Rohlik".into(), count: 5, bought: true}]; // Create the default list

    let mut console_buffer = String::with_capacity(64); // Make sure the string is allocated so that it doesn't slow the user down by 0.01ms 
    let mut args_buffer: Vec<&str>;
    loop {
        console_buffer.clear();
        print!("> ");
        stdout().flush().expect("I/O Error"); // Write the prompt to console
        stdin().read_line(&mut console_buffer).expect("I/O Error");
        args_buffer = console_buffer.split_whitespace().collect();  // Split on spaces and collect the iterator into a vector
        cli_action(args_buffer, &mut the_list); // "dispatch" the command
        print_list(&the_list);
    }

}