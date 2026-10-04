# 🌱 OTP RS Leptos Usage

Adding OTP RS to your project is simple:

1. Make sure your project is set up with **Leptos**. Follow their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add the OTP RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add otprs --features=lep
   ```

1. Import the `Otp`, `Group`, `Separator`, and `Slot` components into your Leptos component and start using them in your app.

## 🛠️ Usage

### Basic 6-Digit OTP

```rust
use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
pub fn BasicOtp() -> impl IntoView {
    view! {
        <Otp max_length=6 aria_label="Enter verification code">
            <Group>
                <Slot index=0 />
                <Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 />
                <Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
    }
}
```

### Controlled with `on_complete`

```rust
use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
pub fn ControlledOtp() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let on_change = Callback::new(move |v: String| set_value.set(v));
    let on_complete = Callback::new(move |code: String| {
        leptos::logging::log!("Complete: {code}");
    });
    view! {
        <Otp
            max_length=6
            value=value.get()
            on_change=on_change
            on_complete=on_complete
            aria_label="Enter your code"
        >
            <Group>
                <Slot index=0 />
                <Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 />
                <Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
    }
}
```

### Invalid State with Error Message

```rust
use otprs::leptos::{Otp, Group, Slot};
use leptos::prelude::*;

#[component]
pub fn InvalidOtp() -> impl IntoView {
    view! {
        <div>
            <Otp
                max_length=6
                is_invalid=true
                aria_label="Invalid verification code"
                aria_describedby="otp-error"
            >
                <Group>
                    <Slot index=0 />
                    <Slot index=1 />
                    <Slot index=2 />
                    <Slot index=3 />
                    <Slot index=4 />
                    <Slot index=5 />
                </Group>
            </Otp>
            <p id="otp-error" role="alert" style="color: #dc2626; margin-top: 4px;">
                "Incorrect code. Please try again."
            </p>
        </div>
    }
}
```

## 🔧 Props

### `Otp`

| Property           | Type                       | Description                   | Default               |
| ------------------ | -------------------------- | ----------------------------- | --------------------- |
| `max_length`       | `usize`                    | Total slot count.             | `6`                   |
| `value`            | `String`                   | Controlled value.             | `""`                  |
| `on_change`        | `Option<Callback<String>>` | Called on every change.       | `None`                |
| `on_complete`      | `Option<Callback<String>>` | Called when all slots filled. | `None`                |
| `is_disabled`      | `bool`                     | Disables all slots.           | `false`               |
| `is_invalid`       | `bool`                     | Shows error state.            | `false`               |
| `variant`          | `Variant`                  | `Primary` or `Secondary`.     | `Primary`             |
| `pattern`          | `&'static str`             | Allowed character regex.      | `"[0-9]"`             |
| `input_mode`       | `InputMode`                | Mobile keyboard type.         | `Numeric`             |
| `name`             | `&'static str`             | Hidden input name.            | `""`                  |
| `auto_focus`       | `bool`                     | Focus first slot on mount.    | `false`               |
| `class`            | `&'static str`             | Extra CSS class on root.      | `""`                  |
| `style`            | `&'static str`             | Inline CSS on root.           | `""`                  |
| `id`               | `&'static str`             | `id` on root element.         | `""`                  |
| `aria_label`       | `&'static str`             | Screen-reader label.          | `"One-time password"` |
| `aria_describedby` | `&'static str`             | Error description element id. | `""`                  |
| `container_class`  | `&'static str`             | Class on inner container.     | `""`                  |
| `container_style`  | `&'static str`             | CSS on inner container.       | `""`                  |

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

- `Slot` **must** be inside an `Otp`, it reads context via Leptos `use_context`.
- **Focus traversal**: After typing a character the focus automatically moves to the next slot. Pressing `Backspace` clears the current slot and retreats focus to the previous one. Focus stops at the last slot when the OTP is full.
- **Multiple instances**: Each `Otp` on the page gets a unique internal ID, so placing several `Otp` widgets on the same page is fully supported without any conflict.
- Use `is_invalid=true` with `aria_describedby` to link error messages accessibly.
- Use `pattern="[A-Za-z0-9]"` for alphanumeric codes instead of the default `"[0-9]"`.
- The blinking caret in the active empty slot is pure CSS, no JS timers.

## 🔗 See Also

- [Input RS](https://crates.io/crates/input-rs): The unstyled `<input>` abstraction layer powering the invisible focus engine.
- [RFC 4226: HOTP](https://rfc-editor.org/info/rfc4226): An HMAC-Based One-Time Password Algorithm.
- [RFC 6238: TOTP](https://rfc-editor.org/info/rfc6238): Time-Based One-Time Password Algorithm.
- [totp-rs](https://crates.io/crates/totp-rs): The underlying Rust crate used by `otprs` for TOTP validation.
