use std::time::Instant;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::prelude::*;

const TIME_FORMATTER: &str = "[day]-[month]-[year] [hour]:[minute]:[second]";

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init_logs(level: &str) {
        let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"));
        let timer = LocalTime::new(time::format_description::parse(TIME_FORMATTER).unwrap());

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_timer(timer).with_target(false))
            .init();
    }

    #[inline]
    pub fn debug(&self, args: std::fmt::Arguments) {
        if tracing::level_enabled!(tracing::Level::DEBUG) {
            debug!("{}", args);
        }
    }

    #[inline]
    pub fn info(&self, args: std::fmt::Arguments) {
        if tracing::level_enabled!(tracing::Level::INFO) {
            info!("{}", args);
        }
    }

    #[inline]
    pub fn warn(&self, args: std::fmt::Arguments) {
        if tracing::level_enabled!(tracing::Level::WARN) {
            warn!("{}", args);
        }
    }

    #[inline]
    pub fn error(&self, args: std::fmt::Arguments) {
        if tracing::level_enabled!(tracing::Level::ERROR) {
            error!("{}", args);
        }
    }
}

pub struct TimerGuard {
    label: &'static str,
    start: Instant,
}

impl Drop for TimerGuard {
    fn drop(&mut self) {
        let dur = self.start.elapsed();
        debug!("{} took {:?}", self.label, dur);
    }
}

#[macro_export]
macro_rules! logger_debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! logger_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! logger_warn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! logger_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(format_args!($($arg)*))
    };
}

