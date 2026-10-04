// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use dioxus::prelude::*;
use otprs::common::{generate_totp, validate_hotp, validate_totp};
use otprs::dioxus::{Otp, Group, Separator, Slot};
use otprs::{Variant, unix_now_secs};

const TOTP_SECRET: &[u8; 20] = b"12345678901234567890";

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        LandingPage {}
    }
}

#[component]
fn Card(title: String, code: String, children: Element) -> Element {
    rsx! {
        article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
            h3 { class: "text-xl font-bold mb-2", "{title}" }
            pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
                "{code}"
            }
            div { style: "width:100%;display:flex;flex-direction:column;align-items:center;gap:8px;",
                {children}
            }
        }
    }
}

#[component]
fn ExampleBasic() -> Element {
    rsx! {
        Otp { max_length: 6, aria_label: "Basic 6-digit OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}

#[component]
fn ExampleControlled() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;",
            Otp {
                max_length: 6, value: value(), aria_label: "Controlled OTP",
                on_change: Some(Callback::new(move |v: String| value.set(v))),
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
            p { style: "font-size:12px;color:#6b7280;margin:0;", "value: {value():?}" }
        }
    }
}

#[component]
fn ExampleOnComplete() -> Element {
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;",
            Otp {
                max_length: 4, aria_label: "4-digit PIN with completion",
                on_complete: Some(Callback::new(move |_: String| done.set(true))),
                Group {
                    Slot { index: 0 } Slot { index: 1 }
                    Slot { index: 2 } Slot { index: 3 }
                }
            }
            if done() {
                p { style: "color:#22c55e;font-size:12px;margin:0;", "✓ PIN complete!" }
            } else {
                p { style: "color:#6b7280;font-size:12px;margin:0;", "Enter all 4 digits" }
            }
        }
    }
}

#[component]
fn ExampleSecondary() -> Element {
    rsx! {
        Otp { max_length: 6, variant: Variant::Secondary, aria_label: "Secondary OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}

#[component]
fn ExampleDisabled() -> Element {
    rsx! {
        Otp {
            max_length: 6, is_disabled: true,
            value: "123456".to_string(), aria_label: "Disabled OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}

#[component]
fn ExampleInvalid() -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:8px;align-items:center;width:100%;",
            Otp {
                max_length: 6, is_invalid: true,
                aria_label: "Invalid OTP", aria_describedby: "dio-otp-err",
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
            p { id: "dio-otp-err", role: "alert",
                style: "color:#ef4444;font-size:12px;margin:0;",
                "Incorrect code. Try again."
            }
        }
    }
}

#[component]
fn ExamplePin() -> Element {
    rsx! {
        Otp { max_length: 4, aria_label: "4-digit PIN",
            Group {
                Slot { index: 0 } Slot { index: 1 }
                Slot { index: 2 } Slot { index: 3 }
            }
        }
    }
}

#[component]
fn ExampleCustomStyle() -> Element {
    const S: &str = "width:52px;height:60px;border-radius:12px;border-color:#7c3aed;";
    rsx! {
        Otp { max_length: 6, aria_label: "Custom styled OTP",
            Group { style: "gap:8px;",
                Slot { index: 0, style: S }
                Slot { index: 1, style: S }
                Slot { index: 2, style: S }
            }
            Separator { style: "width:20px;height:3px;background:#7c3aed;" }
            Group { style: "gap:8px;",
                Slot { index: 3, style: S }
                Slot { index: 4, style: S }
                Slot { index: 5, style: S }
            }
        }
    }
}

#[component]
fn ExampleHotp() -> Element {
    let mut result = use_signal(|| Option::<bool>::None);
    let on_complete = Callback::new(move |code: String| {
        let ok = validate_hotp(b"JBSWY3DPEHPK3PXP", 0, &code, 6).is_ok();
        result.set(Some(ok));
    });
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;",
            Otp {
                max_length: 6, on_complete: Some(on_complete), aria_label: "HOTP validation",
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
            match result() {
                Some(true)  => rsx! { p { style:"color:#22c55e;font-size:12px;margin:0;", "✓ Valid HOTP" } },
                Some(false) => rsx! { p { style:"color:#ef4444;font-size:12px;margin:0;", "✗ Invalid code" } },
                None        => rsx! { p { style:"color:#6b7280;font-size:12px;margin:0;", "Fill 6 digits to validate" } },
            }
        }
    }
}

#[component]
fn ExampleTotp() -> Element {
    let mut result = use_signal(|| Option::<bool>::None);
    let mut token_display = use_signal(|| generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));

    let on_complete = Callback::new(move |code: String| {
        let now = unix_now_secs();
        let ok = validate_totp(TOTP_SECRET, now, &code, 6, 30).is_ok();
        result.set(Some(ok));
    });
    let refresh = move |_| {
        token_display.set(generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));
        result.set(None);
    };
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;",
            p { style: "font-size:11px;color:#6b7280;margin:0;text-align:center;",
                "Current token (refresh then type it): "
                span { style: "color:#7c3aed;font-weight:700;letter-spacing:2px;", "{token_display()}" }
            }
            button {
                onclick: refresh,
                style: "font-size:11px;padding:4px 12px;border-radius:6px;\
                        background:#7c3aed;color:white;border:none;cursor:pointer;",
                "↻ Refresh token"
            }
            Otp {
                max_length: 6, on_complete: Some(on_complete), aria_label: "TOTP validation",
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
            match result() {
                Some(true)  => rsx! { p { style:"color:#22c55e;font-size:12px;margin:0;", "✓ TOTP valid, authenticated!" } },
                Some(false) => rsx! { p { style:"color:#ef4444;font-size:12px;margin:0;", "✗ Token expired or wrong" } },
                None        => rsx! { p { style:"color:#6b7280;font-size:12px;margin:0;", "Type the shown token to verify" } },
            }
        }
    }
}

#[component]
fn ExampleEmailVerify() -> Element {
    let mut value = use_signal(String::new);
    let mut submitted = use_signal(|| false);
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;",
            p { style: "font-size:12px;color:#374151;margin:0;text-align:center;",
                "We sent a code to " strong { "dan@example.com" }
            }
            Otp {
                max_length: 6, value: value(), aria_label: "Email verification code",
                on_change: Some(Callback::new(move |v: String| value.set(v))),
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
            button {
                onclick: move |_| { if value().len() == 6 { submitted.set(true); } },
                disabled: value().len() < 6,
                style: r#"padding:8px 20px;border-radius:8px;background:#7c3aed;color:white;
                        border:none;cursor:pointer;font-size:13px;font-weight:600;
                        opacity:{{if value().len()<6 {{"0.5"}} else {{"1"}};"#,
                "Verify Email"
            }
            if submitted() {
                p { style: "color:#22c55e;font-size:12px;margin:0;", "✓ Email verified!" }
            }
        }
    }
}

#[component]
fn ExampleAlphanumeric() -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:8px;align-items:center;width:100%;",
            p { style: "font-size:11px;color:#6b7280;margin:0;", "8-character alphanumeric recovery code" }
            Otp {
                max_length: 8, pattern: "[A-Z0-9]", aria_label: "Recovery code",
                Group { style: "gap:6px;",
                    Slot { index: 0 } Slot { index: 1 }
                    Slot { index: 2 } Slot { index: 3 }
                }
                Separator {}
                Group { style: "gap:6px;",
                    Slot { index: 4 } Slot { index: 5 }
                    Slot { index: 6 } Slot { index: 7 }
                }
            }
        }
    }
}

#[component]
fn ExampleHeadless() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;",
            p { style: "font-size:11px;color:#6b7280;margin:0;", "Headless, bring your own styles" }
            Otp {
                max_length: 6,
                value: value(),
                on_change: Some(Callback::new(move |v: String| value.set(v))),
                aria_label: "Headless OTP",
                Group {
                    Slot { index: 0, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                    Slot { index: 1, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                    Slot { index: 2, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                }
                Separator { style: "width:8px;height:2px;background:#9ca3af;" }
                Group {
                    Slot { index: 3, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                    Slot { index: 4, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                    Slot { index: 5, style: "width:40px;height:40px;border-radius:4px;\
                        border:1px solid #d1d5db;background:#f9fafb;color:#111827;\
                        font-size:18px;font-weight:600;" }
                }
            }
            p { style: "font-size:12px;color:#374151;", "Typed: \"{value()}\"" }
        }
    }
}

#[component]
fn ExampleTwoFaGate() -> Element {
    let mut otp_value = use_signal(String::new);
    let mut access = use_signal(|| false);
    let mut wrong = use_signal(|| false);
    let expected_token = generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30);

    let on_complete = {
        let expected = expected_token.clone();
        Callback::new(move |code: String| {
            if code == expected {
                access.set(true);
                wrong.set(false);
            } else {
                wrong.set(true);
            }
        })
    };
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;",
            if access() {
                div { style: "padding:16px;border-radius:8px;background:#dcfce7;border:1px solid #86efac;\
                              text-align:center;",
                    p { style: "color:#166534;font-weight:700;margin:0;", "🔓 Access Granted" }
                    p { style: "color:#166534;font-size:11px;margin:4px 0 0;", "2FA verified successfully" }
                }
            } else {
                p { style: "font-size:12px;color:#374151;margin:0;text-align:center;",
                    "Enter your authenticator code"
                    br {}
                    span { style: "color:#7c3aed;font-weight:700;", "(hint: {expected_token})" }
                }
                Otp {
                    max_length: 6,
                    value: otp_value(),
                    on_change: Some(Callback::new(move |v: String| otp_value.set(v))),
                    on_complete: Some(on_complete),
                    is_invalid: wrong(),
                    aria_label: "2FA authenticator code",
                    Group {
                        Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                    }
                    Separator {}
                    Group {
                        Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                    }
                }
                if wrong() {
                    p { role: "alert", style: "color:#ef4444;font-size:12px;margin:0;",
                        "✗ Wrong code. Check your authenticator app."
                    }
                }
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center",
              style: "color:#5e5c7f;background-color:#303030;font-family:'Rubik',sans-serif;overflow-x:hidden;",

            h1 { class: "text-3xl font-bold mb-8 text-white", "OTP RS Dioxus Examples" }

            section { aria_labelledby: "otp-heading", class: "w-full max-w-6xl mb-12",
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    Card {
                        title: "Basic 6-Digit",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
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
}"#.to_string(),
                        ExampleBasic {}
                    }

                    Card {
                        title: "Controlled Value",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        Otp {
            max_length: 6,
            value: value(),
            on_change: Some(Callback::new(move |v: String| value.set(v))),
            aria_label: "Controlled OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
        p { "value: {value():?}" }
    }
}"#.to_string(),
                        ExampleControlled {}
                    }

                    Card {
                        title: "on_complete Callback",
                        code: r#"use otprs::dioxus::{Otp, Group, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    let mut done = use_signal(|| false);
    rsx! {
        Otp {
            max_length: 4,
            on_complete: Some(Callback::new(move |_: String| done.set(true))),
            aria_label: "4-digit PIN",
            Group {
                Slot { index: 0 } Slot { index: 1 }
                Slot { index: 2 } Slot { index: 3 }
            }
        }
        if done() { p { "✓ PIN complete!" } }
    }
}"#.to_string(),
                        ExampleOnComplete {}
                    }

                    Card {
                        title: "Secondary Variant",
                        code: r#"use otprs::common::Variant;
use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        Otp {
            max_length: 6,
            variant: Variant::Secondary,
            aria_label: "Secondary OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}"#.to_string(),
                        ExampleSecondary {}
                    }

                    Card {
                        title: "Disabled State",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        Otp {
            max_length: 6,
            is_disabled: true,
            value: "123456".to_string(),
            aria_label: "Disabled OTP",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}"#.to_string(),
                        ExampleDisabled {}
                    }

                    Card {
                        title: "Invalid / Error State",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        Otp {
            max_length: 6,
            is_invalid: true,
            aria_label: "OTP with error",
            aria_describedby: "otp-err",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
        p { id: "otp-err", role: "alert",
            style: "color:#ef4444;font-size:12px;",
            "Incorrect code. Try again."
        }
    }
}"#.to_string(),
                        ExampleInvalid {}
                    }

                    Card {
                        title: "4-Digit PIN",
                        code: r#"use otprs::dioxus::{Otp, Group, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        Otp { max_length: 4, aria_label: "4-digit PIN",
            Group {
                Slot { index: 0 } Slot { index: 1 }
                Slot { index: 2 } Slot { index: 3 }
            }
        }
    }
}"#.to_string(),
                        ExamplePin {}
                    }

                    Card {
                        title: "HOTP Validation (RFC 4226)",
                        code: r#"use otprs::common::validate_hotp;
use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    let mut result = use_signal(|| Option::<bool>::None);
    let on_complete = Callback::new(move |code: String| {
        let ok = validate_hotp(b"JBSWY3DPEHPK3PXP", 0, &code, 6).is_ok();
        result.set(Some(ok));
    });
    rsx! {
        Otp {
            max_length: 6,
            on_complete: Some(on_complete),
            aria_label: "HOTP code",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}"#.to_string(),
                        ExampleHotp {}
                    }

                    Card {
                        title: "TOTP Validation (RFC 6238, live)",
                        code: r#"use otprs::common::{generate_totp, validate_totp};
use dioxus::prelude::*;

const SECRET: &[u8; 20] = b"12345678901234567890";

use otprs::unix_now_secs;

#[component]
fn Example() -> Element {
    let mut result = use_signal(|| Option::<bool>::None);
    let mut token = use_signal(|| generate_totp(SECRET, unix_now_secs(), 6, 30));
    let on_complete = Callback::new(move |code: String| {
        let ok = validate_totp(SECRET, unix_now_secs(), &code, 6, 30).is_ok();
        result.set(Some(ok));
    });
    rsx! {
        button { onclick: move |_| token.set(generate_totp(SECRET, unix_now_secs(), 6, 30)),
            "↻ Refresh token: {token()}"
        }
        Otp {
            max_length: 6,
            on_complete: Some(on_complete),
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
    }
}"#.to_string(),
                        ExampleTotp {}
                    }

                    Card {
                        title: "Email Verification Flow",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    let mut value = use_signal(String::new);
    let mut submitted = use_signal(|| false);
    rsx! {
        p { "We sent a code to " strong { "dan@example.com" } }
        Otp {
            max_length: 6,
            value: value(),
            on_change: Some(Callback::new(move |v: String| value.set(v))),
            aria_label: "Email verification code",
            Group {
                Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
            }
            Separator {}
            Group {
                Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
            }
        }
        button {
            disabled: value().len() < 6,
            onclick: move |_| { if value().len() == 6 { submitted.set(true); } },
            "Verify Email"
        }
        if submitted() { p { "✓ Email verified!" } }
    }
}"#.to_string(),
                        ExampleEmailVerify {}
                    }

                    Card {
                        title: "Alphanumeric Recovery Code",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

#[component]
fn Example() -> Element {
    rsx! {
        // 8-character alphanumeric recovery code (e.g. GitHub-style)
        Otp {
            max_length: 8,
            pattern: "[A-Z0-9]",
            aria_label: "Recovery code",
            Group { style: "gap:6px;",
                Slot { index: 0 } Slot { index: 1 }
                Slot { index: 2 } Slot { index: 3 }
            }
            Separator {}
            Group { style: "gap:6px;",
                Slot { index: 4 } Slot { index: 5 }
                Slot { index: 6 } Slot { index: 7 }
            }
        }
    }
}"#.to_string(),
                        ExampleAlphanumeric {}
                    }

                    Card {
                        title: "Headless (Bring Your Own CSS)",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

// No default theme, slots styled entirely from scratch via `style` props.
const SLOT: &str = "width:40px;height:40px;border-radius:4px;\
    border:1px solid #d1d5db;background:#f9fafb;\
    color:#111827;font-size:18px;font-weight:600;";

#[component]
fn Example() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        Otp {
            max_length: 6,
            value: value(),
            on_change: Some(Callback::new(move |v: String| value.set(v))),
            Group {
                Slot { index: 0, style: SLOT }
                Slot { index: 1, style: SLOT }
                Slot { index: 2, style: SLOT }
            }
            Separator { style: "width:8px;height:2px;background:#9ca3af;" }
            Group {
                Slot { index: 3, style: SLOT }
                Slot { index: 4, style: SLOT }
                Slot { index: 5, style: SLOT }
            }
        }
    }
}"#.to_string(),
                        ExampleHeadless {}
                    }

                    Card {
                        title: "Custom Per-Slot Style",
                        code: r#"use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

const S: &str = "width:52px;height:60px;border-radius:12px;border-color:#7c3aed;";

#[component]
fn Example() -> Element {
    rsx! {
        Otp { max_length: 6, aria_label: "Custom styled OTP",
            Group { style: "gap:8px;",
                Slot { index: 0, style: S }
                Slot { index: 1, style: S }
                Slot { index: 2, style: S }
            }
            Separator { style: "width:20px;height:3px;background:#7c3aed;" }
            Group { style: "gap:8px;",
                Slot { index: 3, style: S }
                Slot { index: 4, style: S }
                Slot { index: 5, style: S }
            }
        }
    }
}"#.to_string(),
                        ExampleCustomStyle {}
                    }

                    Card {
                        title: "2FA Security Gate",
                        code: r#"use otprs::common::{generate_totp, validate_totp};
use otprs::dioxus::{Otp, Group, Separator, Slot};
use dioxus::prelude::*;

const SECRET: &[u8; 20] = b"12345678901234567890";
use otprs::unix_now_secs;

#[component]
fn Example() -> Element {
    let mut access = use_signal(|| false);
    let mut wrong = use_signal(|| false);
    let token = generate_totp(SECRET, unix_now_secs(), 6, 30);
    let on_complete = {
        let t = token.clone();
        Callback::new(move |code: String| {
            if code == t { access.set(true); }
            else { wrong.set(true); }
        })
    };
    rsx! {
        if access() {
            p { "🔓 Access Granted" }
        } else {
            Otp {
                max_length: 6,
                is_invalid: wrong(),
                on_complete: Some(on_complete),
                aria_label: "2FA authenticator code",
                Group {
                    Slot { index: 0 } Slot { index: 1 } Slot { index: 2 }
                }
                Separator {}
                Group {
                    Slot { index: 3 } Slot { index: 4 } Slot { index: 5 }
                }
            }
        }
    }
}"#.to_string(),
                        ExampleTwoFaGate {}
                    }
                }
            }
        }
    }
}
