//! Test harness for [`all_is_cubes`]’ various renderers, supporting comparing the
//! image outputs of various renderers with each other and reference images.

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::needless_update)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_lifetimes)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::wrong_self_convention)]
#![cfg_attr(test,
    allow(clippy::float_cmp), // Tests work with predictable floats
    allow(clippy::redundant_clone), // Tests prefer regularity over efficiency
)]

use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use image::RgbaImage;

use all_is_cubes::character::Character;
use all_is_cubes::space::Space;
use all_is_cubes::universe::{Universe, UniverseIndex};

mod diff;
pub use diff::*;
mod harness;
pub use harness::*;
mod histogram;
pub use histogram::*;
mod image_files;
pub use image_files::*;
mod render;
pub use render::*;
mod report;
pub use report::*;
pub mod test_cases;

pub type TestId = String;

/// Given a [`Space`], create the [`Character`] looking at it, with the default name.
pub fn finish_universe_from_space(universe: &mut Universe, space: Space) {
    // TODO: "character".into() shouldn't be sprinkled around various universe construction.
    let space_ref = universe.insert("space".into(), space).unwrap();
    let _character_ref = universe
        .insert("character".into(), Character::spawn_default(space_ref))
        .unwrap();
}

/// Result of calling [`compare_rendered_image`]
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComparisonRecord {
    expected_file_name: String,
    actual_file_name: String,
    diff_file_name: Option<String>,
    diff_histogram: Vec<usize>, // length 256; is a Vec for serializability
    outcome: ComparisonOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ComparisonOutcome {
    /// Images had no differences above the threshold.
    Equal,
    /// Images were different by more than the threshold.
    Different { amount: u8 },
    /// There was no expected image to compare against.
    NoExpected,
    /// The images were different, but the renderer signaled a known flaw.
    /// The string is a list of flaws, of unspecified syntax.
    Flawed(String),
}

impl ComparisonRecord {
    fn from_paths(
        expected_file_path: &Path,
        actual_file_path: &Path,
        diff_file_path: Option<&Path>,
        diff_histogram: Histogram,
        outcome: ComparisonOutcome,
    ) -> Self {
        ComparisonRecord {
            expected_file_name: expected_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            actual_file_name: actual_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            diff_file_name: diff_file_path
                .map(|p| p.file_name().unwrap().to_str().unwrap().to_string()),
            diff_histogram: diff_histogram.0.into_iter().collect(),
            outcome,
        }
    }

    fn panic_if_unsuccessful(&self) {
        match self.outcome {
            ComparisonOutcome::Equal | ComparisonOutcome::Flawed(_) => {}
            ComparisonOutcome::Different { amount } => {
                // TODO: show filenames
                panic!("Image mismatch! ({amount})");
            }
            ComparisonOutcome::NoExpected => {
                panic!(
                    "Expected image not found; no comparison done: {p}",
                    p = self.expected_file_name
                );
            }
        }
    }
}

/// Finish a rendering test by storing/displaying/comparing the output image.
pub fn compare_rendered_image(
    test: ImageId,
    allowed_difference: Threshold,
    actual_image: RgbaImage,
) -> ComparisonRecord {
    let actual_file_path = image_path(&test, Version::Actual);
    let diff_file_path = image_path(&test, Version::Diff);

    actual_image
        .save(&actual_file_path)
        .expect("failed to write renderer output image");

    // TODO: all this needs a bunch of code deduplication for conveniently trying-to-open and carrying around the image and path

    // Load expected image, if any
    let (expected_image, expected_file_path): (RgbaImage, PathBuf) =
        match load_and_copy_expected_image(&test) {
            Ok(r) => r,
            Err(NotFound(_)) => {
                // Look for a generic all-renderers output file
                match load_and_copy_expected_image(&ImageId {
                    renderer: RendererId::All,
                    ..test
                }) {
                    Ok(r) => r,
                    Err(NotFound(expected_file_path)) => {
                        return ComparisonRecord::from_paths(
                            &expected_file_path,
                            &actual_file_path,
                            None,
                            Histogram::ZERO,
                            ComparisonOutcome::NoExpected,
                        );
                    }
                }
            }
        };

    // Compare expected and actual images
    let diff_result = diff::diff(&expected_image, &actual_image);
    if let Some(image) = &diff_result.diff_image {
        image
            .save(&diff_file_path)
            .expect("failed to write renderer diff image");
    } else {
        match fs::remove_file(&diff_file_path) {
            Ok(()) => {}
            Err(e) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
            Err(e) => {
                panic!(
                    "failed to delete renderer diff image {p}: {e}",
                    p = diff_file_path.display()
                )
            }
        }
    }

    ComparisonRecord::from_paths(
        &expected_file_path,
        &actual_file_path,
        Some(&diff_file_path),
        diff_result.histogram,
        if diff_result.equal_or_different_below_threshold(allowed_difference) {
            ComparisonOutcome::Equal
        } else {
            ComparisonOutcome::Different {
                amount: diff_result
                    .histogram
                    .0
                    .iter()
                    .copied()
                    .enumerate()
                    .rev() // find highest difference
                    .find(|&(_, v)| v > 0)
                    .map(|(i, _)| i as u8)
                    .unwrap_or(0),
            }
        },
    )
}

pub fn initialize_logging() {
    // Note: Something like this log configuration also appears in all-is-cubes-desktop.
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::ConfigBuilder::new()
            .add_filter_ignore_str("wgpu") // noisy
            .add_filter_ignore_str("naga") // noisy
            .add_filter_ignore_str("winit") // noisy at Trace level only
            .build(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();
}
