// This file is part of rectiloafier.
//
// rectiloafier is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option)
// any later version.
//
// rectiloafier is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General
// Public License along with rectiloafier. If not, see
// <https://www.gnu.org/licenses/>.

#![no_std]
#![forbid(unsafe_code, rust_2018_idioms)]

use bentley_ottmann::trapezoids;
use core::{convert::TryInto, iter::FusedIterator};
use geometry::{Box2D, Edge, FillRule, PathEvent, Point2D, Scalar};
use num_traits::Bounded;

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

/// Tell whether or not a polygon is rectilinear.
pub fn is_polygon_rectilinear<Num: Scalar>(polygon: impl IntoIterator<Item = Edge<Num>>) -> bool {
    polygon.into_iter().all(|edge| {
        approx_eq(edge.line.vector.y, Num::ZERO) || approx_eq(edge.line.vector.x, Num::ZERO)
    })
}

/// Get the rectangles making up a rectilinear path.
///
/// # Panics
///
/// Panics if the path is not rectilinear.
pub fn boxes<Num: Scalar + Bounded + Default>(
    polygon: impl IntoIterator<Item = Edge<Num>>,
) -> impl FusedIterator<Item = Box2D<Num>> {
    trapezoids(polygon, FillRule::Winding)
        .map(|trap| trap.try_into().expect("path is not rectilinear"))
}

fn approx_eq<Num: Scalar>(a: Num, b: Num) -> bool {
    (a - b).abs() < Num::EPSILON
}
