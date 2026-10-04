<div align="center">

# 🔐 OTP RS

[![Crates.io](https://img.shields.io/crates/v/otp-rs)](https://crates.io/crates/otp-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/otp-rs)](https://crates.io/crates/otp-rs)
![Crates.io License](https://img.shields.io/crates/l/otp-rs)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.90%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/opensass)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->

![logo](https://raw.githubusercontent.com/opensass/otprs/refs/heads/main/assets/logo.png)

</div>

## 🎬 Demo

| Framework | Live Demo                                                                                                                                  |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Yew       | [![Netlify Status](hhttps://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://otp-rs.netlify.app) |
| Dioxus    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://otp-dio.netlify.app) |
| Leptos    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://otp-lep.netlify.app) |

## 📜 Intro

A highly customizable, accessible OTP (One-Time Password) input component for WASM frameworks: Yew, Dioxus, and Leptos.

Supports animated per-slot digit entry with automatic focus traversal, `primary` and `secondary` visual variants, built-in HOTP/TOTP validation per RFC 4226 and RFC 6238, and full WCAG compliance.

## 🤔 Why OTP RS?

1. **🎨 Fully Customizable**: Control every attribute, class, style, variant, pattern, ARIA labels and more, on the outermost container, each group, each slot, and each separator.
1. **⌨ Smart Focus Management**: Typing a digit automatically advances focus. Backspace regresses focus. Arrow keys navigate left and right.
1. **✨ Animated Entry**: Each digit pops with a scale animation when entered, and a blinking caret is shown in the active empty slot.
1. **🔐 Built-in Validation**: Validate against HOTP (RFC 4226) or TOTP (RFC 6238) using a pure-Rust implementation, no C, no external crypto crates.
1. **♿ Accessible by Default**: `role="group"`, `aria-label`, `inputmode`, `autocomplete="one-time-code"`, disabled/invalid data-attributes all wired up automatically.
1. **🧩 Framework Agnostic**: Same API semantics across Yew, Dioxus, and Leptos.

## Y Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/otprs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/otprs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/otprs/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make OTP RS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

OTP RS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
