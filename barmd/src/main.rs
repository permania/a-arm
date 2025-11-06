/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
mod error;
mod math;
mod server;

use error::ArmError;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use server::socket::{self, SocketConnection};

fn init_logger() -> Result<(), ArmError> {
    println!("Initializing Logger");

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

fn main() -> Result<(), ArmError> {
    let listener = socket::begin()?;
    init_logger()?;
    info!("Logger Initialized");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let mut conn = SocketConnection::new(s);
                conn.handle_client()?;
            }
            Err(e) => {
                todo!("handle stream errors: {e}");
            }
        }
    }

    Ok(())
}
