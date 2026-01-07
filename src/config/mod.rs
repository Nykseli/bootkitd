use std::str::FromStr;

use clap::Parser;

/// Log levels that are idententical to `tracing::Level` but includes
/// `FullTrace` to separate traces that have library traces
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
    /// Same as `Trace` but includes verbose logs from libraries like sqlx and zbus.
    FullTrace,
}

impl FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let int_parse = s.parse::<usize>().ok().and_then(|num| match num {
            1 => Some(Self::Error),
            2 => Some(Self::Warn),
            3 => Some(Self::Info),
            4 => Some(Self::Debug),
            5 => Some(Self::Trace),
            6 => Some(Self::FullTrace),
            _ => None,
        });

        let str_parse = match s {
            s if s.eq_ignore_ascii_case("error") => Some(Self::Error),
            s if s.eq_ignore_ascii_case("warn") => Some(Self::Warn),
            s if s.eq_ignore_ascii_case("info") => Some(Self::Info),
            s if s.eq_ignore_ascii_case("debug") => Some(Self::Debug),
            s if s.eq_ignore_ascii_case("trace") => Some(Self::Trace),
            s if s.eq_ignore_ascii_case("full_trace") => Some(Self::FullTrace),
            s if s.eq_ignore_ascii_case("full-trace") => Some(Self::FullTrace),
            _ => None,
        };

        match (int_parse, str_parse) {
            (Some(val), None) => Ok(val),
            (None, Some(val)) => Ok(val),
            _ => Err(format!("Argument '{s}' is not any of 'error', 'warn', 'info', 'debug', 'trace' 'full_trace' (case insensitive). Nor in rage of 1-6."))
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
            LogLevel::FullTrace => Self::TRACE,
        }
    }
}

impl From<LogLevel> for tracing::level_filters::LevelFilter {
    fn from(value: LogLevel) -> Self {
        // First turn value to tracing::Level and then into LevelFilter
        let value: tracing::Level = value.into();
        value.into()
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConfigArgs {
    /// Use session/user message bus connection instead of system
    #[arg(short, long, default_value_t = false)]
    pub session: bool,

    /// Set log level, overriding BOOTKIT_LOG_LEVEL env variable.
    ///
    /// Possible values: "error", "warn", "info", "debug", "trace", "full_trace" or a number 1-6
    #[arg(short, long)]
    pub log_level: Option<LogLevel>,

    /// Print pretty logging output that includes colors and timestamps
    #[arg(short, long, default_value_t = false)]
    pub pretty: bool,
}

#[cfg(not(feature = "dev"))]
pub const GRUB_FILE_PATH: &str = "/etc/default/grub";
#[cfg(feature = "dev")]
pub const GRUB_FILE_PATH: &str = "tmp/grub";

#[cfg(not(feature = "dev"))]
pub const GRUB_ROOT_PATH: &str = "/etc/default";
#[cfg(feature = "dev")]
pub const GRUB_ROOT_PATH: &str = "tmp";

#[cfg(not(feature = "dev"))]
pub const GRUB_ENV_PATH: &str = "/boot/grub2/grubenv";
#[cfg(feature = "dev")]
pub const GRUB_ENV_PATH: &str = "tmp/grubenv";

#[cfg(not(feature = "dev"))]
pub const GRUB_CFG_PATH: &str = "/boot/grub2/grub.cfg";
#[cfg(feature = "dev")]
pub const GRUB_CFG_PATH: &str = "tmp/grub.cfg";

#[cfg(not(feature = "dev"))]
pub const DATABASE_PATH: &str = "/var/lib/bootkit/bootkit.db";
#[cfg(feature = "dev")]
pub const DATABASE_PATH: &str = "tmp/bootkit.db";

#[cfg(not(feature = "dev"))]
pub const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;
#[cfg(feature = "dev")]
pub const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;

#[cfg(not(feature = "dev"))]
pub const BOOTKIT_LOG_FILE: &str = "/var/log/bootkitd.log";
#[cfg(feature = "dev")]
pub const BOOTKIT_LOG_FILE: &str = "tmp/bootkitd.log";
