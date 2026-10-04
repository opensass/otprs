# 🧬 OTP RS Dioxus Usage

Adding OTP RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Follow their [Getting Started Guide](https://dioxuslabs.com/learn/0.7/getting_started) for setup instructions.

1. Add the OTP RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add otprs --features=dio
   ```

1. Import the `Otp`, `Group`, `Separator`, and `Slot` components into your Dioxus component and start using them in your app.

## 🛠️ Usage

### Basic 6-Digit OTP

```rust
use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn BasicOtp() -> Element {
    rsx! {
        Otp { max_length: 6, aria_label: "Enter verification code",
            Group {
                Slot { index: 0 }
                Slot { index: 1 }
                Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 }
                Slot { index: 4 }
                Slot { index: 5 }
            }
        }
    }
}
```

### Controlled with `on_complete`

```rust
use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn ControlledOtp() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        Otp {
            max_length: 6,
            value: value(),
            on_change: Some(Callback::new(move |v: String| value.set(v))),
            on_complete: Some(Callback::new(|code: String| {
                dioxus_logger::tracing::info!("Complete: {code}");
            })),
            aria_label: "Verification code",
            Group {
                Slot { index: 0 }
                Slot { index: 1 }
                Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 }
                Slot { index: 4 }
                Slot { index: 5 }
            }
        }
    }
}
```

### 4-Digit PIN

```rust
use otprs::dioxus::{Otp, Group, Slot};
use dioxus::prelude::*;

#[component]
fn PinOtp() -> Element {
    rsx! {
        Otp { max_length: 4, aria_label: "Enter PIN",
            Group {
                Slot { index: 0 }
                Slot { index: 1 }
                Slot { index: 2 }
                Slot { index: 3 }
            }
        }
    }
}
```

## 🔧 Props

### `Otp`

| Property          | Type                       | Description                   | Default               |
| ----------------- | -------------------------- | ----------------------------- | --------------------- |
| `max_length`      | `usize`                    | Total slot count.             | `6`                   |
| `value`           | `String`                   | Controlled value.             | `""`                  |
| `on_change`       | `Option<Callback<String>>` | Called on every keystroke.    | `None`                |
| `on_complete`     | `Option<Callback<String>>` | Called when all slots filled. | `None`                |
| `is_disabled`     | `bool`                     | Disables all slots.           | `false`               |
| `is_invalid`      | `bool`                     | Shows error state.            | `false`               |
| `variant`         | `Variant`                  | `Primary` or `Secondary`.     | `Primary`             |
| `pattern`         | `&'static str`             | Allowed character regex.      | `"[0-9]"`             |
| `input_mode`      | `InputMode`                | Mobile keyboard type.         | `Numeric`             |
| `name`            | `&'static str`             | Hidden input name.            | `""`                  |
| `auto_focus`      | `bool`                     | Focus first slot on mount.    | `false`               |
| `class`           | `&'static str`             | Extra CSS class on root.      | `""`                  |
| `style`           | `&'static str`             | Inline CSS on root.           | `""`                  |
| `id`              | `&'static str`             | `id` on root element.         | `""`                  |
| `aria_label`      | `&'static str`             | Screen-reader label.          | `"One-time password"` |
| `container_class` | `&'static str`             | Class on inner container.     | `""`                  |
| `container_style` | `&'static str`             | CSS on inner container.       | `""`                  |

### `Slot`

| Property | Type           | Description                          | Default |
| -------- | -------------- | ------------------------------------ | ------- |
| `index`  | `usize`        | Zero-based slot index. **Required.** | -       |
| `class`  | `&'static str` | Extra CSS on slot.                   | `""`    |
| `style`  | `&'static str` | Inline CSS on slot.                  | `""`    |
| `id`     | `&'static str` | `id` on slot.                        | `""`    |

### `Group` / `Separator`

| Property | Type           | Description     | Default |
| -------- | -------------- | --------------- | ------- |
| `class`  | `&'static str` | Extra CSS.      | `""`    |
| `style`  | `&'static str` | Inline CSS.     | `""`    |
| `id`     | `&'static str` | `id` attribute. | `""`    |

## 💡 Notes

- `Slot` **must** be inside an `Otp`, it reads context from the parent.
- **Focus traversal**: Typing a character auto-advances to the next slot; pressing `Backspace` clears the current slot and retreats focus to the previous one. Focus stops at the last slot when the OTP is full.
- **Multiple instances**: Each `Otp` on the page gets a unique internal ID, so placing several `Otp` widgets on the same page is fully supported without conflict.
- Use `is_invalid: true` together with `aria_describedby` to surface errors accessibly.
- Use `pattern: "[A-Za-z0-9]"` for alphanumeric codes instead of the default `"[0-9]"`.

## 🔗 See Also

- [Input RS](https://crates.io/crates/input-rs): The unstyled `<input>` abstraction layer powering the invisible focus engine.
- [RFC 4226: HOTP](https://rfc-editor.org/info/rfc4226): An HMAC-Based One-Time Password Algorithm.
- [RFC 6238: TOTP](https://rfc-editor.org/info/rfc6238): Time-Based One-Time Password Algorithm.
- [totp-rs](https://crates.io/crates/totp-rs): The underlying Rust crate used by `otprs` for TOTP validation.
