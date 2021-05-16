//! This crate provides an indexer for videos downloaded with youtube-dl.
//! In order for this crate to work properly,
//! youtube-dl _must_ have been invoked with
//! `--write-info-json` and `--write-thumbnail`.

extern crate serde_json;

pub use crate::info_json::*;

mod info_json;
