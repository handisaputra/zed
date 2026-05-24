# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed v1.2.6 with traffic lights on Windows, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter) modified by [Handisaputra](https://github.com/handisaputra).

![Zed with Traffic Lights on Windows](https://github.com/handisaputra/zed/blob/macos_traffic_lights_on_windows/assets/images/zed_with_traffic_lights_on_windows.png)

---

### Installation

On Windows you can [download Zed directly](https://github.com/handisaputra/zed/releases).


### Developing Zed

- [Building Zed for Windows](./docs/src/development/windows.md).

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... Zed is hiring! Check out [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship
* When you love Zed on Windows with traffic lights you could [buy me a coffee](https://www.paypal.com/paypalme/h4ndy).

* Zed is developed by **Zed Industries, Inc.**, a for-profit company.


  If you’d like to financially support the project, you can do so via GitHub Sponsors.
  Sponsorships go directly to Zed Industries and are used as general company revenue.
  There are no perks or entitlements associated with sponsorship.
