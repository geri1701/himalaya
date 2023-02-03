//! This module provides arguments related to the cache.

use clap::{Arg, ArgAction, ArgMatches};

const ARG_DISABLE_CACHE: &str = "disable-cache";

/// Represents the disable cache flag argument. This argument allows
/// the user to disable any sort of cache.
pub fn arg() -> Arg {
    Arg::new(ARG_DISABLE_CACHE)
        .long("disable-cache")
        .help("Disable any sort of cache")
        .action(ArgAction::SetTrue)
}

/// Represents the disable cache flag parser.
pub fn parse_disable_cache_flag(m: &ArgMatches) -> bool {
    m.get_flag(ARG_DISABLE_CACHE)
}
