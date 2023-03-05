use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_router::{use_router, Link, Redirect, Route, Router};

/// The main app
///
/// # Panics
///
/// If creating the main interface fails
#[must_use]
pub fn Onlivfe(cx: Scope) -> Element {
	use_shared_state_provider(cx, || {
		let store =
			onlivfe_cache_store::OnlivfeCacheStorageBackend::new("app_rs").unwrap();
		let interface = onlivfe_wrapper::Onlivfe::new(store).unwrap();
		Arc::new(interface)
	});

	cx.render(rsx! {
		// All of our routes will be rendered inside this Router component
		Router {
			TopBar {}
			main {
				class: "container is-fluid overflow-touch",
				Route { to: "/dash", crate::dash::Page {} }
				Route { to: "/add_account", crate::add_account::Page {} }
				Route { to: "/about", crate::about::Page {} }
				Route { to: "/settings", crate::settings::Page {} }
				Route { to: "/quit", Quit {} }
			}
			active_class: "disabled"
			initial_url: "http://localhost/dash".to_owned()
		}
	})
}

fn Quit(cx: Scope) -> Element {
	println!("fuckit shipit itworks");
	std::process::exit(0);
}

fn TopBar(cx: Scope) -> Element {
	let routes = [
		vec![("/dash", "Dash"), ("/about", "About")],
		vec![("/settings", "Settings"), ("/quit", "Quit")],
	];

	cx.render(rsx! {
		nav {
			class: "container is-fluid",
			for routes_group in routes.iter() {
				ul {
					class: "is-unstyled",
					for route in routes_group.iter() {
						li {
							Link {
								class: "button"
									to: route.0,
									route.1
							}
						}
					}
				}
			}
		}
	})
}
