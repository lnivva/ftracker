use clap::Parser;
use std::error::Error;

use ftracker_cli::cli::commands::Commands;
use ftracker_cli::cli::Cli;
use ftracker_cli::config;
use ftracker_cli::context::AppContext;
use ftracker_cli::errors::AppResult;

fn main() {
    let cli = Cli::parse();

    init_tracing(cli.global.verbose);

    if let Err(err) = run(cli) {
        eprintln!("error: {err}");
        let mut source = err.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {cause}");
            source = cause.source();
        }
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> AppResult<()> {
    let config = config::load(cli.global.config.as_deref())?;
    let mut ctx = AppContext::new(config)?;

    match cli.command {
        Commands::Add(args) => ftracker_cli::commands::add::run(&args, &mut ctx),
    }
}

fn init_tracing(verbosity: u8) {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let level = match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(verbosity >= 2)
        .with_writer(std::io::stderr)
        .init();
}
