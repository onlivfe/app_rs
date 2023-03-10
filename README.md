# Onlivfe app_rs

A rust-native [onlivfe app](https://onlivfe.com) built using [dioxus](https://dioxuslabs.com), working as an alternative to the [full onlivfe desktop app](https://github.com/onlivfe/desktop) that's built with web technologies & [tauri](https://tauri.app).

Also note that the license is [AGPL](https://tldrlegal.com/license/gnu-affero-general-public-license-v3-(agpl-3.0)).

## Development

Basic requirements:

- [Git](https://git-scm.com)
- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://github.com/DioxusLabs/cli) (`cargo install dioxus-cli`)

### Basics

Start off by cloning the project with git.

```sh
git clone https://github.com/onlivfe/core
```

Then open the project folder in your terminal, & run `dioxus serve --hot-reload`.
Then get to hacking, & optionally replace the dependency in other projects by [overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html).

### Android

Possibly need to add `abiFilters += listOf("arm64-v8a")` under `create("arm")` branch in `:app`'s '`build.gradle.kts`.
