// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    InputMode, Variant, base_otp_inner_style, base_otp_style, base_separator_style,
    base_slot_style, caret_style, slot_active_style, slot_disabled_style,
    slot_enter_animation_style, slot_filled_style, slot_invalid_style,
};
use input_rs::yew::Input;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Context propagated from [`Otp`] to all [`Slot`] children.
///
/// Carries the current value string, disabled/invalid state, and the shared
/// callback that slots call when their single character changes.
#[derive(Clone, Debug, PartialEq)]
pub struct OtpContext {
    /// The full OTP value string. Each character corresponds to one slot by index.
    pub value: String,

    /// Maximum number of slots (equals `max_length` on [`Otp`]).
    pub max_length: usize,

    /// Whether all slots are disabled.
    pub is_disabled: bool,

    /// Whether all slots should render in an invalid/error state.
    pub is_invalid: bool,

    /// Visual variant shared across all slots.
    pub variant: Variant,

    /// Virtual keyboard type hint applied to each slot's `<input>`.
    pub input_mode: InputMode,

    /// Character filter pattern forwarded to the `pattern` attribute of each slot.
    pub pattern: &'static str,

    /// Callback invoked by a slot when its character changes. Receives `(index, new_char_or_empty)`.
    pub on_slot_change: Callback<(usize, String)>,

    /// Callback invoked by a slot on key-down to handle backspace and arrow keys.
    pub on_slot_keydown: Callback<(usize, String)>,
}

/// Props for the [`Otp`] root component.
#[derive(Properties, PartialEq, Clone)]
pub struct OtpProps {
    /// Child [`Group`] and [`Separator`] elements.
    #[prop_or_default]
    pub children: Children,

    /// Maximum number of input slots (required).
    #[prop_or(6)]
    pub max_length: usize,

    /// Controlled value. When provided the component is controlled;
    /// omit or pass an empty string for uncontrolled behaviour.
    #[prop_or_default]
    pub value: String,

    /// Callback fired whenever the OTP value changes.
    #[prop_or_default]
    pub on_change: Callback<String>,

    /// Callback fired when all slots are filled.
    #[prop_or_default]
    pub on_complete: Callback<String>,

    /// Disables all slots.
    #[prop_or_default]
    pub is_disabled: bool,

    /// Renders all slots in the invalid/error state.
    #[prop_or_default]
    pub is_invalid: bool,

    /// Visual variant.
    #[prop_or_default]
    pub variant: Variant,

    /// Character filter regex pattern applied to each slot input.
    #[prop_or("[0-9]")]
    pub pattern: &'static str,

    /// Virtual keyboard type on mobile.
    #[prop_or_default]
    pub input_mode: InputMode,

    /// `name` attribute forwarded to the hidden aggregation input.
    #[prop_or_default]
    pub name: &'static str,

    /// Whether to auto-focus the first slot on mount.
    #[prop_or_default]
    pub auto_focus: bool,

    /// Additional CSS class on the outermost `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the outermost `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` on the outermost `<div>`.
    #[prop_or_default]
    pub id: &'static str,

    /// ARIA label for the OTP group.
    #[prop_or("One-time password")]
    pub aria_label: &'static str,

    /// `aria-describedby` pointing to an error description element.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    /// CSS class on the inner container `<div>`.
    #[prop_or_default]
    pub container_class: &'static str,

    /// Inline CSS on the inner container `<div>`.
    #[prop_or_default]
    pub container_style: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A composable OTP (One-Time Password) input that manages focus across slots.
///
/// Renders a row of individual character slots. Each slot accepts exactly one
/// character and automatically advances focus to the next slot after entry.
/// Pressing `Backspace` on an empty slot moves focus to the previous slot.
///
/// Provides an [`OtpContext`] so descendant [`Slot`] components can read
/// the shared state without prop-drilling.
///
/// # Accessibility
///
/// - Renders as `<div role="group">` with a required `aria_label`.
/// - Each slot renders as `<input type="text">` with `inputmode`, `pattern`,
///   and `aria-label` attributes for screen-reader compatibility.
/// - Focus is managed programmatically via `NodeRef` array and `focus()` calls.
///
/// # Examples
///
/// ```rust
/// use otprs::yew::{Otp, Group, Separator, Slot};
/// use yew::prelude::*;
///
/// #[function_component(BasicOtp)]
/// pub fn basic_otp() -> Html {
///     html! {
///         <Otp max_length={6} aria_label="Enter verification code">
///             <Group>
///                 <Slot index={0} />
///                 <Slot index={1} />
///                 <Slot index={2} />
///             </Group>
///             <Separator />
///             <Group>
///                 <Slot index={3} />
///                 <Slot index={4} />
///                 <Slot index={5} />
///             </Group>
///         </Otp>
///     }
/// }
/// ```
#[function_component(Otp)]
pub fn otp(props: &OtpProps) -> Html {
    let internal_value = use_state(|| props.value.clone());
    let slot_refs = use_memo(props.max_length, |_| {
        (0..props.max_length)
            .map(|_| NodeRef::default())
            .collect::<Vec<_>>()
    });

    {
        let internal_value = internal_value.clone();
        let new_val = props.value.clone();
        use_effect_with(props.value.clone(), move |_| {
            if !new_val.is_empty() {
                internal_value.set(new_val);
            }
            || ()
        });
    }

    {
        let slot_refs = slot_refs.clone();
        let auto_focus = props.auto_focus;
        use_effect_with(auto_focus, move |&af| {
            if af
                && let Some(node) = slot_refs.first()
                && let Some(input) = node.cast::<HtmlInputElement>()
            {
                let _ = input.focus();
            }
            || ()
        });
    }

    let max_length = props.max_length;
    let on_change_cb = props.on_change.clone();
    let on_complete_cb = props.on_complete.clone();

    let on_slot_change = {
        let internal_value = internal_value.clone();
        let slot_refs = slot_refs.clone();
        Callback::from(move |(idx, ch): (usize, String)| {
            let mut chars: Vec<char> = (*internal_value)
                .chars()
                .chain(std::iter::repeat('\0'))
                .take(max_length)
                .collect();
            if let Some(c) = ch.chars().next() {
                chars[idx] = c;
            } else {
                chars[idx] = '\0';
            }
            let new_val: String = chars.iter().filter(|&&c| c != '\0').collect::<String>();
            internal_value.set(new_val.clone());
            on_change_cb.emit(new_val.clone());
            if new_val.len() == max_length {
                on_complete_cb.emit(new_val.clone());
            }
            if !ch.is_empty() {
                let next = (idx + 1).min(max_length - 1);
                if let Some(node) = slot_refs.get(next)
                    && let Some(input) = node.cast::<HtmlInputElement>()
                {
                    let _ = input.focus();
                }
            }
        })
    };

    let on_slot_keydown = {
        let slot_refs = slot_refs.clone();
        Callback::from(move |(idx, key): (usize, String)| {
            if key == "Backspace" {
                if idx > 0
                    && let Some(node) = slot_refs.get(idx - 1)
                    && let Some(input) = node.cast::<HtmlInputElement>()
                {
                    let _ = input.focus();
                }
            } else if key == "ArrowLeft" && idx > 0 {
                if let Some(node) = slot_refs.get(idx - 1)
                    && let Some(input) = node.cast::<HtmlInputElement>()
                {
                    let _ = input.focus();
                }
            } else if key == "ArrowRight"
                && idx + 1 < max_length
                && let Some(node) = slot_refs.get(idx + 1)
                && let Some(input) = node.cast::<HtmlInputElement>()
            {
                let _ = input.focus();
            }
        })
    };

    let ctx = OtpContext {
        value: (*internal_value).clone(),
        max_length: props.max_length,
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        variant: props.variant,
        input_mode: props.input_mode,
        pattern: props.pattern,
        on_slot_change,
        on_slot_keydown,
    };

    let variant_class = props.variant.to_container_class();
    let full_style = format!("{} {}", base_otp_style(), props.style);

    let hidden_handle = use_state(|| (*internal_value).clone());
    let hidden_valid = use_state(|| true);
    {
        let hidden_handle = hidden_handle.clone();
        let val = (*internal_value).clone();
        use_effect_with(val, move |v| {
            hidden_handle.set(v.clone());
            || ()
        });
    }
    let always_valid_fn = Callback::from(|_: String| true);
    let hidden_ref = use_node_ref();

    html! {
        <>
            <ContextProvider<OtpContext> context={ctx}>
                <ContextProvider<Vec<NodeRef>> context={(*slot_refs).clone()}>
                    <div
                        id={props.id}
                        class={format!("{} {}", variant_class, props.class)}
                        style={full_style}
                        role="group"
                        aria-label={props.aria_label}
                        aria-describedby={props.aria_describedby}
                        data-testid={props.data_testid}
                        data-invalid={if props.is_invalid { "true" } else { "false" }}
                        data-disabled={if props.is_disabled { "true" } else { "false" }}
                    >
                        <div
                            class={format!("otp__container {}", props.container_class)}
                            style={format!("{} {}", base_otp_inner_style(), props.container_style)}
                        >
                            { for props.children.iter() }
                        </div>
                        if !props.name.is_empty() {
                            <Input
                                r#type="hidden"
                                r#ref={hidden_ref.clone()}
                                name={props.name}
                                handle={hidden_handle}
                                valid_handle={hidden_valid}
                                validate_function={always_valid_fn.clone()}
                                otp_mode=true
                            />
                        }
                    </div>
                </ContextProvider<Vec<NodeRef>>>
            </ContextProvider<OtpContext>>
        </>
    }
}

/// Props for [`Group`].
#[derive(Properties, PartialEq, Clone)]
pub struct GroupProps {
    /// [`Slot`] children within this group.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class on the group `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the group `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` on the group `<div>`.
    #[prop_or_default]
    pub id: &'static str,
}

/// Groups a set of [`Slot`] components with a shared container.
///
/// Visually groups adjacent slots. Place [`Separator`] between groups.
///
/// # Examples
///
/// ```rust
/// use otprs::yew::{Group, Slot};
/// use yew::prelude::*;
///
/// #[function_component(MyGroup)]
/// pub fn my_group() -> Html {
///     html! {
///         <Group>
///             <Slot index={0} />
///             <Slot index={1} />
///             <Slot index={2} />
///         </Group>
///     }
/// }
/// ```
#[function_component(Group)]
pub fn otp_group(props: &GroupProps) -> Html {
    html! {
        <div
            id={props.id}
            class={format!("otp__group {}", props.class)}
            style={format!("display: inline-flex; align-items: center; gap: 8px; {}", props.style)}
            role="group"
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for [`Separator`].
#[derive(Properties, PartialEq, Clone)]
pub struct SeparatorProps {
    /// Additional CSS class on the separator element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the separator element.
    #[prop_or_default]
    pub style: &'static str,
}

/// A decorative separator rendered between [`Group`] components.
///
/// Renders as a short horizontal rule with `role="separator"` and
/// `aria-hidden="true"` so screen readers skip it.
#[function_component(Separator)]
pub fn otp_separator(props: &SeparatorProps) -> Html {
    html! {
        <span
            class={format!("otp__separator {}", props.class)}
            style={format!("{} {}", base_separator_style(), props.style)}
            role="separator"
            aria-hidden="true"
        />
    }
}

/// Props for [`Slot`].
#[derive(Properties, PartialEq, Clone)]
pub struct SlotProps {
    /// Zero-based index of this slot within the parent [`Otp`].
    pub index: usize,

    /// Additional CSS class on the slot wrapper `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the slot wrapper `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` on the slot wrapper `<span>`.
    #[prop_or_default]
    pub id: &'static str,
}

/// A single character slot within an [`Otp`] input.
///
/// Must be a descendant of [`Otp`] - reads [`OtpContext`] from context.
/// Internally delegates to [`input_rs::yew::Input`] with `otp_mode = true`, which
/// renders only the bare `<input>` element. Focus, animation, and data-attribute
/// states are tracked by the slot wrapper.
///
/// # Panics
///
/// Panics in debug mode when used outside an [`Otp`] context.
#[function_component(Slot)]
pub fn otp_slot(props: &SlotProps) -> Html {
    let ctx = use_context::<OtpContext>().expect("Slot must be used inside Otp");
    let slot_refs = use_context::<Vec<NodeRef>>().unwrap_or_default();

    let index = props.index;
    let is_active = use_state(|| false);
    let is_animating = use_state(|| false);
    let input_ref = slot_refs.get(index).cloned().unwrap_or_default();

    let char_at_index: String = ctx
        .value
        .chars()
        .nth(index)
        .filter(|&c| c != ' ')
        .map(|c| c.to_string())
        .unwrap_or_default();

    let is_filled = !char_at_index.is_empty();
    let is_slot_active = *is_active;

    let slot_handle = use_state(|| char_at_index.clone());
    let slot_valid = use_state(|| true);

    let on_focus_cb = {
        let is_active = is_active.clone();
        Callback::from(move |_: FocusEvent| {
            is_active.set(true);
        })
    };

    let on_blur_cb = {
        let is_active = is_active.clone();
        Callback::from(move |_: FocusEvent| {
            is_active.set(false);
        })
    };

    let on_keydown = {
        let on_slot_keydown = ctx.on_slot_keydown.clone();
        let on_slot_change = ctx.on_slot_change.clone();
        let slot_handle = slot_handle.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            if key == "ArrowLeft" || key == "Backspace" {
                e.prevent_default();
            }
            if key == "Backspace" {
                if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                on_slot_change.emit((index, String::new()));
                slot_handle.set(String::new());
            }
            on_slot_keydown.emit((index, key));
        })
    };

    let validate_fn: Callback<String, bool> = {
        let on_slot_change = ctx.on_slot_change.clone();
        let is_animating = is_animating.clone();
        let slot_handle = slot_handle.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |v: String| {
            let ch = v.chars().last().map(|c| c.to_string()).unwrap_or_default();
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input.set_value("");
            }
            if !ch.is_empty() {
                is_animating.set(true);
            }
            on_slot_change.emit((index, ch.clone()));
            slot_handle.set(String::new());
            true
        })
    };

    let mut slot_style = base_slot_style().to_string();
    if ctx.is_disabled {
        slot_style.push_str(slot_disabled_style());
    } else if ctx.is_invalid {
        slot_style.push_str(slot_invalid_style());
    } else if is_slot_active {
        slot_style.push_str(slot_active_style());
    } else if is_filled {
        slot_style.push_str(slot_filled_style());
    }
    if *is_animating {
        slot_style.push_str(slot_enter_animation_style());
    }
    slot_style.push_str(props.style);

    let static_input_style: &'static str = Box::leak(format!(
        "position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; cursor: {}; background: transparent; border: none; outline: none;",
        if ctx.is_disabled { "not-allowed" } else { "text" }
    ).into_boxed_str());
    let slot_index_label: &'static str = Box::leak(format!("Digit {}", index + 1).into_boxed_str());

    let slot_class = format!("{} {}", ctx.variant.to_slot_class(), props.class);

    html! {
        <span
            id={props.id}
            class={slot_class}
            style={slot_style}
            data-active={if is_slot_active { "true" } else { "false" }}
            data-filled={if is_filled { "true" } else { "false" }}
            data-disabled={if ctx.is_disabled { "true" } else { "false" }}
            data-invalid={if ctx.is_invalid { "true" } else { "false" }}
            onkeydown={on_keydown}
        >
            <Input
                r#type="text"
                r#ref={input_ref}
                handle={slot_handle.clone()}
                valid_handle={slot_valid}
                validate_function={validate_fn}
                input_style={static_input_style}
                otp_mode=true
                maxlength={Some(1usize)}
                pattern={ctx.pattern}
                autocomplete="one-time-code"
                aria_label={slot_index_label}
                aria_invalid={if ctx.is_invalid { "true" } else { "false" }}
                disabled={ctx.is_disabled}
                on_focus={on_focus_cb}
                on_blur={on_blur_cb}
            />
            if is_filled {
                <span
                    class="otp__slot-value"
                    aria-hidden="true"
                    style="pointer-events: none; user-select: none;"
                >
                    { char_at_index.chars().last().map(|c| c.to_string()).unwrap_or_default() }
                </span>
            } else if is_slot_active {
                <span class="otp__caret" aria-hidden="true" style={caret_style()} />
            }
        </span>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
