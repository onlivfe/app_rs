use dioxus::prelude::*;
use dioxus_router::Link;
use onlivfe::PlatformType;
use strum::IntoEnumIterator;

#[must_use]
pub fn Page(cx: Scope) -> Element {
	let interface = use_shared_state::<crate::Onlivfe>(cx)?.read().clone();

	let accounts_fut = use_future(cx, (), |_| async move {
		// TODO: Join into single future
		PlatformType::iter().map(|platform| interface.check_auth(platform));

		vec![PlatformType::NeosVR]
	});

	let Accounts = accounts_fut.value().map(|accounts| {
		rsx! {
			h2 {"Accounts"}
			for account in accounts.iter() {
				li {
					key: "{account.as_ref()}",
					p { "Authenticated on {account.as_ref()}" }
				}
			}
			Link {
				class: "button"
					to: "/add_account",
					"Add account"
			}
		}
	});

	cx.render(rsx! {
		h1 {"Settings"}
		Accounts
	})
}
