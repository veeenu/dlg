use std::{fmt::Display, io::BufRead, process::Command};

use clap::{ArgMatches, arg};
use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

fn choice<S: AsRef<str> + Display>(choices: &[S]) {
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

fn choices<S: AsRef<str> + Display>(choices: &[S]) {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&choices[..])
        .interact_on_opt(&Term::stderr());

    match selection {
        Ok(Some(indices)) => {
            for idx in indices {
                println!("{}", choices[idx])
            }
        }
        Ok(None) => {}
        Err(e) => eprintln!("{e}"),
    }
}

fn choice_subcmd(matches: &ArgMatches) {
    let (subcmd, subcmd_args) = {
        let mut it = matches.values_of("cmd").unwrap();
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

    choice(&input);
}

fn choices_subcmd(matches: &ArgMatches) {
    let (subcmd, subcmd_args) = {
        let mut it = matches.values_of("cmd").unwrap();
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

    choices(&input);
}

fn main() {
    let cmd = clap::Command::new("dlg")
        .bin_name("dlg")
        .subcommand(
            clap::Command::new("choice")
                .about("Run a command, choose one of the lines of its output, and print it")
                .arg(arg!(<cmd> ... "command")),
        )
        .subcommand(
            clap::Command::new("choices")
                .about("Run a command, choose one or more of the lines of its output, and print it")
                .arg(arg!(<cmd> ... "command")),
        )
        .get_matches();

    match cmd.subcommand().unwrap() {
        ("choice", matches) => choice_subcmd(matches),
        ("choices", matches) => choices_subcmd(matches),
        _ => unreachable!()
    }
}
