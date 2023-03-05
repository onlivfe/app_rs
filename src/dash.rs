use std::sync::Arc;

use dioxus::prelude::*;

#[must_use]
pub fn Page(cx: Scope) -> Element {
	let interface = use_shared_state::<crate::Onlivfe>(cx)?;

	cx.render(rsx! {
		h1 {"Dash"}
	})
}
