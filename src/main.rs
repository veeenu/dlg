use std::{fmt::Display, io::BufRead, process::Command};

use clap::arg;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn select<S: AsRef<str> + Display>(choices: &[S]) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&choices[..])
        .interact_on_opt(&Term::stderr());

    match selection {
        Ok(Some(idx)) => println!("{}", choices[idx]),
        Ok(None) => {}
        Err(e) => eprintln!("{e}"),
    }
}

fn main() {
    let cmd = clap::Command::new("dlg")
        .bin_name("dlg")
        .arg(arg!(<cmd> ... "command"))
        .get_matches();

    let (subcmd, subcmd_args) = {
        let mut it = cmd.values_of("cmd").unwrap();
        (it.next().unwrap(), it.collect::<Vec<&str>>())
    };

    let input: Vec<String> = Command::new(subcmd)
        .args(subcmd_args)
        .output()
        .unwrap()
        .stdout
        .lines()
        .map(Result::unwrap)
        .collect();

    select(&input);
}
