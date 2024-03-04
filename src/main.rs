use clap::Parser;
use tracing::debug;
use tracing_subscriber::EnvFilter;

// region:	  --- CLI
#[derive(Parser, Debug)]
#[command(name = "dingbat")]
#[command(version = "0.1")]
#[command(about = "CLI audio notifier for instant awareness")]
#[command(next_line_help = true)]
struct Cli {
    /// PID for the monitored process
    #[arg(short, long)]
    pid: u32,

    /// String to monitor for
    #[arg(short, long)]
    trigger: String,

    /// Sound to play when triggered
    #[arg(short, long)]
    sound: String,
}
// endregion: --- CLI

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    debug!("{:<12} - PID - {}", "ARGUMENTS", args.pid);
    debug!("{:<12} - Sound - {}", "ARGUMENTS", args.sound);
    debug!("{:<12} - Trigger - {}", "ARGUMENTS", args.trigger);
}
