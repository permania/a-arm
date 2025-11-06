/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
mod error;
mod math;
mod server;

use error::ArmError;
use server::socket::{self, SocketConnection};

fn main() -> Result<(), ArmError> {
    let listener = socket::begin()?;

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
