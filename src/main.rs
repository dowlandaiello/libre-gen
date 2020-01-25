use libre_gen::cli::Opts;

fn main() {
    // Get any options passed by the user from the command-line
    let opts: Opts = Opts::parse();

    opts.configure_logging();

    println!("Hello, world!");
}
