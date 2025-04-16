# log4you

> Structured logging for Rust with dynamic UUID log IDs, built on [log4rs](https://docs.rs/log4rs).


**log4you** is a lightweight logging crate, designed for applications that need consistent, structured logging with unique log identifiers (UUIDs). It allows simple, efficient, and consistent logging with unique log IDs for each request.

## ✨ Features

- 🔧 Powered by `log4rs`, configure logging dynamically with YAML configuration files, compatible with the standard Rust `log` facade
- ✅ Structured logging with automatic **UUID log IDs**
- 🆔 Generates a unique `log_id` (UUID v7) per log entry
- 🪄 Easy-to-use macros: `log_info!`, `log_error!`, etc.
- 🛠️ Supports dynamic config paths, log rotation, and file size management
- 🚀 Easy setup and integration — works out of the box
- 🧵 Great for async or multi-threaded apps

Perfect for microservices, APIs, and any system where traceability and clean logs matter.

## 📦 Installation


Add `log4you` to your `Cargo.toml`:

```toml
[dependencies]
log4you = "0.1.1"
```

Or, use [cargo-edit](https://crates.io/crates/cargo-edit) to add it directly from your terminal:

```bash
cargo add log4you
```


## ⚙️ Example YAML Configuration

See the [`log4rs` configuration documentation](https://docs.rs/log4rs/latest/log4rs/#configuration) for more details.

```yaml
appenders:
  stdout:
    kind: console
    encoder:
      pattern: "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l})} {f}:{L}] - {m}{n}"

  log4you:
    kind: rolling_file
    path: "logs/log4you.log"
    policy:
      kind: compound
      trigger:
        kind: size
        limit: 100MB
      roller:
        kind: fixed_window
        pattern: "logs/log4you-{}.log"
        count: 5
    encoder:
      pattern: "[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l})} {f}:{L}] - {m}{n}"

root:
  level: info
  appenders:
    - stdout

loggers:
  log4you:
    level: debug
    appenders:
      - log4you
```

## 🛠️ Usage Example

```rust
use log4you::{logger::Logger, log_id, log_info, log_info_with_id};

fn main() {
    let logid = log_id!();
    // Initialize the logger with a log_id, a path to the YAML config, and the service name
    Logger::init(&logid,  Some("config/log4you.yaml"), Some("log4you"));

    // Log an info message, logid will be generated automatically
    log_info!("Service started");

    // Log an info message, logid is defined by yourself
    let custom_log_id = log_id!();
    log_info_with_id!(custom_log_id, "This log uses custom log_id");
}
```


## Author
Jerry Maheswara <jerrymaheswara@gmail.com>


## 📖 License

This project is licensed under the Apache-2.0 license.

---

> Built with ❤️ in Rust