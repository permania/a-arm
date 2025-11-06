/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{f64::consts::PI, ops::Mul};

use libm::atan2;

use crate::server::socket::{CoordinateRequest, CoordinateResponse};

/// The lenth of the upper arm in **centimeters** (cm).
const UPPER_ARM_LENGTH: f64 = 6.0;
/// The lenth of the forearm in **centimeters** (cm).
const FOREARM_LENGTH: f64 = 6.0;

/// Performs **inverse kinematics** calculations on a target point.
pub fn calculate_angles(target: CoordinateRequest) -> Option<CoordinateResponse> {
    let l1 = UPPER_ARM_LENGTH;
    let l2 = FOREARM_LENGTH;
    let dt = distance_from_origin(target.x, target.z).clamp(0., l1 + l2);

    let elbow_angle = angle_from_sides(l1, l2, dt);
    let elbow_angle_deg = rad_to_deg(elbow_angle);

    let outer_shoulder_angle = atan2(target.z, target.x);
    let outer_shoulder_angle_deg = rad_to_deg(outer_shoulder_angle);

    let inner_shoulder_angle = angle_from_sides(l1, dt, l2);
    let inner_shoulder_angle_deg = rad_to_deg(inner_shoulder_angle);

    let shoulder_angle_deg = inner_shoulder_angle_deg + outer_shoulder_angle_deg;

    let base_angle = atan2(target.y, target.x);
    let base_angle_deg = (rad_to_deg(base_angle) + 360.) % 360.;

    if [elbow_angle_deg, shoulder_angle_deg, base_angle_deg]
        .iter()
        .any(|v| v.is_nan())
    {
        return None;
    }

    let resp = CoordinateResponse::from((
        elbow_angle_deg as u8,
        shoulder_angle_deg as u8,
        base_angle_deg as u16,
    ));
    Some(resp)
}

/// Returns the distance of a point `(x, y)` from the origin `(0, 0)`.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the point.
/// * `y` - The y-coordinate of the point.
///
/// # Returns
///
/// * The Euclidean distance from the origin.
///
/// # Example
///
/// ```
/// let d = distance_from_origin(3.0, 4.0);
/// assert_eq!(d, 5.0);
/// ```
fn distance_from_origin(x: f64, y: f64) -> f64 {
    let distance: f64 = square(x) + square(y);
    distance.sqrt()
}

/// Computes the angle (in radians) opposite side `opposite` in a triangle
/// with side lengths `l1`, `l2`, and `opposite` using the Law of Cosines.
///
/// # Arguments
///
/// * `l1` - Length of the first side.
/// * `l2` - Length of the second side.
/// * `opposite` - Length of the side opposite the angle to find.
///
/// # Returns
///
/// * The angle in radians.
///
/// # Example
///
/// ```
/// let angle = angle_from_sides(3.0, 4.0, 5.0);
/// // angle is approximately 0.6435 radians (~36.87deg)
/// ```
fn angle_from_sides(l1: f64, l2: f64, opposite: f64) -> f64 {
    ((square(l1) + square(l2) - square(opposite)) / (2.0 * l1 * l2)).acos()
}

/// Converts an angle from radians to degrees.
///
/// # Arguments
///
/// * `rads` - Angle in radians.
///
/// # Returns
///
/// * Angle in degrees.
///
/// # Example
///
/// ```
/// use std::f64::consts::PI;
/// let degrees = rad_to_deg(PI);
/// assert_eq!(degrees, 180.0);
/// ```
fn rad_to_deg(rads: f64) -> f64 {
    rads * 180.0 / PI
}

/// Returns the square of a number.
///
/// Works for any type `T` that implements `Mul` with `Output = T` and is `Copy`.
///
/// # Arguments
///
/// * `x` - The value to square.
///
/// # Returns
///
/// * The squared value.
///
/// # Example
///
/// ```
/// let n = square(5);
/// assert_eq!(n, 25);
/// let f = square(3.0);
/// assert_eq!(f, 9.0);
/// ```
fn square<T>(x: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    x * x
}
