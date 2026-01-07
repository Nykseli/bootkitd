use std::{
    fs::{File, OpenOptions},
    path::Path,
    str::FromStr,
};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::{
    config::{ConfigArgs, LogLevel, BOOTKIT_LOG_FILE, DEFAULT_LOG_LEVEL},
    dctx,
    errors::{DRes, DResult},
};

fn open_log_file<P: AsRef<Path>>(path: P) -> DResult<File> {
    let path = path.as_ref();
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .ctx(
            dctx!(),
            format!("Cannot open a log file: '{}'", path.to_string_lossy()),
        )
}

fn log_level() -> LogLevel {
    if let Some(level) = option_env!("BOOTKIT_LOG_LEVEL") {
        // Possible values (case insensitive)
        // ERROR
        // WARN
        // INFO
        // DEBUG
        // TRACE
        LogLevel::from_str(level).unwrap_or(DEFAULT_LOG_LEVEL)
    } else {
        DEFAULT_LOG_LEVEL
    }
}

pub fn setup_logging(args: &ConfigArgs) -> DResult<()> {
    let log_file = open_log_file(BOOTKIT_LOG_FILE)?;

    let level = if let Some(level) = args.log_level {
        level
    } else {
        log_level()
    };

    // Always inclode at least debug info in log files
    let file_level = if level <= LogLevel::Debug {
        LogLevel::Debug
    } else {
        level
    };

    let filter = Targets::new().with_default(level);
    let file_filter = Targets::new().with_default(file_level);
    // turn off library logging unless we want to trace everything
    let (filter, file_filter) = if level != LogLevel::FullTrace {
        (
            filter
                .with_target("sqlx", LevelFilter::OFF)
                .with_target("zbus", LevelFilter::OFF),
            file_filter
                .with_target("sqlx", LevelFilter::OFF)
                .with_target("zbus", LevelFilter::OFF),
        )
    } else {
        (filter, file_filter)
    };

    let subscriber = tracing_subscriber::registry();
    // the different fmt::layer makes subscriber type very unhappy
    // so we need to have separate init calls
    if !args.pretty {
        subscriber
            .with(
                fmt::layer()
                    .without_time()
                    .with_ansi(false)
                    .with_filter(filter),
            )
            .with(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(log_file)
                    .with_filter(file_filter),
            )
            .init()
    } else {
        subscriber
            .with(fmt::layer().with_filter(filter))
            .with(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(log_file)
                    .with_filter(file_filter),
            )
            .init()
    }

    Ok(())
}
