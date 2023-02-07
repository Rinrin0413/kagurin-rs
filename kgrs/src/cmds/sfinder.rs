//! Functions for solution-finder

use std::{fs, path::Path};

/// Returns a path to the output directory for the solution-finder.
pub fn output_dir() -> String {
    format!("{}/../output", env!("CARGO_MANIFEST_DIR"))
}

/// Initializes the output directory for the solution-finder.
pub fn init_output_dir() {
    let output_dir = output_dir();
    if Path::new(&output_dir).exists() {
        fs::remove_dir_all(&output_dir).unwrap();
    }
}
