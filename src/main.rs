use std::io::{self, Write};
extern crate colored;
use colored::*;

fn main() {
    println!("Hello~~ Welcome to TwacqwqOS!");
    let mut input = String::new();

    while input != "\n" {
        print!("❯");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "twacqwq" => {
                println!("{}", "Hello~~ Welcome to TwacqwqOS!".bright_purple())
            }
            "who" => {
                println!("{}", "I'm Twacqwq!".bright_cyan())
            }
            "where" => {
                println!("{}", "Shenzhen, China".bright_blue())
            }
            "blog" => {
                println!("{}", "blog.xiaohao233.top".bright_green())
            }
            "status" => {
                println!(
                    "{}",
                    "A Gopher. continue learning, continue pondering.".bright_yellow()
                )
            }
            "stack" => {
                println!(
                    "{}, {}, {}, {}, {}",
                    "Golang".on_bright_purple().purple(),
                    "Java".on_bright_red().red(),
                    "Python".on_bright_green().green(),
                    "JavaScript".on_bright_blue().blue(),
                    "ShellScript".on_bright_black().black(),
                )
            }
            "email" => {
                println!("{}", "xiaot.qaq@gmail.com".bold().bright_magenta())
            }
            "exit" => {
                println!(
                    "{}",
                    "月がきれい"
                        .italic()
                        .bright_blue()
                );
                input = String::from("\n");
            }
            _ => println!("not match"),
        }
    }
}
