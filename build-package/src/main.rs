// A package archive building program, reads from a toml file. This program should
// be capable of installing programs, compiling source, installing some specified dependencies.

use progress_bar::*;
use std::path::PathBuf;
use structopt::StructOpt;
pub mod parse;
fn main()
{
    init_progress_bar(1);
    let opt = Opt::from_args();


}

// Command line options
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.

    /// Clean build files after building
    #[structopt(short,long)]
    clean: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Modlulate the verbosity of the output. (-v, -vv, -vvv)
    /// the more occurences of 'v' the more verbosity.
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// The build file. By default the program looks for 'ubuild.toml' in the current directory.
    #[structopt(short, long)]
    build_file: PathBuf,
}



// Progress bar: https://lib.rs/crates/progress_bar
// Temporary files: https://lib.rs/crates/tempfile
// Creation of archive: https://lib.rs/crates/tar
// Configuration locations https://lib.rs/crates/directories


