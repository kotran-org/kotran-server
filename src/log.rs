use std::{panic, thread};
use tracing::{error, level_filters::LevelFilter};
use tracing_appender::non_blocking::WorkerGuard;

use crate::OperatingEnvironment;

pub struct Logger {}

impl Logger {
    pub fn init(op_env: OperatingEnvironment) -> WorkerGuard {
        let file_logger = tracing_appender::rolling::daily("logs", "daily.log");
        let console_logger = std::io::stdout();

        // Note : Update the log level filter for own use.
        let max_level = match op_env {
            OperatingEnvironment::Development => LevelFilter::TRACE,
            OperatingEnvironment::Production => LevelFilter::DEBUG,
        };

        let (non_blocking, guard) = match op_env {
            OperatingEnvironment::Development => tracing_appender::non_blocking(console_logger),
            OperatingEnvironment::Production => tracing_appender::non_blocking(file_logger),
        };

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_max_level(max_level)
            .init();

        // Catch panic and log them using tracing instead of default output to StdErr.
        panic::set_hook(Box::new(|info| {
            let thread = thread::current();
            let thread = thread.name().unwrap_or("unknown");

            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };

            let backtrace = backtrace::Backtrace::new();

            match info.location() {
                Some(location) => {
                    // Without backtrace
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}",
                            thread,
                            msg.replace("notrace - ", ""),
                            location.file(),
                            location.line()
                        );
                    }
                    // With backtrace
                    else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                            thread,
                            msg,
                            location.file(),
                            location.line(),
                            backtrace
                        );
                    }
                }
                None => {
                    // Without backtrace
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'",
                            thread,
                            msg.replace("notrace - ", ""),
                        );
                    }
                    // With backtrace
                    else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                            thread,
                            msg,
                            backtrace
                        );
                    }
                }
            }
        }));
        guard
    }
}