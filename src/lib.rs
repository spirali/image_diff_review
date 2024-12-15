// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::difference::{compute_differences, Difference, ImageInfoResult, PairResult};
use image::ImageError;
use std::path::{Path, PathBuf};

use crate::pair::pairs_from_paths;
use crate::report::create_html_report;
use thiserror::Error;

mod difference;
mod fs;
mod pair;
mod report;

#[cfg(feature = "xtask-cli")]
pub mod xtask_cli;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Path is a directory: `{0}`")]
    NotDirectory(PathBuf),

    #[error("Image error")]
    ImageError(#[from] ImageError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct CompareConfig<'a> {
    ignore_match: bool,
    ignore_left_missing: bool,
    ignore_right_missing: bool,
    filter_name: Option<&'a str>,
}

impl<'a> CompareConfig<'a> {
    pub fn set_ignore_match(&mut self, value: bool) {
        self.ignore_match = value;
    }

    pub fn set_ignore_left_missing(&mut self, value: bool) {
        self.ignore_left_missing = value;
    }

    pub fn set_ignore_right_missing(&mut self, value: bool) {
        self.ignore_right_missing = value;
    }

    pub fn set_filter_name(&mut self, value: Option<&'a str>) {
        self.filter_name = value;
    }
}

pub struct ReportConfig<'a> {
    left_title: &'a str,
    right_title: &'a str,
}

impl Default for ReportConfig<'_> {
    fn default() -> Self {
        ReportConfig {
            left_title: "Left image",
            right_title: "Right image",
        }
    }
}

impl<'a> ReportConfig<'a> {
    pub fn set_left_title(&mut self, title: &'a str) {
        self.left_title = title;
    }

    pub fn set_right_title(&mut self, title: &'a str) {
        self.right_title = title;
    }
}

#[derive(Default)]
pub struct ImageDiff {
    diffs: Vec<PairResult>,
}

impl ImageDiff {
    pub fn compare_directories(
        &mut self,
        config: &CompareConfig,
        left_path: &Path,
        right_path: &Path,
    ) -> Result<()> {
        let pairs = pairs_from_paths(left_path, right_path, config.filter_name)?;
        let mut diffs = compute_differences(pairs);

        if config.ignore_match {
            diffs.retain(|pair| !matches!(pair.difference, Difference::None));
        }

        if config.ignore_left_missing {
            diffs.retain(|pair| !matches!(pair.left_info, ImageInfoResult::Missing));
        }

        if config.ignore_right_missing {
            diffs.retain(|pair| !matches!(pair.right_info, ImageInfoResult::Missing));
        }
        self.diffs.append(&mut diffs);
        Ok(())
    }

    pub fn create_report(&self, config: &ReportConfig, output: &Path, verbose: bool) -> Result<()> {
        if verbose && self.diffs.is_empty() {
            println!("Nothing to report");
            return Ok(());
        }
        let count = self.diffs.len();
        create_html_report(config, &self.diffs, output)?;
        if verbose {
            println!(
                "Report written into '{}'; found {} images",
                output.display(),
                count,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
