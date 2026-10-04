// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use leptos::prelude::*;
use otprs::Variant;
use otprs::leptos::{Otp, Group, Separator, Slot};
use otprs::{generate_totp, unix_now_secs};

const TOTP_SECRET: &[u8] = b"JBSWY3DPEHPK3PXP";

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(LandingPage);
}

#[component]
fn ExampleBasic() -> impl IntoView {
    view! {
        <Otp max_length=6 aria_label="Basic 6-digit OTP">
            <Group>
                <Slot index=0 /><Slot index=1 /><Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 /><Slot index=5 />
            </Group>
        </Otp>
    }
}

#[component]
fn ExampleControlled() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <Otp max_length=6
                 value=value.get()
                 on_change=leptos::callback::Callback::new(move |v: String| value.set(v))
                 aria_label="Controlled OTP">
                <Group>
                    <Slot index=0 /><Slot index=1 /><Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 /><Slot index=5 />
                </Group>
            </Otp>
            <p style="font-size:12px;color:#374151;">
                {"Value: "}<strong>{move || value.get()}</strong>
            </p>
        </div>
    }
}

#[component]
fn ExampleOnComplete() -> impl IntoView {
    let message = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <Otp max_length=6
                 on_complete=leptos::callback::Callback::new(move |code: String| {
                     message.set(format!("Complete: {}", code));
                 })
                 aria_label="on_complete OTP">
                <Group>
                    <Slot index=0 /><Slot index=1 /><Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 /><Slot index=5 />
                </Group>
            </Otp>
            <Show when=move || !message.get().is_empty()>
                <p style="color:#7c3aed;font-size:12px;font-weight:700;">{move || message.get()}</p>
            </Show>
        </div>
    }
}

#[component]
fn ExampleVariants() -> impl IntoView {
    let variants = [
        (Variant::Primary, "Primary"),
        (Variant::Secondary, "Secondary"),
    ];
    view! {
        <div style="display:flex;flex-direction:column;gap:16px;width:100%;">
            {variants.into_iter().map(|(v, label)| view! {
                <div style="display:flex;flex-direction:column;gap:6px;align-items:center;">
                    <span style="font-size:10px;color:#6b7280;font-family:monospace;">{label}</span>
                    <Otp max_length=4 variant=v aria_label=label>
                        <Group>
                            <Slot index=0 /><Slot index=1 />
                            <Slot index=2 /><Slot index=3 />
                        </Group>
                    </Otp>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn ExampleDisabled() -> impl IntoView {
    view! {
        <Otp max_length=6 is_disabled=true aria_label="Disabled OTP">
            <Group>
                <Slot index=0 /><Slot index=1 /><Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 /><Slot index=5 />
            </Group>
        </Otp>
    }
}

#[component]
fn ExampleInvalid() -> impl IntoView {
    let is_invalid = RwSignal::new(false);
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;">
            <Otp max_length=6 is_invalid=is_invalid.get()
                 on_complete=leptos::callback::Callback::new(move |code: String| {
                     is_invalid.set(code != "123456");
                 })
                 aria_label="Invalid-state OTP">
                <Group>
                    <Slot index=0 /><Slot index=1 /><Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 /><Slot index=5 />
                </Group>
            </Otp>
            <Show when=move || is_invalid.get()>
                <p role="alert" style="color:#ef4444;font-size:12px;">{"✗ Wrong code (try 123456)"}</p>
            </Show>
        </div>
    }
}

#[component]
fn ExamplePin() -> impl IntoView {
    let verified = RwSignal::new(false);
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;">
            <p style="font-size:12px;color:#6b7280;margin:0;">{"PIN: enter any 4 digits"}</p>
            <Otp max_length=4
                 on_complete=leptos::callback::Callback::new(move |_: String| verified.set(true))
                 aria_label="4-digit PIN">
                <Group>
                    <Slot index=0 /><Slot index=1 />
                    <Slot index=2 /><Slot index=3 />
                </Group>
            </Otp>
            <Show when=move || verified.get()>
                <p style="color:#059669;font-size:12px;font-weight:700;">{"✓ PIN accepted"}</p>
            </Show>
        </div>
    }
}

#[component]
fn ExampleHotp() -> impl IntoView {
    let counter = RwSignal::new(0u64);
    let hotp_val = RwSignal::new(otprs::common::hotp(TOTP_SECRET, 0, 6));
    let status = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <p style="font-size:11px;color:#6b7280;margin:0;text-align:center;">
                {"Counter: "}<strong>{move || counter.get()}</strong>
                {" | Token: "}<code style="color:#7c3aed;">{move || hotp_val.get()}</code>
            </p>
            <Otp max_length=6
                 on_complete=leptos::callback::Callback::new(move |code: String| {
                     if code == hotp_val.get() {
                         status.set("✓ Valid HOTP".into());
                         let next = counter.get() + 1;
                         counter.set(next);
                         hotp_val.set(otprs::common::hotp(TOTP_SECRET, next, 6));
                     } else {
                         status.set("✗ Invalid HOTP".into());
                     }
                 })
                 aria_label="HOTP code">
                <Group>
                    <Slot index=0 /><Slot index=1 /><Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 /><Slot index=5 />
                </Group>
            </Otp>
            <Show when=move || !status.get().is_empty()>
                <p style={move || if status.get().starts_with('✓') {
                    "color:#059669;font-size:12px;font-weight:700;"
                } else {
                    "color:#ef4444;font-size:12px;font-weight:700;"
                }}>{move || status.get()}</p>
            </Show>
        </div>
    }
}

#[component]
fn ExampleTotp() -> impl IntoView {
    let token = RwSignal::new(generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));
    let status = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <div style="display:flex;align-items:center;gap:8px;">
                <span style="font-size:11px;color:#6b7280;">{"Token:"}</span>
                <code style="background:#ede9fe;color:#7c3aed;padding:2px 8px;border-radius:4px;font-weight:700;">
                    {move || token.get()}
                </code>
                <button
                    style="font-size:10px;padding:2px 8px;border:1px solid #d1d5db;border-radius:4px;cursor:pointer;"
                    on:click=move |_| token.set(generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30))
                >{"↻"}</button>
            </div>
            <Otp max_length=6
                 on_complete=leptos::callback::Callback::new(move |code: String| {
                     status.set(if code == token.get() {
                         "✓ TOTP valid".into()
                     } else {
                         "✗ TOTP invalid".into()
                     });
                 })
                 aria_label="TOTP code">
                <Group>
                    <Slot index=0 /><Slot index=1 /><Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 /><Slot index=5 />
                </Group>
            </Otp>
            <Show when=move || !status.get().is_empty()>
                <p style={move || if status.get().starts_with('✓') {
                    "color:#059669;font-size:12px;font-weight:700;"
                } else {
                    "color:#ef4444;font-size:12px;font-weight:700;"
                }}>{move || status.get()}</p>
            </Show>
        </div>
    }
}

#[component]
fn ExampleEmail() -> impl IntoView {
    let sent = RwSignal::new(false);
    let code = RwSignal::new(String::new());
    let status = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <Show when=move || !sent.get()>
                <button
                    style="padding:8px 20px;background:#7c3aed;color:#fff;border:none;border-radius:8px;cursor:pointer;font-size:13px;"
                    on:click=move |_| {
                        code.set("482031".into());
                        sent.set(true);
                    }
                >{"Send verification email"}</button>
            </Show>
            <Show when=move || sent.get()>
                <p style="font-size:12px;color:#374151;margin:0;text-align:center;">
                    {"Code sent to "}<strong>{"dan@example.com"}</strong>
                    <span style="color:#7c3aed;margin-left:4px;">{"(hint: "}{move || code.get()}{")"}</span>
                </p>
                <Otp max_length=6
                     on_complete=leptos::callback::Callback::new(move |entered: String| {
                         status.set(if entered == code.get() {
                             "✓ Email verified!".into()
                         } else {
                             "✗ Wrong code".into()
                         });
                     })
                     aria_label="Email verification code">
                    <Group>
                        <Slot index=0 /><Slot index=1 /><Slot index=2 />
                    </Group>
                    <Separator />
                    <Group>
                        <Slot index=3 /><Slot index=4 /><Slot index=5 />
                    </Group>
                </Otp>
                <Show when=move || !status.get().is_empty()>
                    <p style={move || if status.get().starts_with('✓') {
                        "color:#059669;font-size:12px;font-weight:700;"
                    } else {
                        "color:#ef4444;font-size:12px;font-weight:700;"
                    }}>{move || status.get()}</p>
                </Show>
            </Show>
        </div>
    }
}

#[component]
fn ExampleAlphanumeric() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <p style="font-size:11px;color:#6b7280;margin:0;">{"8-char recovery code (alphanumeric)"}</p>
            <Otp max_length=8
                 pattern="[A-Za-z0-9]"
                 on_change=leptos::callback::Callback::new(move |v: String| value.set(v))
                 aria_label="Alphanumeric recovery code">
                <Group>
                    <Slot index=0 /><Slot index=1 />
                    <Slot index=2 /><Slot index=3 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=4 /><Slot index=5 />
                    <Slot index=6 /><Slot index=7 />
                </Group>
            </Otp>
            <p style="font-size:12px;color:#374151;font-family:monospace;">
                {move || value.get()}
            </p>
        </div>
    }
}

#[component]
fn ExampleHeadless() -> impl IntoView {
    view! {
        <Otp max_length=6 variant=Variant::Secondary aria_label="Headless OTP">
            <Group style="gap:4px;">
                <Slot index=0 /><Slot index=1 /><Slot index=2 />
            </Group>
            <Separator style="width:12px;height:2px;background:#9ca3af;" />
            <Group style="gap:4px;">
                <Slot index=3 /><Slot index=4 /><Slot index=5 />
            </Group>
        </Otp>
    }
}

#[component]
fn ExampleCustomStyle() -> impl IntoView {
    let s = "width:52px;height:52px;border-radius:50%;border:2px solid #7c3aed;\
             background:#ede9fe;display:flex;align-items:center;justify-content:center;\
             font-size:20px;font-weight:700;cursor:text;position:relative;";
    view! {
        <Otp max_length=6 aria_label="Custom styled OTP">
            <Group style="gap:8px;">
                <Slot index=0 style=s /><Slot index=1 style=s /><Slot index=2 style=s />
            </Group>
            <Separator style="width:20px;height:3px;background:#7c3aed;" />
            <Group style="gap:8px;">
                <Slot index=3 style=s /><Slot index=4 style=s /><Slot index=5 style=s />
            </Group>
        </Otp>
    }
}

#[component]
fn ExampleTwoFaGate() -> impl IntoView {
    let access = RwSignal::new(false);
    let wrong = RwSignal::new(false);
    let expected_token = RwSignal::new(generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <Show when=move || access.get()>
                <div style="padding:16px;border-radius:8px;background:#dcfce7;border:1px solid #86efac;text-align:center;">
                    <p style="color:#166534;font-weight:700;margin:0;">{"🔓 Access Granted"}</p>
                    <p style="color:#166534;font-size:11px;margin:4px 0 0;">{"2FA verified successfully"}</p>
                </div>
            </Show>
            <Show when=move || !access.get()>
                <p style="font-size:12px;color:#374151;margin:0;text-align:center;">
                    {"Enter your authenticator code"}
                    <br />
                    <span style="color:#7c3aed;font-weight:700;">
                        {move || format!("(hint: {})", expected_token.get())}
                    </span>
                </p>
                <Otp max_length=6 is_invalid=wrong.get()
                     on_complete=leptos::callback::Callback::new(move |code: String| {
                         if code == expected_token.get() { access.set(true); wrong.set(false); }
                         else { wrong.set(true); }
                     })
                     aria_label="2FA code">
                    <Group><Slot index=0 /><Slot index=1 /><Slot index=2 /></Group>
                    <Separator />
                    <Group><Slot index=3 /><Slot index=4 /><Slot index=5 /></Group>
                </Otp>
                <Show when=move || wrong.get()>
                    <p role="alert" style="color:#ef4444;font-size:12px;margin:0;">
                        {"✗ Wrong code. Check your authenticator app."}
                    </p>
                </Show>
            </Show>
        </div>
    }
}

#[component]
fn Card(title: &'static str, code: String, children: Children) -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;background:#ffffff;border:1px solid #e5e7eb;border-radius:12px;overflow:hidden;box-shadow:0 4px 6px -1px rgba(0,0,0,0.1);">
            <div style="padding:16px;background:#ffffff;">
                <h3 style="margin:0 0 12px 0;font-size:16px;font-weight:600;color:#111827;">{title}</h3>
                <pre style="margin:0;padding:16px;background:#1f2937;color:#f8fafc;font-size:12px;border-radius:8px;overflow-x:auto;font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace;">
                    <code>{code}</code>
                </pre>
            </div>
            <div style="padding:24px;display:flex;flex-direction:column;align-items:center;justify-content:center;background:#f9fafb;border-top:1px solid #e5e7eb;overflow-x:auto;">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div style="background-color:#303030; color:#5e5c7f; font-family:'Rubik', sans-serif; min-height:100vh; padding: 24px; display: flex; flex-direction: column; align-items: center; overflow-x: hidden;">
            <h1 style="font-size: 1.875rem; font-weight: 700; margin-bottom: 2rem; color: #ffffff; text-align: center;">{ "OTP RS Leptos Examples" }</h1>

            <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(320px,1fr));gap:24px;width:100%;max-width:1200px;">

                <Card title="Basic 6-Digit" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn BasicOtp() -> impl IntoView {
    view! {
        <Otp max_length=6 aria_label="Enter code">
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
}"#.to_string()>
                    <ExampleBasic />
                </Card>

                <Card title="Controlled with Display" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn ControlledOtp() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <Otp max_length=6
             value=value.get()
             on_change=Callback::new(move |v: String| {
                 value.set(v);
             })
             aria_label="Controlled OTP">
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
        <p>{"Value: "}<strong>{move || value.get()}</strong></p>
    }
}"#.to_string()>
                    <ExampleControlled />
                </Card>

                <Card title="on_complete Callback" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn OnCompleteOtp() -> impl IntoView {
    let message = RwSignal::new(String::new());
    view! {
        <Otp max_length=6
             on_complete=Callback::new(move |code: String| {
                 message.set(format!("Complete: {}", code));
             })
             aria_label="on_complete OTP">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
        <Show when=move || !message.get().is_empty()>
            <p style="color:#7c3aed;">{move || message.get()}</p>
        </Show>
    }
}"#.to_string()>
                    <ExampleOnComplete />
                </Card>

                <Card title="Variants" code=r#"use otprs::leptos::{Otp, Group, Slot};
use otprs::Variant;
use leptos::prelude::*;

#[component]
fn VariantsOtp() -> impl IntoView {
    view! {
        // Variant::Primary (default)
        <Otp max_length=4 variant=Variant::Primary>
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
        </Otp>
        // Variant::Secondary
        <Otp max_length=4 variant=Variant::Secondary>
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
        </Otp>
        // Variant::Outline
        <Otp max_length=4 variant=Variant::Outline>
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
        </Otp>
        // Variant::Ghost (headless)
        <Otp max_length=4 variant=Variant::Ghost>
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
        </Otp>
    }
}"#.to_string()>
                    <ExampleVariants />
                </Card>

                <Card title="Disabled" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn DisabledOtp() -> impl IntoView {
    view! {
        <Otp max_length=6 is_disabled=true
             aria_label="Disabled OTP input">
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
}"#.to_string()>
                    <ExampleDisabled />
                </Card>

                <Card title="Invalid State" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn InvalidOtp() -> impl IntoView {
    let is_invalid = RwSignal::new(false);
    view! {
        <Otp max_length=6 is_invalid=is_invalid.get()
             on_complete=Callback::new(move |code: String| {
                 is_invalid.set(code != "123456");
             })
             aria_label="Validation OTP">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
        <Show when=move || is_invalid.get()>
            <p role="alert" style="color:#ef4444;">
                {"✗ Wrong code (try 123456)"}
            </p>
        </Show>
    }
}"#.to_string()>
                    <ExampleInvalid />
                </Card>

                <Card title="4-Digit PIN" code=r#"use otprs::leptos::{Otp, Group, Slot};
use leptos::prelude::*;

#[component]
fn PinOtp() -> impl IntoView {
    let verified = RwSignal::new(false);
    view! {
        <Otp max_length=4
             on_complete=Callback::new(move |_: String| {
                 verified.set(true);
             })
             aria_label="4-digit PIN">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
        </Otp>
        <Show when=move || verified.get()>
            <p style="color:#059669;font-weight:700;">{"✓ PIN accepted"}</p>
        </Show>
    }
}"#.to_string()>
                    <ExamplePin />
                </Card>

                <Card title="HOTP Validation" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use otprs::common::generate_hotp;
use leptos::prelude::*;

const SECRET: &[u8] = b"JBSWY3DPEHPK3PXP";

#[component]
fn HotpOtp() -> impl IntoView {
    let counter  = RwSignal::new(0u64);
    let hotp_val = RwSignal::new(generate_hotp(SECRET, 0, 6));
    let status   = RwSignal::new(String::new());
    view! {
        <p>{"Counter: "}<strong>{move || counter.get()}</strong>
           {" | Token: "}<code>{move || hotp_val.get()}</code></p>
        <Otp max_length=6
             on_complete=Callback::new(move |code: String| {
                 if code == hotp_val.get() {
                     let next = counter.get() + 1;
                     counter.set(next);
                     hotp_val.set(generate_hotp(SECRET, next, 6));
                     status.set("✓ Valid".into());
                 } else {
                     status.set("✗ Invalid".into());
                 }
             })
             aria_label="HOTP code">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
    }
}"#.to_string()>
                    <ExampleHotp />
                </Card>

                <Card title="TOTP Live Validation" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use otprs::common::{generate_totp, unix_now_secs};
use leptos::prelude::*;

const SECRET: &[u8] = b"JBSWY3DPEHPK3PXP";

#[component]
fn TotpOtp() -> impl IntoView {
    let token  = RwSignal::new(
        generate_totp(SECRET, unix_now_secs(), 6, 30));
    let status = RwSignal::new(String::new());
    view! {
        <div style="display:flex;align-items:center;gap:8px;">
            <span>{"Token:"}</span>
            <code style="color:#7c3aed;">{move || token.get()}</code>
            <button on:click=move |_| {
                token.set(generate_totp(SECRET, unix_now_secs(), 6, 30));
            }>{"↻"}</button>
        </div>
        <Otp max_length=6
             on_complete=Callback::new(move |code: String| {
                 status.set(if code == token.get() {
                     "✓ TOTP valid".into()
                 } else {
                     "✗ Invalid".into()
                 });
             })
             aria_label="TOTP code">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator />
            <Group>
                <Slot index=3 /><Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
    }
}"#.to_string()>
                    <ExampleTotp />
                </Card>

                <Card title="Email Verification" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn EmailOtp() -> impl IntoView {
    let sent   = RwSignal::new(false);
    let code   = RwSignal::new(String::new());
    let status = RwSignal::new(String::new());
    view! {
        <Show when=move || !sent.get()>
            <button on:click=move |_| {
                // In production: send real email code via API
                code.set("482031".into());
                sent.set(true);
            }>{"Send verification email"}</button>
        </Show>
        <Show when=move || sent.get()>
            <p>{"Code sent to "}<strong>{"dan@example.com"}</strong></p>
            <Otp max_length=6
                 on_complete=Callback::new(move |entered: String| {
                     status.set(if entered == code.get() {
                         "✓ Email verified!".into()
                     } else {
                         "✗ Wrong code".into()
                     });
                 })
                 aria_label="Email verification code">
                <Group>
                    <Slot index=0 /><Slot index=1 />
                    <Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 />
                    <Slot index=5 />
                </Group>
            </Otp>
        </Show>
    }
}"#.to_string()>
                    <ExampleEmail />
                </Card>

                <Card title="Alphanumeric Recovery" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

#[component]
fn AlphaOtp() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <Otp max_length=8
             pattern="[A-Za-z0-9]"
             on_change=Callback::new(move |v: String| value.set(v))
             aria_label="Recovery code">
            <Group>
                <Slot index=0 /><Slot index=1 />
                <Slot index=2 /><Slot index=3 />
            </Group>
            <Separator />
            <Group>
                <Slot index=4 /><Slot index=5 />
                <Slot index=6 /><Slot index=7 />
            </Group>
        </Otp>
        <p style="font-family:monospace;">{move || value.get()}</p>
    }
}"#.to_string()>
                    <ExampleAlphanumeric />
                </Card>

                <Card title="2FA Security Gate" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use otprs::common::{generate_totp, unix_now_secs};
use leptos::prelude::*;

const SECRET: &[u8] = b"JBSWY3DPEHPK3PXP";

#[component]
fn TwoFaGate() -> impl IntoView {
    let access = RwSignal::new(false);
    let wrong  = RwSignal::new(false);
    let token  = RwSignal::new(
        generate_totp(SECRET, unix_now_secs(), 6, 30));
    view! {
        <Show when=move || access.get()>
            <div style="background:#dcfce7;padding:16px;border-radius:8px;">
                <p style="color:#166534;font-weight:700;">
                    {"🔓 Access Granted"}
                </p>
            </div>
        </Show>
        <Show when=move || !access.get()>
            <p>{"Token: "}<code style="color:#7c3aed;">
                {move || token.get()}
            </code></p>
            <Otp max_length=6 is_invalid=wrong.get()
                 on_complete=Callback::new(move |code: String| {
                     if code == token.get() {
                         access.set(true);
                         wrong.set(false);
                     } else {
                         wrong.set(true);
                     }
                 })
                 aria_label="2FA code">
                <Group>
                    <Slot index=0 /><Slot index=1 />
                    <Slot index=2 />
                </Group>
                <Separator />
                <Group>
                    <Slot index=3 /><Slot index=4 />
                    <Slot index=5 />
                </Group>
            </Otp>
            <Show when=move || wrong.get()>
                <p role="alert" style="color:#ef4444;">
                    {"✗ Wrong code"}
                </p>
            </Show>
        </Show>
    }
}"#.to_string()>
                    <ExampleTwoFaGate />
                </Card>

                <Card title="Headless (Ghost Variant)" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use otprs::Variant;
use leptos::prelude::*;

// Ghost variant removes all built-in slot styling.
// Use style= on Slot for full visual control.
#[component]
fn HeadlessOtp() -> impl IntoView {
    view! {
        <Otp max_length=6 variant=Variant::Ghost
             aria_label="Headless OTP">
            <Group style="gap:4px;">
                <Slot index=0 />
                <Slot index=1 />
                <Slot index=2 />
            </Group>
            <Separator
                style="width:12px;height:2px;background:#9ca3af;" />
            <Group style="gap:4px;">
                <Slot index=3 />
                <Slot index=4 />
                <Slot index=5 />
            </Group>
        </Otp>
    }
}"#.to_string()>
                    <ExampleHeadless />
                </Card>

                <Card title="Custom Per-Slot Style" code=r#"use otprs::leptos::{Otp, Group, Separator, Slot};
use leptos::prelude::*;

// Each Slot accepts a style= prop for per-slot CSS.
const SLOT: &str = "width:52px;height:52px;\
    border-radius:50%;\
    border:2px solid #7c3aed;\
    background:#ede9fe;\
    display:flex;align-items:center;\
    justify-content:center;\
    font-size:20px;font-weight:700;\
    cursor:text;position:relative;";

#[component]
fn StyledOtp() -> impl IntoView {
    view! {
        <Otp max_length=6 aria_label="Styled OTP">
            <Group style="gap:8px;">
                <Slot index=0 style=SLOT />
                <Slot index=1 style=SLOT />
                <Slot index=2 style=SLOT />
            </Group>
            <Separator
                style="width:20px;height:3px;background:#7c3aed;" />
            <Group style="gap:8px;">
                <Slot index=3 style=SLOT />
                <Slot index=4 style=SLOT />
                <Slot index=5 style=SLOT />
            </Group>
        </Otp>
    }
}"#.to_string()>
                    <ExampleCustomStyle />
                </Card>

            </div>
        </div>
    }
}
