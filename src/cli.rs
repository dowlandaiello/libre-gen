/// Libre-gen is a parser for the GenBank nucleotide sequence format.
#[derive(Clap)]
#[clap(version = "1.0", author = "Dowland A.")]
pub struct Opts {
    /// Prints debugging-related logs to the console.
    #[clap(short = "d", long = "debug")]
    debug: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn configure_logging(&self) {}
}

#[derive(Clap)]
enum SubCommand {
    /// Parses the provided GenBank file, printing its parsed contents to the console.
    Parse(ParseCommand),
}

/// Parses the provided GenBank file, printing its parsed contents to the console.
#[derive(Clap)]
struct ParseCommand {
    /// The path to the GenBank file that will be parsed.
    file: String,
}

/// Configures logging with the env_logger, considering a given Opts struct.
pub fn configure_logging(opts: &Opts) {
    // If we're in debug mode, configure env_logger to use debug mode
    if opts.debug {
        // Tell the logger to print debug! calls
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        // Tell the logger to print info! calls, but not debug!
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
}
