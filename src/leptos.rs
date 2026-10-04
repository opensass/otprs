// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    InputMode, Variant, base_otp_inner_style, base_otp_style, base_separator_style,
    base_slot_style, caret_style, slot_active_style, slot_disabled_style,
    slot_enter_animation_style, slot_filled_style, slot_invalid_style,
};
use input_rs::leptos::Input;
use leptos::callback::Callback;
use leptos::ev;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

/// Global counter to mint a unique u32 per Otp instance.
static OTP_INSTANCE_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Shared context propagated from [`Otp`] to all [`Slot`] descendants.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct OtpContext {
    /// Whether all slots are disabled.
    pub is_disabled: bool,
    /// Whether all slots show the error/invalid state.
    pub is_invalid: bool,
    /// Visual variant shared across slots.
    pub variant: Variant,
    /// Virtual keyboard hint for each slot.
    pub input_mode: InputMode,
    /// Per-instance prefix used to mint unique slot IDs that don't collide with
    /// other `Otp` components on the same page.
    pub instance_id: u32,
    /// Total slot count.
    pub max_length: usize,
    /// Reactive value signal holding the entire OTP string.
    pub value: RwSignal<String>,
}

/// A composable OTP (One-Time Password) input for Leptos.
///
/// Manages a reactive value and provides [`OtpContext`] via [`provide_context`]
/// so child [`Slot`] components can read shared state.
///
/// # Accessibility
///
/// - Renders as `<div role="group">` with `aria_label`.
/// - Each slot is `<input type="text" maxlength="1">` with `inputmode`.
///
/// # Examples
///
/// ```rust
/// use otprs::leptos::{Otp, Group, Separator, Slot};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn BasicOtp() -> impl IntoView {
///     view! {
///         <Otp max_length=6 aria_label="Enter code">
///             <Group>
///                 <Slot index=0 />
///                 <Slot index=1 />
///                 <Slot index=2 />
///             </Group>
///             <Separator />
///             <Group>
///                 <Slot index=3 />
///                 <Slot index=4 />
///                 <Slot index=5 />
///             </Group>
///         </Otp>
///     }
/// }
/// ```
#[component]
pub fn Otp(
    /// Child [`Group`] and [`Separator`] elements.
    children: Children,

    /// Total number of input slots.
    #[prop(default = 6)]
    max_length: usize,

    /// Controlled value string. Empty for uncontrolled.
    #[prop(default = String::new())]
    value: String,

    /// Called whenever the OTP value changes.
    #[prop(optional)]
    on_change: Option<Callback<String>>,

    /// Called when all slots are filled.
    #[prop(optional)]
    on_complete: Option<Callback<String>>,

    /// Disables all slots.
    #[prop(default = false)]
    is_disabled: bool,

    /// Marks all slots as invalid.
    #[prop(default = false)]
    is_invalid: bool,

    /// Visual variant.
    #[prop(default = Variant::Primary)]
    variant: Variant,

    /// Character filter regex pattern.
    #[prop(default = "[0-9]")]
    pattern: &'static str,

    /// Virtual keyboard type on mobile.
    #[prop(default = InputMode::Numeric)]
    input_mode: InputMode,

    /// `name` attribute for a hidden aggregation input (uses input-rs in otp_mode).
    #[prop(default = "")]
    name: &'static str,

    /// Whether to auto-focus the first slot on mount.
    #[prop(default = false)]
    auto_focus: bool,

    /// Additional CSS class on the outermost `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the outermost `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` on the outermost `<div>`.
    #[prop(default = "")]
    id: &'static str,

    /// ARIA label for the group.
    #[prop(default = "One-time password")]
    aria_label: &'static str,

    /// `aria-describedby` for error description.
    #[prop(default = "")]
    aria_describedby: &'static str,

    /// CSS class on the inner container.
    #[prop(default = "")]
    container_class: &'static str,

    /// Inline CSS on the inner container.
    #[prop(default = "")]
    container_style: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let internal_value = RwSignal::new(value);
    let _ = auto_focus;

    let instance_id = OTP_INSTANCE_COUNTER.fetch_add(1, Ordering::Relaxed);

    provide_context(OtpContext {
        is_disabled,
        is_invalid,
        variant,
        input_mode,
        instance_id,
        max_length,
        value: internal_value,
    });
    provide_context(StoredValue::new(pattern));

    let focus_slot = move |idx: usize| {
        let id = format!("otp-{instance_id}-slot-{idx}");
        focus_by_id(&id);
    };

    let on_slot_change = Callback::new(move |(idx, ch): (usize, String)| {
        let current = internal_value.get_untracked();
        let mut chars: Vec<char> = current
            .chars()
            .chain(std::iter::repeat('\0'))
            .take(max_length)
            .collect();
        if let Some(c) = ch.chars().next() {
            chars[idx] = c;
            let next = (idx + 1).min(max_length - 1);
            focus_slot(next);
        } else {
            chars[idx] = '\0';
            if idx > 0 {
                focus_slot(idx - 1);
            }
        }
        let new_val: String = chars.into_iter().filter(|&c| c != '\0').collect();
        internal_value.set(new_val.clone());
        if let Some(cb) = on_change {
            cb.run(new_val.clone());
        }
        if new_val.len() == max_length
            && let Some(cb) = on_complete
        {
            cb.run(new_val);
        }
    });
    provide_context(on_slot_change);

    let variant_class = variant.to_container_class();
    let full_style = format!("{} {}", base_otp_style(), style);

    let hidden_handle = signal(String::new());
    Effect::new(move |_| {
        hidden_handle.1.set(internal_value.get());
    });
    let hidden_valid = signal(true);

    view! {
        <div
            id=id
            class=format!("{} {}", variant_class, class)
            style=full_style
            role="group"
            aria-label=aria_label
            aria-describedby=aria_describedby
            data-testid=data_testid
            data-invalid=if is_invalid { "true" } else { "false" }
            data-disabled=if is_disabled { "true" } else { "false" }
        >
            <div
                class=format!("otp__container {}", container_class)
                style=format!("{} {}", base_otp_inner_style(), container_style)
            >
                {children()}
            </div>
            <Show when=move || !name.is_empty()>
                <Input
                    r#type="hidden"
                    name=name
                    handle=hidden_handle
                    valid_handle=hidden_valid
                    validate_function=|_| true
                    otp_mode=true
                />
            </Show>
        </div>
    }
}

/// Focus an input element by its DOM id.
fn focus_by_id(id: &str) {
    use wasm_bindgen::JsCast as _;
    if let Some(window) = web_sys::window()
        && let Some(doc) = window.document()
        && let Some(el) = doc.get_element_by_id(id)
        && let Ok(inp) = el.dyn_into::<web_sys::HtmlInputElement>()
    {
        let _ = inp.focus();
    }
}

/// Groups [`Slot`] components visually.
///
/// # Examples
///
/// ```rust
/// use otprs::leptos::{Group, Slot};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyGroup() -> impl IntoView {
///     view! {
///         <Group>
///             <Slot index=0 />
///             <Slot index=1 />
///         </Group>
///     }
/// }
/// ```
#[component]
pub fn Group(
    /// Slot children.
    children: Children,
    /// Additional CSS class.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    view! {
        <div
            id=id
            class=format!("otp__group {}", class)
            style=format!("display: inline-flex; align-items: center; gap: 4px; {}", style)
            role="group"
        >
            {children()}
        </div>
    }
}

/// Decorative separator between [`Group`] components.
#[component]
pub fn Separator(
    /// Additional CSS class.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS.
    #[prop(default = "")]
    style: &'static str,
) -> impl IntoView {
    view! {
        <span
            class=format!("otp__separator {}", class)
            style=format!("{} {}", base_separator_style(), style)
            role="separator"
            aria-hidden="true"
        />
    }
}

/// A single character slot within an [`Otp`] input.
///
/// Must be a descendant of [`Otp`] - reads [`OtpContext`], the slot-change
/// callback, and the pattern from context.
///
/// Uses a unique DOM id per instance (`otp-{instance_id}-slot-{index}`) so that
/// multiple [`Otp`] widgets on the same page don't interfere with each other's focus.
///
/// # Panics
///
/// Panics in debug builds when used outside an [`Otp`] context.
#[component]
pub fn Slot(
    /// Zero-based index of this slot.
    index: usize,
    /// Additional CSS class on the slot wrapper.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS on the slot wrapper.
    #[prop(default = "")]
    style: &'static str,
    /// `id` on the slot wrapper.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let ctx = use_context::<OtpContext>().expect("Slot must be used inside Otp");
    let on_slot_change =
        use_context::<Callback<(usize, String)>>().expect("Slot requires Otp ancestor");
    let pattern_sv = use_context::<StoredValue<&'static str>>()
        .map(|sv| sv.get_value())
        .unwrap_or("[0-9]");

    let is_active = RwSignal::new(false);
    let is_animating = RwSignal::new(false);

    let char_signal = move || {
        ctx.value
            .get()
            .chars()
            .nth(index)
            .filter(|&c| c != '\0')
            .map(|c| c.to_string())
            .unwrap_or_default()
    };

    let handle = signal(String::new());
    let valid_handle = signal(true);

    let slot_style = move || {
        let mut s = base_slot_style().to_string();
        if ctx.is_disabled {
            s.push_str(slot_disabled_style());
        } else if ctx.is_invalid {
            s.push_str(slot_invalid_style());
        } else if is_active.get() {
            s.push_str(slot_active_style());
        } else if !char_signal().is_empty() {
            s.push_str(slot_filled_style());
        }
        if is_animating.get() {
            s.push_str(slot_enter_animation_style());
        }
        s.push_str(style);
        s
    };

    let slot_class = format!("{} {}", ctx.variant.to_slot_class(), class);
    let static_input_style: &'static str = Box::leak(
        format!(
            "position:absolute;inset:0;width:100%;height:100%;opacity:0.01;\
         cursor:{};background:transparent;border:none;outline:none;\
         z-index:1;color:transparent;caret-color:transparent;font-size:1px;",
            if ctx.is_disabled {
                "not-allowed"
            } else {
                "text"
            }
        )
        .into_boxed_str(),
    );
    let slot_index_label: &'static str = Box::leak(format!("Digit {}", index + 1).into_boxed_str());

    let input_id: &'static str =
        Box::leak(format!("otp-{}-slot-{}", ctx.instance_id, index).into_boxed_str());
    let always_valid: fn(String) -> bool = |_| true;

    let on_input_cb: Callback<web_sys::Event> = Callback::new(move |e: web_sys::Event| {
        let raw: String = e
            .target()
            .and_then(|t| {
                use wasm_bindgen::JsCast as _;
                t.dyn_into::<web_sys::HtmlInputElement>().ok().map(|el| {
                    let v = el.value();
                    el.set_value("");
                    v
                })
            })
            .unwrap_or_default();
        let ch = raw
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_default();
        on_slot_change.run((index, ch.clone()));
        if !ch.is_empty() {
            is_animating.set(true);
            spawn_local(async move {
                gloo_timers::future::sleep(Duration::from_millis(120)).await;
                is_animating.set(false);
            });
        }
        handle.1.set(String::new());
    });

    view! {
        <span
            id=id
            class=slot_class.clone()
            style=slot_style
            data-slot-index=index.to_string()
            data-active=move || if is_active.get() { "true" } else { "false" }
            data-filled=move || if !char_signal().is_empty() { "true" } else { "false" }
            data-disabled=if ctx.is_disabled { "true" } else { "false" }
            data-invalid=if ctx.is_invalid { "true" } else { "false" }
            on:keydown=move |e: web_sys::KeyboardEvent| {
                let key = e.key();
                if key == "ArrowLeft" || key == "Backspace" {
                    e.prevent_default();
                }
                if key == "Backspace" {
                    if let Some(target) = e.target() {
                        use wasm_bindgen::JsCast as _;
                        if let Ok(el) = target.dyn_into::<web_sys::HtmlInputElement>() {
                            el.set_value("");
                        }
                    }
                    on_slot_change.run((index, String::new()));
                    handle.1.set(String::new());
                }
            }
        >
            <Input
                r#type="text"
                id=input_id
                handle=handle
                valid_handle=valid_handle
                validate_function=always_valid
                input_style=static_input_style
                otp_mode=true
                maxlength=Some(1usize)
                pattern=pattern_sv
                inputmode="numeric"
                autocomplete="one-time-code"
                aria_label=slot_index_label
                aria_invalid=if ctx.is_invalid { "true" } else { "false" }
                disabled=ctx.is_disabled
                on_input=on_input_cb
                on_focus=Callback::new(move |_: ev::FocusEvent| is_active.set(true))
                on_blur=Callback::new(move |_: ev::FocusEvent| is_active.set(false))
            />
            <Show when=move || !char_signal().is_empty()>
                <span
                    class="otp__slot-value"
                    aria-hidden="true"
                    style="pointer-events:none;user-select:none;position:absolute;\
                           inset:0;display:flex;align-items:center;justify-content:center;\
                           font-size:22px;font-weight:700;"
                >
                    {move || char_signal()}
                </span>
            </Show>
            <Show when=move || char_signal().is_empty() && is_active.get()>
                <span
                    class="otp__caret"
                    aria-hidden="true"
                    style=caret_style()
                />
            </Show>
        </span>
    }
}
