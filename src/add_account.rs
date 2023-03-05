use std::sync::Arc;

use dioxus::{html::*, prelude::*};
use onlivfe::PlatformType;
use strum::IntoEnumIterator;

#[must_use]
pub fn Page(cx: Scope) -> Element {
	let interface = use_shared_state::<crate::Onlivfe>(cx)?.read().clone();

	let platforms_fut = use_future(cx, (), |_| async move {
		PlatformType::iter().map(|platform| interface.check_auth(platform));

		vec![PlatformType::VRChat, PlatformType::ChilloutVR]
	});

	let mut selected_platform = use_state(cx, || None::<PlatformType>);

	cx.render(match platforms_fut.value() {
		Some(available_platforms) => rsx! {
			h1 {"Add account"}
			select {
				for platform in available_platforms.iter() {
					option {
						key: "{platform.as_ref()}",
						platform.as_ref()
					}
				}
			},
			form {
				onsubmit: move |event| {
						println!("Submitted! {event:?}")
				},
				input { name: "name", },
				input { name: "age", },
				input { name: "date", },
				input { r#type: "submit", },
			}
		},
		None => rsx! {
			progress {"indeterminate progress"}
		},
	})
}
