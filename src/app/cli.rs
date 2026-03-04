use clap::ArgMatches;

pub(super) fn generate_matches() -> ArgMatches {
    clap::command!().get_matches()
}
