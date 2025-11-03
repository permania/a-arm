/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{error::Error, fs, io::{BufReader, BufRead}, os::unix::net::{UnixListener, UnixStream}};

const SOCKET_PATH: &'static str = "/tmp/barmd.sock";

fn main() -> Result<(), Box<dyn Error>> {
    if fs::exists(SOCKET_PATH)? {
	fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;

    for stream in listener.incoming() {
	handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(stream: UnixStream) -> Result<(), Box<dyn Error>> {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line?);
    }
    Ok(())
}
