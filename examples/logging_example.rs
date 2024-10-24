use extrans::log_builder::LogBuilder;
use log::{debug, info, warn, error};

fn main() {
    if let Err(e) = LogBuilder::new()
        .with_console_level("info")
        .with_log_file("output.log")
        .build() {
        println!("failed to initialise logger: {:?}", e);
    }

    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}