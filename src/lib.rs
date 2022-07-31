// This file is part of bentley-ottmann.
//
// bentley-ottmann is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option)
// any later version.
//
// bentley-ottmann is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General
// Public License along with bentley-ottmann. If not, see
// <https://www.gnu.org/licenses/>.

#![no_std]
#![forbid(unsafe_code, rust_2018_idioms)]

use geometry::{PathEvent, Point2D, Scalar};

/// Tell whether or not a path is rectilinear.
pub fn is_rectilinear<Num: Scalar>(
    path: impl IntoIterator<Item = PathEvent<Point2D<Num>, Point2D<Num>>>,
) -> bool {
    path.into_iter().all(|event| match event {
        PathEvent::Begin { .. } | PathEvent::End { close: false, .. } => true,
        PathEvent::Cubic { .. } | PathEvent::Quadratic { .. } => false,
        PathEvent::Line { from, to }
        | PathEvent::End {
            last: to,
            first: from,
            close: true,
        } => {
            // determine the difference between the two points
            let diff = to - from;
            // either x or y has to be zero
            approx_eq(diff.x, Num::ZERO) || approx_eq(diff.y, Num::ZERO)
        }
    })
}

fn approx_eq<Num: Scalar>(a: Num, b: Num) -> bool {
    (a - b).abs() < Num::EPSILON
}
