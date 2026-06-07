#[cfg(feature = "bin")]
#[macro_use]
extern crate log;

#[cfg(feature = "bin")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use onetun::{config::Config, events::Bus};

    let config = Config::from_args().context("Configuration has errors")?;
    init_logger(&config)?;

    for warning in &config.warnings {
        warn!("{}", warning);
    }

    let bus = Bus::default();
    onetun::start_tunnels(config, bus).await?;

    futures::future::pending().await
}

#[cfg(not(feature = "bin"))]
fn main() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("Binary compiled without 'bin' feature"))
}

#[cfg(feature = "bin")]
fn init_logger(config: &onetun::config::Config) -> anyhow::Result<()> {
    use anyhow::Context;

    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.parse_filters(&config.log);
    // Default target for the builder is Stdout; non-error messages will go there.
    builder.target(pretty_env_logger::env_logger::Target::Stdout);

    // Use a custom formatter that writes ERROR records directly to stderr
    // while other levels are formatted into the builder's buffer (which goes to stdout).
    // Timestamp is UTC ISO8601 (e.g. 2026-06-07T12:34:56.789Z).
    builder.format(|buf, record| {
        use std::io::Write;

        // UTC timestamp with millisecond precision
        let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        // map INFO labels to "DEBUG" per your request, keep other levels as-is
        let level_label = if record.level() == log::Level::Info {
            "DEBUG".to_string()
        } else {
            record.level().to_string()
        };

        if record.level() == log::Level::Error {
            // write directly to stderr for ERROR level
            let mut stderr = std::io::stderr();
            writeln!(stderr, "{} {:<5} - {}", ts, level_label, record.args()).map(|_| ())
        } else {
            // non-error: write into the provided buffer (goes to stdout)
            writeln!(buf, "{} {:<5} - {}", ts, level_label, record.args())
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "format error"))
        }
    });

    builder.try_init().context("Failed to initialize logger")
}
