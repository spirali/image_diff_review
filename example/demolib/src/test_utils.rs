// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::RgbImage;
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
pub(crate) fn check_snapshot(image: RgbImage, image_name: &str) {
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
