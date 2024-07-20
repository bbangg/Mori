mod bot;
mod manager;
mod types;
mod utils;

use manager::Manager;
use spdlog::prelude::*;
use types::e_login_method::ELoginMethod;

fn main() {
    let mut manager = match Manager::new() {
        Ok(manager) => manager,
        Err(err) => {
            error!("Error: {}", err);
            return;
        }
    };
    manager.add_bot(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        ELoginMethod::GOOGLE,
    );
    loop {}
}
