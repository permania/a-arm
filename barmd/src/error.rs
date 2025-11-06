/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io;
use thiserror::Error;

pub const ERROR_INVALID_REQUEST: (u8, u8, u16) = (0x00, 0x00, 0x0001u16);

#[derive(Debug, Error)]
pub enum ArmError {
    /// Mirror of `std::io::Error`
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    /// Mirror of `fern::InitError`
    #[error("Error Initializing Logger: {0}")]
    LogInit(#[from] fern::InitError),

    /// Mirror of `log::SetLoggerError`
    #[error("Error Setting Logger: {0}")]
    SetLogger(#[from] log::SetLoggerError),

    #[error("Invalid Coordinate Request: ({0}, {1}, {2})")]
    InvalidCoordinates(f64, f64, f64),
}
