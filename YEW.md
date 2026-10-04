# Y OTP RS Yew Usage

Adding OTP RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the OTP RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add otprs --features=yew
   ```

1. Import the `Otp`, `Group`, `Separator`, and `Slot` components into your Yew component and start using them in your app.

## 🛠️ Usage

### Basic 6-Digit OTP

```rust
use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(BasicOtp)]
pub fn basic_otp() -> Html {
    html! {
        <Otp max_length={6} aria_label="Enter verification code">
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
            </Group>
            <Separator />
            <Group>
                <Slot index={3} />
                <Slot index={4} />
                <Slot index={5} />
            </Group>
        </Otp>
    }
}
```

### Controlled with `on_complete`

```rust
use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(ControlledOtp)]
pub fn controlled_otp() -> Html {
    let value = use_state(String::new);
    let on_change = {
        let value = value.clone();
        Callback::from(move |v: String| value.set(v))
    };
    let on_complete = Callback::from(|code: String| {
        web_sys::console::log_1(&format!("Complete: {code}").into());
    });
    html! {
        <Otp
            max_length={6}
            value={(*value).clone()}
            on_change={on_change}
            on_complete={on_complete}
            aria_label="Verification code"
        >
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
            </Group>
            <Separator />
            <Group>
                <Slot index={3} />
                <Slot index={4} />
                <Slot index={5} />
            </Group>
        </Otp>
    }
}
```

### Disabled State

```rust
use otprs::yew::{Otp, Group, Slot};
use yew::prelude::*;

#[function_component(DisabledOtp)]
pub fn disabled_otp() -> Html {
    html! {
        <Otp max_length={4} is_disabled={true} aria_label="Disabled PIN">
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
                <Slot index={3} />
            </Group>
        </Otp>
    }
}
```

### Secondary Variant (for Surface backgrounds)

```rust
use otprs::yew::{Otp, Group, Separator, Slot};
use otprs::Variant;
use yew::prelude::*;

#[function_component(SecondaryOtp)]
pub fn secondary_otp() -> Html {
    html! {
        <Otp max_length={6} variant={Variant::Secondary} aria_label="Enter code">
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
            </Group>
            <Separator />
            <Group>
                <Slot index={3} />
                <Slot index={4} />
                <Slot index={5} />
            </Group>
        </Otp>
    }
}
```

## 🔧 Props

### `Otp`

| Property           | Type               | Description                     | Default               |
| ------------------ | ------------------ | ------------------------------- | --------------------- |
| `max_length`       | `usize`            | Total slot count.               | `6`                   |
| `value`            | `String`           | Controlled value.               | `""`                  |
| `on_change`        | `Callback<String>` | Called on every keystroke.      | no-op                 |
| `on_complete`      | `Callback<String>` | Called when all slots filled.   | no-op                 |
| `is_disabled`      | `bool`             | Disables all slots.             | `false`               |
| `is_invalid`       | `bool`             | Shows error state on all slots. | `false`               |
| `variant`          | `Variant`          | `Primary` or `Secondary`.       | `Primary`             |
| `pattern`          | `&'static str`     | Allowed character regex.        | `"[0-9]"`             |
| `input_mode`       | `InputMode`        | Mobile keyboard type.           | `Numeric`             |
| `name`             | `&'static str`     | Hidden input name for forms.    | `""`                  |
| `auto_focus`       | `bool`             | Focus first slot on mount.      | `false`               |
| `class`            | `&'static str`     | Extra CSS class on root.        | `""`                  |
| `style`            | `&'static str`     | Extra inline CSS on root.       | `""`                  |
| `id`               | `&'static str`     | `id` on root element.           | `""`                  |
| `aria_label`       | `&'static str`     | Screen-reader label.            | `"One-time password"` |
| `aria_describedby` | `&'static str`     | Error description element id.   | `""`                  |
| `container_class`  | `&'static str`     | Vlass on inner container.       | `""`                  |
| `container_style`  | `&'static str`     | Inline CSS on inner container.  | `""`                  |

### `Slot`

| Property | Type           | Description                          | Default |
| -------- | -------------- | ------------------------------------ | ------- |
| `index`  | `usize`        | Zero-based slot index. **Required.** | -       |
| `class`  | `&'static str` | Extra CSS on slot wrapper.           | `""`    |
| `style`  | `&'static str` | Extra inline CSS on slot.            | `""`    |
| `id`     | `&'static str` | `id` on slot wrapper.                | `""`    |

### `Group`

| Property | Type           | Description            | Default |
| -------- | -------------- | ---------------------- | ------- |
| `class`  | `&'static str` | Extra CSS on group.    | `""`    |
| `style`  | `&'static str` | Inline CSS on group.   | `""`    |
| `id`     | `&'static str` | `id` on group element. | `""`    |

### `Separator`

| Property | Type           | Description              | Default |
| -------- | -------------- | ------------------------ | ------- |
| `class`  | `&'static str` | Extra CSS on separator.  | `""`    |
| `style`  | `&'static str` | Inline CSS on separator. | `""`    |

## 💡 Notes

- `Slot` **must** be a descendant of `Otp`, it reads context from the parent.
- The caret blinks inside the active empty slot for a native feel.
- Use `pattern={otprs::pattern::CHARS_ONLY}` for letter-only codes.
- Use `is_invalid={true}` together with `aria_describedby` to surface errors accessibly.

## 🔗 See Also

- [Input RS](https://crates.io/crates/input-rs): The unstyled `<input>` abstraction layer powering the invisible focus engine.
- [RFC 4226: HOTP](https://rfc-editor.org/info/rfc4226): An HMAC-Based One-Time Password Algorithm.
- [RFC 6238: TOTP](https://rfc-editor.org/info/rfc6238): Time-Based One-Time Password Algorithm.
- [totp-rs](https://crates.io/crates/totp-rs): The underlying Rust crate used by `otprs` for TOTP validation.
