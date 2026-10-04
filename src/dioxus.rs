// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    InputMode, Variant, base_otp_inner_style, base_otp_style, base_separator_style,
    base_slot_style, caret_style, slot_active_style, slot_disabled_style,
    slot_enter_animation_style, slot_filled_style, slot_invalid_style,
};
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Global counter to mint a unique u32 per Otp instance.
static OTP_INSTANCE_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Shared context propagated from [`Otp`] to all [`Slot`] descendants.
///
/// Holds a live **reactive** `Signal<String>` for the full OTP value so that
/// every `Slot` automatically re-renders whenever the value changes.
#[derive(Clone, Debug)]
pub struct OtpContext {
    /// Reactive signal holding the full OTP value - one char per slot index.
    pub value: Signal<String>,
    /// Total slot count (equals `max_length` on [`Otp`]).
    pub max_length: usize,
    /// Whether all slots are disabled.
    pub is_disabled: bool,
    /// Whether all slots show the error/invalid state.
    pub is_invalid: bool,
    /// Visual variant shared across slots.
    pub variant: Variant,
    /// Virtual keyboard hint for each slot.
    pub input_mode: InputMode,
    /// Character filter pattern for each slot's `<input>`.
    pub pattern: &'static str,
    /// Per-instance prefix so multiple Otp widgets on one page don't share
    /// slot DOM IDs and steal each other's focus.
    pub instance_id: u32,
}

/// Callback type provided as context: `(slot_index, char_or_empty)`.
///
/// When `char` is non-empty a character was typed; when empty the slot was cleared.
#[derive(Clone)]
pub struct SlotChangeFn(pub std::rc::Rc<dyn Fn(usize, String)>);

/// Props for the [`Otp`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct OtpProps {
    /// Child [`Group`] and [`Separator`] elements.
    #[props(default)]
    pub children: Element,

    /// Total number of input slots.
    #[props(default = 6)]
    pub max_length: usize,

    /// Controlled value string. Empty string for uncontrolled.
    #[props(default)]
    pub value: String,

    /// Called whenever the OTP value changes.
    #[props(default)]
    pub on_change: Option<Callback<String>>,

    /// Called when all slots are filled.
    #[props(default)]
    pub on_complete: Option<Callback<String>>,

    /// Disables all slots.
    #[props(default)]
    pub is_disabled: bool,

    /// Marks all slots as invalid.
    #[props(default)]
    pub is_invalid: bool,

    /// Visual variant.
    #[props(default)]
    pub variant: Variant,

    /// Character filter regex pattern.
    #[props(default = "[0-9]")]
    pub pattern: &'static str,

    /// Virtual keyboard type on mobile.
    #[props(default)]
    pub input_mode: InputMode,

    /// `name` attribute for a hidden aggregation input (uses input-rs in otp_mode).
    #[props(default)]
    pub name: &'static str,

    /// Whether to auto-focus the first slot on mount.
    #[props(default)]
    pub auto_focus: bool,

    /// Additional CSS class on the outermost `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the outermost `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` on the outermost `<div>`.
    #[props(default)]
    pub id: &'static str,

    /// ARIA label for the group.
    #[props(default = "One-time password")]
    pub aria_label: &'static str,

    /// `aria-describedby` for error description.
    #[props(default)]
    pub aria_describedby: &'static str,

    /// CSS class on the inner container.
    #[props(default)]
    pub container_class: &'static str,

    /// Inline CSS on the inner container.
    #[props(default)]
    pub container_style: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A composable OTP (One-Time Password) input for Dioxus.
///
/// Manages focus traversal across slots. Provides [`OtpContext`] and
/// [`SlotChangeFn`] via [`use_context_provider`] so child [`Slot`]
/// components can read reactive state and notify the parent of changes.
///
/// # Accessibility
///
/// - Renders as `<div role="group">` with `aria_label`.
/// - Each slot is an `<input type="text" maxlength="1">` with `inputmode`.
///
/// # Examples
///
/// ```rust
/// use otprs::dioxus::{Otp, Group, Separator, Slot};
/// use dioxus::prelude::*;
///
/// #[component]
/// fn BasicOtp() -> Element {
///     rsx! {
///         Otp { max_length: 6, aria_label: "Enter verification code",
///             Group {
///                 Slot { index: 0 }
///                 Slot { index: 1 }
///                 Slot { index: 2 }
///             }
///             Separator {}
///             Group {
///                 Slot { index: 3 }
///                 Slot { index: 4 }
///                 Slot { index: 5 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Otp(props: OtpProps) -> Element {
    use input_rs::dioxus::Input;

    let max_length = props.max_length;
    let internal_value: Signal<String> = use_signal(|| props.value.clone());

    let instance_id = OTP_INSTANCE_COUNTER.fetch_add(1, Ordering::Relaxed);

    use_context_provider(|| OtpContext {
        value: internal_value,
        max_length,
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        variant: props.variant,
        input_mode: props.input_mode,
        pattern: props.pattern,
        instance_id,
    });

    let on_change = props.on_change;
    let on_complete = props.on_complete;

    use_context_provider(|| {
        SlotChangeFn(std::rc::Rc::new(move |idx: usize, ch: String| {
            let mut iv = internal_value;
            let mut chars: Vec<char> = iv()
                .chars()
                .chain(std::iter::repeat('\0'))
                .take(max_length)
                .collect();
            if let Some(c) = ch.chars().next() {
                chars[idx] = c;
                let next = (idx + 1).min(max_length - 1);
                focus_slot(instance_id, next);
            } else {
                chars[idx] = '\0';
                if idx > 0 {
                    focus_slot(instance_id, idx - 1);
                }
            }
            let new_val: String = chars.into_iter().filter(|&c| c != '\0').collect();
            iv.set(new_val.clone());
            if let Some(ref cb) = on_change {
                cb.call(new_val.clone());
            }
            if new_val.len() == max_length
                && let Some(ref cb) = on_complete
            {
                cb.call(new_val);
            }
        }))
    });

    let variant_class = props.variant.to_container_class();

    #[allow(clippy::redundant_closure)]
    let hidden_handle = use_signal(move || internal_value());
    // TODO: Report Clippy error
    // let hidden_handle = use_signal(*internal_value);
    let hidden_valid = use_signal(|| true);

    rsx! {
        div {
            id: props.id,
            class: "{variant_class} {props.class}",
            style: "{base_otp_style()} {props.style}",
            role: "group",
            aria_label: props.aria_label,
            aria_describedby: props.aria_describedby,
            "data-testid": props.data_testid,
            "data-invalid": if props.is_invalid { "true" } else { "false" },
            "data-disabled": if props.is_disabled { "true" } else { "false" },
            div {
                class: "otp__container {props.container_class}",
                style: "{base_otp_inner_style()} {props.container_style}",
                {props.children}
            }
            if !props.name.is_empty() {
                Input {
                    r#type: "hidden",
                    name: props.name,
                    handle: hidden_handle,
                    valid_handle: hidden_valid,
                    validate_function: always_valid_fn,
                    otp_mode: true,
                }
            }
        }
    }
}

/// Focus a slot's input element using its instance-scoped id `otp-{instance_id}-slot-{idx}`.
fn focus_slot(instance_id: u32, idx: usize) {
    use wasm_bindgen::JsCast as _;
    let id = format!("otp-{instance_id}-slot-{idx}");
    if let Some(window) = web_sys::window()
        && let Some(doc) = window.document()
        && let Some(el) = doc.get_element_by_id(&id)
        && let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>()
    {
        let _ = input.focus();
    }
}

/// Props for [`Group`].
#[derive(Props, Clone, PartialEq)]
pub struct GroupProps {
    /// Slot children.
    #[props(default)]
    pub children: Element,
    /// Additional CSS class.
    #[props(default)]
    pub class: &'static str,
    /// Inline CSS.
    #[props(default)]
    pub style: &'static str,
    /// `id` attribute.
    #[props(default)]
    pub id: &'static str,
}

/// Groups [`Slot`] components visually.
#[component]
pub fn Group(props: GroupProps) -> Element {
    rsx! {
        div {
            id: props.id,
            class: "otp__group {props.class}",
            style: "display: inline-flex; align-items: center; gap: 8px; {props.style}",
            role: "group",
            {props.children}
        }
    }
}

/// Props for [`Separator`].
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Additional CSS class.
    #[props(default)]
    pub class: &'static str,
    /// Inline CSS.
    #[props(default)]
    pub style: &'static str,
}

/// Decorative separator between [`Group`] components.
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    rsx! {
        span {
            class: "otp__separator {props.class}",
            style: "{base_separator_style()} {props.style}",
            role: "separator",
            aria_hidden: "true",
        }
    }
}

/// Props for [`Slot`].
#[derive(Props, Clone, PartialEq)]
pub struct SlotProps {
    /// Zero-based index of this slot within the [`Otp`] parent.
    pub index: usize,
    /// Additional CSS class on the slot wrapper.
    #[props(default)]
    pub class: &'static str,
    /// Inline CSS on the slot wrapper.
    #[props(default)]
    pub style: &'static str,
    /// `id` on the slot wrapper `<span>`.
    #[props(default)]
    pub id: &'static str,
}

/// A single character slot within an [`Otp`] input.
///
/// Reads [`OtpContext`] (which holds a reactive `Signal<String>`) from context
/// and subscribes to it - the component re-renders whenever any slot value changes.
///
/// Uses [`input_rs::dioxus::Input`] in `otp_mode` with the `on_input` hook.
/// After each keystroke `slot_handle` is reset to `""` so input-rs clears the
/// DOM `<input>`, guaranteeing repeated characters always fire a fresh event.
///
/// Focus advance and retreat are driven by direct `web_sys::HtmlInputElement::focus()`
/// calls in the parent's [`SlotChangeFn`] context (no Signal indirection needed).
#[component]
pub fn Slot(props: SlotProps) -> Element {
    use input_rs::dioxus::Input;

    let ctx = use_context::<OtpContext>();
    let slot_change_fn = use_context::<SlotChangeFn>();
    let slot_change_fn_bk = slot_change_fn.clone();

    let index = props.index;
    let mut is_active = use_signal(|| false);
    let mut is_animating = use_signal(|| false);
    let mut slot_handle = use_signal(String::new);
    let slot_valid = use_signal(|| true);

    let char_at_index: String = ctx
        .value
        .read()
        .chars()
        .nth(index)
        .filter(|&c| c != '\0')
        .map(|c| c.to_string())
        .unwrap_or_default();

    let is_filled = !char_at_index.is_empty();
    let is_disabled = ctx.is_disabled;
    let is_invalid = ctx.is_invalid;

    let input_id: &'static str =
        Box::leak(format!("otp-{}-slot-{}", ctx.instance_id, index).into_boxed_str());

    let slot_variant_class = ctx.variant.to_slot_class();
    let slot_class = format!("{slot_variant_class} {}", props.class);

    let mut slot_style = base_slot_style().to_string();
    if is_disabled {
        slot_style.push_str(slot_disabled_style());
    } else if is_invalid {
        slot_style.push_str(slot_invalid_style());
    } else if is_active() {
        slot_style.push_str(slot_active_style());
    } else if is_filled {
        slot_style.push_str(slot_filled_style());
    }
    if is_animating() {
        slot_style.push_str(slot_enter_animation_style());
    }
    slot_style.push_str(props.style);

    let cursor = if is_disabled { "not-allowed" } else { "text" };
    let input_style = format!(
        "position:absolute;inset:0;width:100%;height:100%;opacity:0.01;\
         cursor:{cursor};background:transparent;border:none;outline:none;\
         z-index:1;color:transparent;caret-color:transparent;font-size:1px;"
    );
    let slot_label: &'static str = Box::leak(format!("Digit {}", index + 1).into_boxed_str());
    let input_style_static: &'static str = Box::leak(input_style.into_boxed_str());
    let slot_class_static: &'static str = Box::leak(slot_class.into_boxed_str());
    let slot_style_static: &'static str = Box::leak(slot_style.into_boxed_str());
    let slot_index_str: &'static str = Box::leak(index.to_string().into_boxed_str());
    let aria_invalid_static: &'static str = if is_invalid { "true" } else { "false" };

    let on_input_handler = EventHandler::new(move |e: Event<FormData>| {
        let raw = e.value();
        let ch = raw
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_default();
        if !ch.is_empty() {
            is_animating.set(true);
            spawn(async move {
                gloo_timeout_120ms().await;
                is_animating.set(false);
            });
        }
        slot_change_fn.0(index, ch);
        slot_handle.set(String::new());
    });

    let on_keydown = move |e: KeyboardEvent| {
        let key = e.key().to_string();
        if key == "ArrowLeft" || key == "Backspace" {
            e.prevent_default();
        }
        if key == "Backspace" {
            slot_change_fn_bk.0(index, String::new());
            slot_handle.set(String::new());
        }
    };

    rsx! {
        span {
            id: props.id,
            class: slot_class_static,
            style: slot_style_static,
            "data-slot-index": slot_index_str,
            "data-active": if is_active() { "true" } else { "false" },
            "data-filled": if is_filled { "true" } else { "false" },
            "data-disabled": if is_disabled { "true" } else { "false" },
            onkeydown: on_keydown,

            Input {
                r#type: "text",
                id: input_id,
                handle: slot_handle,
                valid_handle: slot_valid,
                validate_function: always_valid_fn,
                input_style: input_style_static,
                otp_mode: true,
                maxlength: Some(1usize),
                pattern: ctx.pattern,
                inputmode: "numeric",
                autocomplete: "one-time-code",
                aria_label: slot_label,
                disabled: is_disabled,
                aria_invalid: aria_invalid_static,
                on_input: Some(on_input_handler),
                on_focus: Some(EventHandler::new(move |_| is_active.set(true))),
                on_blur: Some(EventHandler::new(move |_| is_active.set(false))),
            }

            if is_filled {
                span {
                    class: "otp__slot-value",
                    aria_hidden: "true",
                    style: "pointer-events:none;user-select:none;position:absolute;\
                            inset:0;display:flex;align-items:center;justify-content:center;\
                            font-size:22px;font-weight:700;",
                    "{char_at_index}"
                }
            } else if is_active() {
                span {
                    class: "otp__caret",
                    aria_hidden: "true",
                    style: caret_style(),
                }
            }
        }
    }
}

async fn gloo_timeout_120ms() {
    std::future::ready(()).await
}

fn always_valid_fn(_: String) -> bool {
    true
}
