//! Fully rust powered [onlivfe](https://onlivfe.com) app.
//!
//! Very WIP alternative to compete against the [web based desktop app](https://github.com/onlivfe/desktop).

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]
// Dioxus components
#![allow(non_snake_case)]

use std::sync::Arc;

pub mod about;
pub mod add_account;
pub mod app;
pub mod dash;
pub mod settings;

static STYLES: [&str; 2] =
	[include_str!("../res/hiq.css"), include_str!("../res/styles.css")];

type Onlivfe = Arc<
	onlivfe_wrapper::Onlivfe<onlivfe_cache_store::OnlivfeCacheStorageBackend>,
>;

/// Starts the application
///
/// # Errors
///
/// If the app encountered an error whilst starting or running
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
	dioxus_desktop::launch_cfg(
		app::Onlivfe,
		dioxus_desktop::Config::new().with_custom_head(
			STYLES.map(|style| format!("<style>{style}</style>")).join("\n"),
		),
	);

	Ok(())
}
