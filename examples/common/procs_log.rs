#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::sync::Arc;

fn main() {
    let _guard = log_util::init_log(None);
    // let env = Arc::new(EnvBuilder::new().build());

    info!("Greeter received: {}", "..");
}
