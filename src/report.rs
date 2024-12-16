// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::difference::{Difference, ImageInfoResult, PairResult, Size};
use crate::ReportConfig;
use base64::prelude::*;
use chrono::SubsecRound;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

const ICON: &[u8] = include_bytes!("../docs/logo_small.png");
const IMAGE_SIZE_LIMIT: u32 = 400;

fn embed_png_url(data: &[u8]) -> String {
    let mut url = "data:image/png;base64,".to_string();
    url.push_str(&base64::engine::general_purpose::STANDARD.encode(data));
    url
}

fn render_image(
    config: &ReportConfig,
    image_info: &ImageInfoResult,
    path: &Path,
) -> crate::Result<Markup> {
    Ok(match image_info {
        ImageInfoResult::Loaded(info) => {
            let (w, h) = html_size(&info.size, IMAGE_SIZE_LIMIT);
            let path = if config.embed_images {
                let image_data = std::fs::read(path)?;
                embed_png_url(&image_data)
            } else {
                path.display().to_string()
            };
            html! {
                img class="zoom" src=(path) width=[w] height=[h] onclick="openImageDialog(this)";
            }
        }
        ImageInfoResult::Missing => {
            html! { "File is missing" }
        }
        ImageInfoResult::Error(err) => {
            html! { "Error: " (err) }
        }
    })
}

pub fn html_size(size: &Size, size_limit: u32) -> (Option<u32>, Option<u32>) {
    if size.width > size.height {
        (Some(size.width.min(size_limit)), None)
    } else {
        (None, Some(size.height.min(size_limit)))
    }
}

fn render_difference_image(difference: &Difference) -> Markup {
    match difference {
        Difference::None
        | Difference::LoadError
        | Difference::MissingFile
        | Difference::SizeMismatch => html!("N/A"),
        Difference::Content { diff_image, .. } => {
            let (w, h) = html_size(
                &Size::new(diff_image.width(), diff_image.height()),
                IMAGE_SIZE_LIMIT,
            );
            let mut data = Vec::new();
            diff_image
                .write_to(&mut Cursor::new(&mut data), image::ImageFormat::Png)
                .unwrap();
            html! {
                img class="zoom" src=(embed_png_url(&data)) width=[w] height=[h] onclick="openImageDialog(this)";
            }
        }
    }
}

fn render_stat_item(label: &str, value_type: &str, value: &str) -> Markup {
    html! {
        div .stat-item {
            div .stat-label {
                (label)
            }
            @let value_class = format!("stat-value {}", value_type);
            div class=(value_class) {
                (value)
            }
        }
    }
}

fn render_difference_info(config: &ReportConfig, pair_diff: &PairResult) -> Markup {
    match &pair_diff.difference {
        Difference::None => render_stat_item("Status", "ok", "Match"),
        Difference::LoadError => render_stat_item("Status", "error", "Loading error"),
        Difference::MissingFile => render_stat_item("Status", "error", "Missing file"),
        Difference::SizeMismatch => html! {
            (render_stat_item("Status", "error", "Size mismatch"))
            (render_stat_item(&format!("{} size", config.left_title), "", &pair_diff.left_info.info().unwrap().size.to_string()))
            (render_stat_item(&format!("{} size", config.right_title), "", &pair_diff.right_info.info().unwrap().size.to_string()))
        },
        Difference::Content {
            n_different_pixels,
            distance_sum,
            ..
        } => {
            let size = &pair_diff.left_info.info().unwrap().size;
            let n_pixels = size.width as f32 * size.height as f32;
            let pct = *n_different_pixels as f32 / n_pixels * 100.0;
            let distance_sum = *distance_sum as f32 / 255.0; // Normalize
            let avg_color_distance = distance_sum / n_pixels;
            html! {
                (render_stat_item("Different pixels", "warning", &format!("{n_different_pixels} ({pct:.1}%)")))
                (render_stat_item("Color distance", "", &format!("{distance_sum:.3}")))
                (render_stat_item("Avg. color distance", "", &format!("{avg_color_distance:.4}")))
            }
        }
    }
}

fn render_pair_diff(config: &ReportConfig, pair_diff: &PairResult) -> crate::Result<Markup> {
    Ok(html! {
        div class="diff-entry" {
            h2 {(pair_diff.pair.title)};
            div class="comparison-container" {
                div class="image-container" {
                    div class="stats-container" {
                        (render_difference_info(config, &pair_diff))
                    }
                    div class="image-box" {
                        h3 { (config.left_title) }
                        (render_image(config, &pair_diff.left_info, &pair_diff.pair.left)?)
                    }
                    div class="image-box" {
                        h3 { (config.right_title) }
                        (render_image(config, &pair_diff.right_info, &pair_diff.pair.right)?)
                    }
                    div class="image-box" {
                        h3 { "Difference"}
                        (render_difference_image(&pair_diff.difference))
                    }
                }
            }
        }
    })
}

const CSS_STYLE: &str = "
body {
    font-family: Roboto, sans-serif;
    margin: 0;
    padding: 20px;
    background: #f5f5f5;
    color: #333;
}

.header {
    background: #fff;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.logo {
    vertical-align: -10%;
}

.header h1 {
    margin: 0;
    color: #2d3748;
}

.summary {
    margin-bottom: 20px;
    padding: 15px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.diff-entry {
    background: #fff;
    margin-bottom: 30px;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.diff-entry h2 {
    margin-top: 0;
    color: #2d3748;
    border-bottom: 2px solid #edf2f7;
    padding-bottom: 10px;
}

.comparison-container {
    display: flex;
    gap: 20px;
    margin-top: 15px;
}

.image-container {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    flex: 1;
}

.image-box {
    flex: 1;
    min-width: 250px;
    max-width: 400px;
}

.image-box h3 {
    margin: 0 0 10px 0;
    color: #4a5568;
    font-size: 1rem;
}

.image-box img {
    max-width: 100%;
    border: 1px solid #e2e8f0;
    border-radius: 4px;
}

.stats-container {
    width: 200px;
    flex-shrink: 0;
    background: #f8fafc;
    padding: 15px;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
}

.stat-item {
    margin-bottom: 15px;
}

.stat-label {
    font-size: 0.875rem;
    color: #64748b;
    margin-bottom: 4px;
}

.stat-value {
    font-size: 1.25rem;
    font-weight: 600;
    color: #2d3748;
}

.stat-value.ok {
    color: #77d906;
}

.stat-value.warning {
    color: #d97706;
}

.stat-value.error {
    color: #dc2626;
}

@media (max-width: 1200px) {
    .comparison-container {
        flex-direction: column-reverse;
    }

    .stats-container {
        width: auto;
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
    }

    .stat-item {
        flex: 1;
        min-width: 150px;
        margin-bottom: 0;
    }
}

@media (max-width: 768px) {
    .image-box {
        min-width: 100%;
    }
}

img.zoom:hover {
    transform: scale(1.05);
}

dialog {
    width: 80%;
    height: 80%;
    max-width: 800px;
    max-height: 820px;
    padding: 0;
    border: none;
    border-radius: 10px;
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
}

.zoomed-image {
    object-fit: contain;
    image-rendering: -moz-crisp-edges;
    image-rendering: -o-crisp-edges;
    image-rendering: -webkit-optimize-contrast;
    -ms-interpolation-mode: nearest-neighbor;
    image-rendering: pixelated;
}
";

const JS_CODE: &str = "
function openImageDialog(img) {
    const dialog = document.getElementById('imageDialog');
    const zoomedImg = document.getElementById('zoomedImage');
    zoomedImg.src = img.src;
    if (img.width < img.height) {
        zoomedImg.style.width = \"100%\";
        zoomedImg.style.height = \"auto\";
    } else {
        zoomedImg.style.width = \"auto\";
        zoomedImg.style.height = \"100%\";
    }
    dialog.showModal();
}

function closeImageDialog() {
    const dialog = document.getElementById('imageDialog');
    dialog.close();
}

document.getElementById('imageDialog').addEventListener('click', function(event) {
    closeImageDialog();
});
";

pub(crate) fn create_html_report(
    config: &ReportConfig,
    diffs: &[PairResult],
    output: &Path,
) -> crate::Result<()> {
    let now = chrono::Local::now().round_subsecs(0);
    let report = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Image diff" }
                style { (PreEscaped(CSS_STYLE)) }
                link rel="icon" type="image/png" href=(embed_png_url(&ICON));
            }
            body {
                 div class="header" {
                    h1 { img class="logo" src=(embed_png_url(ICON)) width="32" height="32"; "Kompari Report" }
                    p { "Generated on " (now) }
                }
                dialog id="imageDialog" {
                    img id="zoomedImage" class="zoomed-image" src="" alt="Zoomed Image";
                }
                script { (PreEscaped(JS_CODE)) }
                @for pair_diff in diffs {
                   (render_pair_diff(config, pair_diff)?)
                }
            }
        }
    };
    let mut file = File::create(output)?;
    file.write_all(report.into_string().as_bytes())?;
    Ok(())
}
