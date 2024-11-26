use crate::difference::{Difference, ImageInfo, ImageInfoResult, PairResult};
use chrono::SubsecRound;
use maud::{html, Markup, DOCTYPE};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn render_image(image_info: &ImageInfoResult, path: &Path) -> Markup {
    let size_limit = 400;
    match image_info {
        Ok(info) => {
            let size = &info.size;
            let (w, h) = if size.width > size.height {
                (Some(size.width.min(size_limit)), None)
            } else {
                (None, Some(size.height.min(size_limit)))
            };
            html! {
                img src=(path.display()) width=[w] height=[h];
            }
        }
        Err(err) => {
            html! { "Error: " (err) }
        }
    }
}

fn render_difference_image(difference: &Difference) -> Markup {
    html!("N/A")
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

fn render_difference_info(pair_diff: &PairResult) -> Markup {
    match &pair_diff.difference {
        Difference::None => render_stat_item("Status", "ok", "Match"),
        Difference::LoadError => render_stat_item("Status", "error", "Loading error"),
        Difference::SizeMismatch => html! {
            (render_stat_item("Status", "error", "Size mismatch"))
            (render_stat_item("Left size", "", &pair_diff.left_info.as_ref().unwrap().size.to_string()))
            (render_stat_item("Right size", "", &pair_diff.right_info.as_ref().unwrap().size.to_string()))
        },
        Difference::ContentDifference { .. } => {
            todo!()
        }
    }
}

fn render_pair_diff(pair_diff: &PairResult) -> Markup {
    html! {
        div class="diff-entry" {
            h2 {(pair_diff.pair.title)};
            div class="comparison-container" {
                div class="image-container" {
                    div class="stats-container" {
                        (render_difference_info(&pair_diff))
                    }
                    div class="image-box" {
                        h3 { "Left image"}
                        (render_image(&pair_diff.left_info, &pair_diff.pair.left))
                    }
                    div class="image-box" {
                        h3 { "Right image"}
                        (render_image(&pair_diff.right_info, &pair_diff.pair.right))
                    }
                    div class="image-box" {
                        h3 { "Difference"}
                        (render_difference_image(&pair_diff.difference))
                    }

                    //     <div class="image-box">
            //         <h3>Left Image</h3>
            //         <img src="/api/placeholder/400/300" alt="Left image">
            //     </div>
            //     <div class="image-box">
            //         <h3>Right Image</h3>
            //         <img src="/api/placeholder/400/300" alt="Right image">
            //     </div>
            //     <div class="image-box">
            //         <h3>Difference</h3>
            //         <img src="/api/placeholder/400/300" alt="Difference visualization">
            //     </div>
            // </div>
            // <div class="stats-container">
            //     <div class="stat-item">
            //         <div class="stat-label">Different Pixels</div>
            //         <div class="stat-value warning">15.3%</div>
            //     </div>
            //     <div class="stat-item">
            //         <div class="stat-label">Mean Difference</div>
            //         <div class="stat-value">0.142</div>
            //     </div>
            //     <div class="stat-item">
            //         <div class="stat-label">Max Difference</div>
            //         <div class="stat-value">0.856</div>
            //     </div>
            //     <div class="stat-item">
            //         <div class="stat-label">SSIM Score</div>
            //         <div class="stat-value">0.925</div>
            //     </div>
            //     <div class="stat-item">
            //         <div class="stat-label">Image Size</div>
            //         <div class="stat-value">800×600</div>
            //     </div>
            }
        }
        // <div class="metadata">
        //     <p>Path: /path/to/image1.png</p>
        //     <p>Difference score: 0.15</p>
        // </div>
        }
    }
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

.metadata {
    margin-top: 15px;
    padding-top: 15px;
    border-top: 1px solid #edf2f7;
    font-size: 0.9rem;
    color: #718096;
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
";

pub(crate) fn create_report(diffs: Vec<PairResult>) -> anyhow::Result<()> {
    let now = chrono::Local::now().round_subsecs(0);
    let report = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Image diff" }
                style { (CSS_STYLE) }
            }
            body {
                 div class="header" {
                    h1 { "Image Diff Report" }
                    p { "Generated on " (now) }
                }
                @for pair_diff in &diffs {
                   (render_pair_diff(pair_diff))
                }
            }
        }
    };
    let mut file = File::create("report.html")?;
    file.write_all(report.into_string().as_bytes())?;
    Ok(())
}