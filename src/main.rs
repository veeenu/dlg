use std::{io::BufRead, process::Command};

use anyhow::{anyhow, Result};
use clap::{arg, ArgMatches};
use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

fn select(choices: Vec<String>) -> Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&choices[..])
        .interact_on_opt(&Term::stderr());

    match selection {
        Ok(Some(idx)) => {
            println!("{}", choices[idx]);
            Ok(())
        }
        Ok(None) => Err(anyhow!("")),
        Err(e) => Err(anyhow!(e)),
    }
}

fn checkboxes(choices: Vec<String>) -> Result<()> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&choices[..])
        .interact_on_opt(&Term::stderr());

    match selection {
        Ok(Some(indices)) => {
            for idx in indices {
                println!("{}", choices[idx])
            }
            Ok(())
        }
        Ok(None) => Err(anyhow!("")),
        Err(e) => Err(anyhow!(e)),
    }
}

fn run_command(matches: &ArgMatches) -> Vec<String> {
    let (subcmd, subcmd_args) = {
        let mut it = matches.values_of("cmd").unwrap();
        (it.next().unwrap(), it.collect::<Vec<&str>>())
    };

    Command::new(subcmd)
        .args(subcmd_args)
        .output()
        .unwrap()
        .stdout
        .lines()
        .map(Result::unwrap)
        .collect()
}

fn main() {
    let cmd = clap::Command::new("dlg")
        .bin_name("dlg")
        .subcommand(
            clap::Command::new("select")
                .about("Run a command, select one of the lines of its output, and print it")
                .arg(arg!(<cmd> ... "command")),
        )
        .subcommand(
            clap::Command::new("checkboxes")
                .about(
                    "Run a command, choose zero or more of the lines of its output, and print them",
                )
                .arg(arg!(<cmd> ... "command")),
        )
        .get_matches();

    let outcome = match cmd.subcommand().unwrap() {
        ("select", matches) => select(run_command(matches)),
        ("checkboxes", matches) => checkboxes(run_command(matches)),
        _ => unreachable!(),
    };

    if let Err(e) = outcome {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
