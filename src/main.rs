use clap::{Command, arg};
use dialoguer::{theme::ColorfulTheme, Select};

fn select(input: &str) {
    let choices = input
        .lines()
        .collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&choices[..])
        .interact();

    match selection {
        Ok(idx) => println!("{}", choices[idx]),
        Err(e) => eprintln!("{e}"),
    }
}

fn main() {
    let cmd = Command::new("dlg")
        .bin_name("dlg")
        .arg(arg!([INPUT]))
        .get_matches();

    select(&cmd.get_one::<String>("INPUT").unwrap());
}
