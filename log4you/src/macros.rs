/// Logs an info-level message with a dynamically generated `log_id`.
///
/// This macro automatically generates a `log_id` using the `log_id!` macro and uses the `Logger::target`
/// to determine the log target, which can be customized for the service. It then formats the message using
/// `log::info!` with the generated `log_id` and the provided message arguments.
///
/// # Example:
/// ```rust
/// use log4you::log_info;
/// log_info!("User logged in successfully");
/// ```
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let log_id = $crate::log_id!();
        let target = $crate::logger::Logger::target();
        log::info!(target: &*target, "log_id={}, {}", log_id, format_args!($($arg)*));
    }}
}

/// Logs a warning-level message with a dynamically generated `log_id`.
///
/// This macro generates a `log_id` using the `log_id!` macro and utilizes the `Logger::target`
/// to log the warning message with the `log_id` and the provided message arguments.
///
/// # Example:
/// ```rust
/// use log4you::log_warn;
/// log_warn!("Failed to load configuration");
/// ```
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        let log_id = $crate::log_id!();
        let target = $crate::logger::Logger::target();
        log::warn!(target: &*target, "log_id={}, {}", log_id, format_args!($($arg)*));
    }}
}

/// Logs an error-level message with a dynamically generated `log_id`.
///
/// This macro generates a `log_id` using the `log_id!` macro and utilizes the `Logger::target`
/// to log the error message with the `log_id` and the provided message arguments.
///
/// # Example:
/// ```rust
/// use log4you::log_error;
/// log_error!("An unexpected error occurred");
/// ```
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        let log_id = $crate::log_id!();
        let target = $crate::logger::Logger::target();
        log::error!(target: &*target, "log_id={}, {}", log_id, format_args!($($arg)*));
    }}
}

/// Logs a debug-level message with a dynamically generated `log_id`.
///
/// This macro generates a `log_id` using the `log_id!` macro and utilizes the `Logger::target`
/// to log the debug message with the `log_id` and the provided message arguments.
///
/// # Example:
/// ```rust
/// use log4you::log_debug;
/// log_debug!("Debugging user session data");
/// ```
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        let log_id = $crate::log_id!();
        let target = $crate::logger::Logger::target();
        log::debug!(target: &*target, "log_id={}, {}", log_id, format_args!($($arg)*));
    }}
}

/// Logs an info-level message with a custom `log_id` provided as a parameter.
///
/// This macro allows specifying a custom `log_id` for logging messages at the info level, along
/// with the message arguments. The `Logger::target` is used to log the message to the appropriate service.
///
/// # Example:
/// ```rust
/// use log4you::log_info_with_id;
/// let email = "jerrymaheswara@gmail.com";
/// log_info_with_id!("CUSTOM_LOG_ID", "User logged in with email: {}", email);
/// ```
#[macro_export]
macro_rules! log_info_with_id {
    ($log_id:expr, $($arg:tt)*) => {
        let target = $crate::logger::Logger::target();
        log::info!(target: &*target, "log_id={}, {}", $log_id, format_args!($($arg)*));
    };
}

/// Logs a warning-level message with a custom `log_id` provided as a parameter.
///
/// This macro allows specifying a custom `log_id` for logging messages at the warning level, along
/// with the message arguments. The `Logger::target` is used to log the message to the appropriate service.
///
/// # Example:
/// ```rust
/// use log4you::log_warn_with_id;
/// log_warn_with_id!("CUSTOM_LOG_ID", "Configuration file missing, using defaults");
/// ```
#[macro_export]
macro_rules! log_warn_with_id {
    ($log_id:expr, $($arg:tt)*) => {
        let target = $crate::logger::Logger::target();
        log::warn!(target: &*target, "log_id={}, {}", $log_id, format_args!($($arg)*));
    };
}

/// Logs an error-level message with a custom `log_id` provided as a parameter.
///
/// This macro allows specifying a custom `log_id` for logging messages at the error level, along
/// with the message arguments. The `Logger::target` is used to log the message to the appropriate service.
///
/// # Example:
/// ```rust
/// use log4you::log_error_with_id;
/// log_error_with_id!("CUSTOM_LOG_ID", "Failed to connect to database");
/// ```
#[macro_export]
macro_rules! log_error_with_id {
    ($log_id:expr, $($arg:tt)*) => {
        let target = $crate::logger::Logger::target();
        log::error!(target: &*target, "log_id={}, {}", $log_id, format_args!($($arg)*));
    };
}

/// Logs a debug-level message with a custom `log_id` provided as a parameter.
///
/// This macro allows specifying a custom `log_id` for logging messages at the debug level, along
/// with the message arguments. The `Logger::target` is used to log the message to the appropriate service.
///
/// # Example:
/// ```rust
/// use log4you::log_debug_with_id;
/// log_debug_with_id!("CUSTOM_LOG_ID", "Debugging API response data");
/// ```
#[macro_export]
macro_rules! log_debug_with_id {
    ($log_id:expr, $($arg:tt)*) => {
        let target = $crate::logger::Logger::target();
        log::debug!(target: &*target, "log_id={}, {}", $log_id, format_args!($($arg)*));
    };
}

/// A macro to generate a log ID based on the current time using the `LogIdFormat` trait.
///
/// This macro generates a unique log ID by creating a UUID using the current timestamp (using the `now_v7` method),
/// and then formats it into a string without dashes (a 32-character string). The generated log ID is useful for tracing
/// log entries within a service and correlating events.
///
/// # Example:
/// ```rust
/// use log4you::log_id;
/// let log_id = log_id!();
/// println!("Generated Log ID: {}", log_id);
/// ```
///
/// This log ID is typically used for tracking the flow of requests or events through a system, ensuring consistency
/// and easy correlation across log entries.
///
/// # Dependencies:
/// - This macro depends on the `uuid` crate and expects that the `LogIdFormat` trait is implemented for `Uuid`.
/// - The generated log ID is a string representation of a UUID with dashes removed, typically 32 characters long.
#[macro_export]
macro_rules! log_id {
    () => {{
        use uuid::Uuid;
        use $crate::utils::log_id::LogIdFormat;
        Uuid::now_v7().to_log_id()
    }};
}
