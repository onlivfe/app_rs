//! Fully rust powered [onlivfe](https://onlivfe.com) app.
//!
//! Very WIP alternative to compete against the [web based desktop app](https://github.com/onlivfe/desktop).

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![deny(unsafe_code)]
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
fn main() {
	onlivfe_wrapper::init(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
		.expect("Initialization failed");

	#[cfg(not(any(target_os = "android", target_os = "ios")))]
	{
		dioxus_desktop::launch_cfg(
			app::Onlivfe,
			dioxus_desktop::Config::new().with_custom_head(
				STYLES.map(|style| format!("<style>{style}</style>")).join("\n"),
			),
		);
	}
	#[cfg(any(target_os = "android", target_os = "ios"))]
	{
		dioxus_mobile::launch_cfg(
			app::Onlivfe,
			dioxus_mobile::Config::new().with_custom_head(
				STYLES.map(|style| format!("<style>{style}</style>")).join("\n"),
			),
		);
	}
}
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
	match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
		Ok(t) => t,
		Err(err) => {
			eprintln!("attempt to unwind out of `rust` with err: {err:?}");
			std::process::abort()
		}
	}
}

/// Starts the app
///
/// # Panics
///
/// If app crashes
#[allow(unsafe_code)]
#[no_mangle]
#[inline(never)]
pub extern "C" fn start_app() {
	//#[cfg(target_os = "android")]
	//wry::android_binding!(com_onlivfe, onlivfe, _start_app);
	stop_unwind(main);
}
