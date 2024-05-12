use std::process;
use std::io::stdin;
use inline_colorization::*;
use clearscreen;

#[derive(Debug)]
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
        stdin().read_line(&mut console_buffer).expect("Couldn't read input from console");
        args_buffer = console_buffer.split_whitespace().collect();  // Split on spaces and collect the iterator into a vector
        cli_action(args_buffer, &mut the_list); // "dispatch" the command
        print_list(&the_list);
    }

}