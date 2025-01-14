//! This module provides arguments related to the user account config.

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use log::info;

use crate::ui::table;

const ARG_ACCOUNT: &str = "account";
const CMD_ACCOUNTS: &str = "accounts";

/// Represents the account commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    /// Represents the list accounts command.
    List(table::args::MaxTableWidth),
}

/// Represents the account command matcher.
pub fn matches(m: &ArgMatches) -> Result<Option<Cmd>> {
    let cmd = if let Some(m) = m.subcommand_matches(CMD_ACCOUNTS) {
        info!("accounts command matched");
        let max_table_width = table::args::parse_max_width(m);
        Some(Cmd::List(max_table_width))
    } else {
        None
    };

    Ok(cmd)
}

/// Represents the account subcommands.
pub fn subcmds<'a>() -> Vec<App<'a, 'a>> {
    vec![SubCommand::with_name(CMD_ACCOUNTS)
        .aliases(&["account", "acc", "a"])
        .about("Lists accounts")
        .arg(table::args::max_width())]
}

/// Represents the user account name argument. This argument allows
/// the user to select a different account than the default one.
pub fn arg<'a>() -> Arg<'a, 'a> {
    Arg::with_name(ARG_ACCOUNT)
        .long("account")
        .short("a")
        .help("Selects a specific account")
        .value_name("STRING")
}

/// Represents the user account name argument parser.
pub fn parse_arg<'a>(matches: &'a ArgMatches<'a>) -> Option<&'a str> {
    matches.value_of(ARG_ACCOUNT)
}
