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
mod tests {
    use super::*;

    use std::path::{Path, PathBuf};

    /// Directory where current tests creates images
    fn current_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("current")
    }

    /// Directory with blessed snapshots
    fn snapshot_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("snapshots")
    }

    fn is_generate_all_mode() -> bool {
        std::env::var("DEMOLIB_TEST")
            .map(|x| x.to_ascii_lowercase() == "generate-all")
            .unwrap_or(false)
    }

    /// Check an image against snapshot
    fn check_snapshot(image: RgbImage, image_name: &str) {
        let snapshot_dir = snapshot_dir();
        let snapshot = image::ImageReader::open(snapshot_dir.join(image_name))
            .map_err(|e| e.to_string())
            .and_then(|x| x.decode().map_err(|e| e.to_string()))
            .map(|x| x.to_rgb8());
        if let Ok(snapshot) = snapshot {
            if snapshot != image {
                image.save(current_dir().join(image_name)).unwrap();
                panic!("Snapshot is different; run 'cargo xtask-test report' for report")
            }
        } else {
            println!("{}", current_dir().join(image_name).display());
            image.save(current_dir().join(image_name)).unwrap();
            snapshot.unwrap();
        }
        if is_generate_all_mode() {
            image.save(current_dir().join(image_name)).unwrap();
        }
    }

    #[test]
    fn test_create_rectangle() {
        let image = create_rectangle(10, 5, 50, 70, Rgb([255, 0, 0]));
        check_snapshot(image, "create_rectangle.png");
    }
}
