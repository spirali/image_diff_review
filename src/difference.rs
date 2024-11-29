use crate::difference::ImageInfoResult::Loaded;
use crate::pair::Pair;
use image::{Pixel, Rgb, RgbImage};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub(crate) struct ImageInfo {
    pub size: Size,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl ImageInfo {
    pub fn from_image(image: &RgbImage) -> Self {
        ImageInfo {
            size: Size::new(image.width(), image.height()),
        }
    }
}

pub(crate) enum ImageInfoResult {
    Loaded(ImageInfo),
    Missing,
    Error(String),
}

impl ImageInfoResult {
    pub fn info(&self) -> Option<&ImageInfo> {
        match self {
            Loaded(info) => Some(info),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Difference {
    None,
    MissingFile,
    LoadError,
    SizeMismatch,
    Content {
        n_different_pixels: u64,
        distance_sum: u64,
        diff_image: RgbImage,
    },
}

pub(crate) struct PairResult {
    pub pair: Pair,
    pub difference: Difference,
    pub left_info: ImageInfoResult,
    pub right_info: ImageInfoResult,
}

fn load_image(path: &Path) -> crate::Result<RgbImage> {
    Ok(image::ImageReader::open(path)?.decode()?.into_rgb8())
}

fn load_image_with_info(path: &Path) -> (Option<RgbImage>, ImageInfoResult) {
    if !path.exists() {
        return (None, ImageInfoResult::Missing);
    }
    match load_image(path) {
        Ok(image) => {
            let info = ImageInfo::from_image(&image);
            (Some(image), ImageInfoResult::Loaded(info))
        }
        Err(e) => (None, ImageInfoResult::Error(e.to_string())),
    }
}

fn compute_pair_diff(pair: &Pair) -> (Difference, ImageInfoResult, ImageInfoResult) {
    let (left, left_info) = load_image_with_info(&pair.left);
    let (right, right_info) = load_image_with_info(&pair.right);

    let (left, right) = match (left, right) {
        (Some(left), Some(right)) => (left, right),
        _ => {
            return (
                match (&left_info, &right_info) {
                    (_, ImageInfoResult::Error(_)) | (ImageInfoResult::Error(_), _) => {
                        Difference::LoadError
                    }
                    (_, ImageInfoResult::Missing) | (ImageInfoResult::Missing, _) => {
                        Difference::MissingFile
                    }
                    _ => unreachable!(),
                },
                left_info,
                right_info,
            )
        }
    };

    if left.width() != right.width() || left.height() != right.height() {
        return (Difference::SizeMismatch, left_info, right_info);
    }

    let n_different_pixels: u64 = left
        .pixels()
        .zip(right.pixels())
        .map(|(p1, p2)| if p1 != p2 { 1 } else { 0 })
        .sum();

    if n_different_pixels == 0 {
        return (Difference::None, left_info, right_info);
    }

    let mut distance_sum: u64 = 0;

    let diff_image_data: Vec<u8> = left
        .pixels()
        .zip(right.pixels())
        .flat_map(|(p1, p2)| {
            let (abs_v, v) = compute_distance(p1, p2);
            distance_sum += abs_v as u64;
            if v < 0 {
                [abs_v as u8, 0, 0]
            } else {
                [0, abs_v as u8, 0]
            }
        })
        .collect();
    let diff_image = RgbImage::from_vec(left.width(), left.height(), diff_image_data).unwrap();
    (
        Difference::Content {
            n_different_pixels,
            distance_sum,
            diff_image,
        },
        left_info,
        right_info,
    )
}

fn compute_distance(p1: &Rgb<u8>, p2: &Rgb<u8>) -> (i32, i32) {
    p1.channels()
        .iter()
        .zip(p2.channels())
        .fold((0, 0), |(abs_v, v), (c1, c2)| {
            let new = (*c2 as i32) - (*c1 as i32);
            let abs_new = new.abs();
            if abs_new > abs_v {
                (abs_new, new)
            } else {
                (abs_v, v)
            }
        })
}

pub(crate) fn compute_differences(pairs: Vec<Pair>) -> Vec<PairResult> {
    pairs
        .into_iter()
        .map(|pair| {
            let (difference, left_info, right_info) = compute_pair_diff(&pair);
            PairResult {
                pair,
                difference,
                left_info,
                right_info,
            }
        })
        .collect()
}
