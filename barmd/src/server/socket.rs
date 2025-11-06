/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs,
    io::Write,
    os::unix::net::{UnixListener, UnixStream},
};

use byteorder::{LittleEndian, ReadBytesExt};
use log::{error, info, warn};

use crate::error::{ArmError, ERROR_INVALID_REQUEST};
use crate::math::kinematics;

const SOCKET_PATH: &str = "/tmp/barmd.sock";

pub struct SocketConnection {
    stream: UnixStream,
}

impl SocketConnection {
    pub fn new(stream: UnixStream) -> Self {
	info!("Connection initialized");
        Self { stream }
    }

    pub fn handle_client(&mut self) -> Result<(), ArmError> {
        let stream = &mut self.stream;

        Ok(loop {
            let buf: [f64; 3] = match try_read_f64_array::<3>(stream) {
                Ok(b) => b,
                Err(e) => {
                    if let ArmError::Io(_) = e {
                        break;
                    } else {
                        error!("{e:?}");
                        continue;
                    }
                }
            };

	    let buf_round = buf.map(|n| (n * 100.0).round() / 100.0);
	    info!("Incoming Request: {:?}", buf_round);

            let angles = match kinematics::calculate_angles(CoordinateRequest {
                x: buf_round[0],
                y: buf_round[1],
                z: buf_round[2],
            }) {
                Some(a) => a,
                None => {
                    warn!("{}", ArmError::InvalidCoordinates(buf[0], buf[1], buf[2]));
                    CoordinateResponse::from(ERROR_INVALID_REQUEST)
                }
            };

            let data = angles_to_byte_stream(angles);

            respond_to_client(stream, data)
        })
    }
}

impl Drop for SocketConnection {
    fn drop(&mut self) {
        info!("Connection dropped");
    }
}

/// Represents a 3D coordinate request for the mechanical arm in cm.
///
/// This struct uses `f64` for all components and is intended for **math and IK calculations**.
/// Always specify target positions in cm.
///
/// This struct is only constructed from a request to the `barmd` socket by the client.
///
/// # Fields
/// - `x`: The X-coordinate (horizontal) in centimeters (cm).
/// - `y`: The Y-coordinate (horizontal) in centimeters (cm).
/// - `z`: The Z-coordinate (vertical) in centimeters (cm).
pub struct CoordinateRequest {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents a compact 3D coordinates response from the mechanical arm.
///
/// This struct stores the coordinates in **quantized integer format** due to servos' ~1 degree accuracy.
/// It is derived from a `CoordinateRequest` by performing **inverse kinematics calculations**.
///
/// It is 4 bytes, 2 unsigned 8-bit integers and 1 unsigned 16-bit integer.
///
/// # Serialization
/// When sent over the `barmd` socket, the fields are converted into a byte stream.
/// In that stream, `r` **will be sent as 2 unsigned 8-bit integers** for convenient transmission.
/// **This value must be reconstructed at the call site.**
///
/// # Example
/// ```
/// let response = CoordinateResponse { shoulder: 128 /* degrees */, elbow: 64, rotation: 512 };
/// ```
#[derive(Debug)]
pub struct CoordinateResponse {
    /// The quantized angle of the shoulder joint in degrees.
    shoulder: u8,
    /// The quantized angle of the elbow joint in degrees.
    elbow: u8,
    /// The quantized angle of the rotating base in degrees.
    rotation: u16,
}

impl From<(u8, u8, u16)> for CoordinateResponse {
    fn from(value: (u8, u8, u16)) -> Self {
        CoordinateResponse {
            shoulder: value.0,
            elbow: value.1,
            rotation: value.2,
        }
    }
}

pub fn begin() -> Result<UnixListener, ArmError> {
    if fs::exists(SOCKET_PATH)? {
        fs::remove_file(SOCKET_PATH)?;
    }

    Ok(UnixListener::bind(SOCKET_PATH)?)
}

fn respond_to_client(stream: &mut UnixStream, data: [u8; 4]) {
    info!("Response: {:02x?}", data);
    _ = stream.write_all(&data);
}

/// Converts a `CoordinateResponse` to a stream of `u8` bytes fit for writing to a `UnixStream`
fn angles_to_byte_stream(angles: CoordinateResponse) -> [u8; 4] {
    let mut buf = [0u8; 4];
    buf[0..2].copy_from_slice(&[angles.shoulder, angles.elbow]);
    buf[2..4].copy_from_slice(&angles.rotation.to_le_bytes());

    buf
}

fn try_read_f64_array<const N: usize>(stream: &mut UnixStream) -> Result<[f64; N], ArmError> {
    let mut arr = [0f64; N];
    for i in 0..N {
        arr[i] = stream.read_f64::<LittleEndian>()?;
    }
    Ok(arr)
}
