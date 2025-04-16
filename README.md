# log4you
Rust structured logging with dynamic UUID log IDs, built on log4rs. Plug and play!

**log4you** is a lightweight logging crate built on top of [log4rs], designed for applications that need consistent, structured logging with unique log identifiers (UUIDs).

- 🔧 Powered by `log4rs`
- 🆔 Generates a unique `log_id` (UUID v7) per log entry
- 🪄 Easy-to-use macros: `log_info!`, `log_error!`, etc.
- 🛠️ Supports dynamic config paths, no boilerplate needed

Perfect for microservices, APIs, and any system where traceability and clean logs matter.
