use std::path::PathBuf;
use std::process;

use log4rs;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use uuid::Uuid;
use crate::utils::log_id::LogIdFormat;

/// Global target log name (default: "log4you")
pub static LOG_TARGET: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("log4you".to_string()));

/// A struct responsible for initializing and managing the logger for the application.
///
/// The `Logger` struct handles the configuration of the logging system, allowing you to
/// define log levels, log formats, and the log output destinations such as the console
/// and rolling file logs. It also ensures that logs are enriched with a unique `log_id`
/// and optional `service_name` for easier traceability across different parts of the application.
///
/// You can initialize the logger with a specific log identifier (`log_id`), path to a configuration
/// file (`config_path`), and the service name (`service_name`). The configuration is applied
/// through a YAML configuration file, which defines the log format and the output policy.
///
/// # Example Usage
///
/// ```rust
/// use log4you::{logger::Logger, log_id, log_info, log_info_with_id};
///
/// fn main() {
///     let logid = log_id!();
///     // Initialize the logger with a log_id, a path to the YAML config, and the service name
///     Logger::init(&logid,  Some("config/log4you.yaml"), Some("log4you"));
///
///     // Log an info message, logid will be generated automatically
///     log_info!("Service started");
///
///     // Log an info message, logid is defined by yourself
///     let custom_id = log4you::log_id!();
///     log_info_with_id!(custom_id, "This log uses custom log_id");
/// }
/// ```
///
/// ## Methods
/// - `init`: Initializes the logger with the given `log_id`, `config_path`, and `service_name`.
///
/// ## Notes
/// - Make sure that the log configuration file exists at the specified path and is accessible.
/// - By default, the logger will write logs to the console (`stdout`) and to the log file if configured.
/// - The logger will automatically apply the log configuration defined in the YAML file.
///
/// ## Example YAML Configuration:
/// See the [`log4rs` configuration documentation](https://docs.rs/log4rs/latest/log4rs/#configuration)
/// for details on how to configure the loggers, appenders, and log levels.

pub struct Logger;

impl Logger {
    /// Initializes the logging system using a log4rs YAML configuration file.
    ///
    /// This function must be called at the start of the application to enable the logging system.
    /// It will use the provided YAML configuration file and add the `log_id` and `service_name`
    /// to every log entry.
    ///
    /// # Parameters
    ///
    /// - `log_id`: A global log identifier (typically a UUID or unique label for tracing logs per request).
    /// - `config_path`: The path to the `log4you.yaml` configuration file. If `None`, the default path
    ///   is `config/log4you.yaml`.
    /// - `service_name`: The name of the service or module used as the log target. If `None`, the default
    ///   target is `"log4you"`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use log4you::{logger::Logger, log_info, log_info_with_id};
    ///
    /// fn main() {
    ///     Logger::init(
    ///         "MY-LOG-ID",
    ///         Some("config/log4you.yaml"),
    ///         Some("my_service_name"),
    ///     );
    ///
    ///     log_info!("Service started");
    ///
    ///     let custom_id = log4you::log_id!();
    ///     log_info_with_id!(custom_id, "This log uses custom log_id");
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Ensure the `log4you.yaml` file exists and is valid, otherwise the program will exit (`exit(1)`).
    /// - Use macros like `log_info!`, `log_error!`, etc., from this crate to maintain consistent log format.
    ///
    /// # Panics
    ///
    /// This function will terminate the program if the configuration file is not found or is invalid.
    ///
    /// # Example log4rs YAML configuration for the `log4you` crate.
    ///
    /// This configuration defines two appenders: one for logging to the console and
    /// one for rolling file logs. The file appender uses a rolling policy based on
    /// file size, with a limit of 100MB per file and up to 5 rolled files.
    ///
    /// # Example: config/log4you.yaml
    ///
    /// ```yaml
    /// appenders:
    ///   stdout:
    ///     kind: console
    ///     encoder:
    ///       pattern: "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l})} {f}:{L}] - {m}{n}"
    ///
    ///   log4you:
    ///     kind: rolling_file
    ///     path: "logs/log4you.log"
    ///     policy:
    ///       kind: compound
    ///       trigger:
    ///         kind: size
    ///         limit: 100MB
    ///       roller:
    ///         kind: fixed_window
    ///         pattern: "logs/log4you-{}.log"
    ///         count: 5
    ///     encoder:
    ///       pattern: "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l})} {f}:{L}] - {m}{n}"
    ///
    /// root:
    ///   level: info
    ///   appenders:
    ///     - stdout
    ///
    /// loggers:
    ///   log4you:
    ///     level: debug
    ///     appenders:
    ///       - log4you
    /// ```
    ///
    /// ## Description:
    /// - **appenders**: Defines where the logs will be written. The `stdout` appender writes logs to the console,
    ///   while `log4you` writes to a rolling file with a size limit and multiple log file backups.
    /// - **root**: The root logger's log level is set to `info`, and it uses the `stdout` appender.
    /// - **loggers**: The `log4you` logger is set to `debug` level, and it uses the `log4you` rolling file appender.
    ///
    /// ## Notes:
    /// - The pattern format used in the encoder specifies the timestamp, log level, file name, and line number,
    ///   followed by the log message. You can modify this pattern to suit your needs.
    /// - Ensure that the specified log file paths (`logs/log4you.log`) exist or are writable by the application.
    /// - The rolling policy ensures log files do not grow too large by archiving older logs (with up to 5 backups).

    pub fn init(log_id: &str, config_path: Option<&str>, service_name: Option<&str>) {
        if let Some(name) = service_name {
            let mut target = LOG_TARGET.write().unwrap();
            *target = name.to_string();
        }

        let config_path = config_path.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("../config/log4you.yaml"));

        if !config_path.exists() {
            eprintln!(
                "log_id={}, Warning: Config file {} not found. Exiting.",
                log_id,
                config_path.display()
            );
            process::exit(1);
        }

        match log4rs::init_file(&config_path, Default::default()) {
            Ok(_) => {
                log::info!("log_id={}, Logger initialized from {}", log_id, config_path.display());
                let _ = Uuid::from_log_id(log_id);
            }
            Err(e) => {
                eprintln!("log_id={}, Logger init error: {}. Exiting.", log_id, e);
                process::exit(1);
            }
        }
    }

    /// Getter for target global
    pub fn target() -> String {
        LOG_TARGET.read().unwrap().clone()
    }
}
