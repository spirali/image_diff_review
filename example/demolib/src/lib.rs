// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::{Rgb, RgbImage};

/// Create an image with rectangle
pub fn create_rectangle(x1: u32, y1: u32, x2: u32, y2: u32, color: Rgb<u8>) -> RgbImage {
    RgbImage::from_fn(100, 100, |x, y| {
        if x1 <= x && x < x2 && y1 <= y && y < y2 {
            color
        } else {
            Rgb([255, 255, 255])
        }
    })
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::check_snapshot;

    #[test]
    fn test_create_rectangle() {
        //      If you want to make this test fails, change something
        //
        //      For example:
        //      Change the value ----\
        //      here to e.g. 25      |
        //                           v
        let image = create_rectangle(10, 5, 50, 70, Rgb([255, 0, 0]));
        check_snapshot(image, "create_rectangle.png");
    }
}
