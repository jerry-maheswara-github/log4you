use log4you::{log_debug, log_error, log_id, log_info, log_info_with_id, log_warn, logger::Logger};
fn main() {
    let logid = log_id!();
    Logger::init(&logid,  Some("config/log4you.yaml"), Some("log4you"));

    log_info!("User logged in");
    log_warn!("Slow response detected");
    log_error!("Failed to connect to DB");
    log_debug!("Debug info here");

    let custom_id = log4you::log_id!();
    log_info_with_id!(custom_id, "This log uses custom log_id");
}
