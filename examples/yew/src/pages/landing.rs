// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use otprs::common::{Variant, generate_totp, unix_now_secs, validate_hotp, validate_totp};
use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

const TOTP_SECRET: &[u8; 20] = b"12345678901234567890";

#[function_component(ExampleBasic)]
pub fn example_basic() -> Html {
    html! {
        <Otp max_length=6 aria_label="Basic 6-digit OTP">
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

#[function_component(ExampleControlled)]
pub fn example_controlled() -> Html {
    let value = use_state(String::new);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    html! {
        <div style="display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;">
            <Otp
                max_length=6
                value={(*value).clone()}
                on_change={on_change}
                aria_label="Controlled OTP"
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
            <p style="font-size:12px;color:#6b7280;margin:0;">{ format!("value: {:?}", *value) }</p>
        </div>
    }
}

#[function_component(ExampleOnComplete)]
pub fn example_on_complete() -> Html {
    let done = use_state(|| false);
    let code = use_state(String::new);
    let on_complete = {
        let done = done.clone();
        let code = code.clone();
        Callback::from(move |s: String| {
            done.set(true);
            code.set(s);
        })
    };
    html! {
        <div style="display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;">
            <Otp max_length=4 on_complete={on_complete} aria_label="4-digit PIN">
                <Group>
                    <Slot index=0 />
                    <Slot index=1 />
                    <Slot index=2 />
                    <Slot index=3 />
                </Group>
            </Otp>
            if *done {
                <p style="color:#22c55e;font-size:12px;margin:0;">{ format!("✓ {}", *code) }</p>
            } else {
                <p style="color:#6b7280;font-size:12px;margin:0;">{ "Enter all 4 digits" }</p>
            }
        </div>
    }
}

#[function_component(ExampleSecondary)]
pub fn example_secondary() -> Html {
    html! {
        <Otp max_length=6 variant={Variant::Secondary} aria_label="Secondary OTP">
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

#[function_component(ExampleDisabled)]
pub fn example_disabled() -> Html {
    html! {
        <Otp max_length=6 is_disabled=true value={"123456".to_string()} aria_label="Disabled OTP">
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

#[function_component(ExampleInvalid)]
pub fn example_invalid() -> Html {
    html! {
        <div style="display:flex;flex-direction:column;gap:8px;align-items:center;width:100%;">
            <Otp
                max_length=6
                is_invalid=true
                aria_label="Invalid OTP"
                aria_describedby="yew-otp-err"
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
            <p id="yew-otp-err" role="alert" style="color:#ef4444;font-size:12px;margin:0;">
                { "Incorrect code. Try again." }
            </p>
        </div>
    }
}

#[function_component(ExamplePin)]
pub fn example_pin() -> Html {
    html! {
        <Otp max_length=4 aria_label="4-digit PIN">
            <Group>
                <Slot index=0 />
                <Slot index=1 />
                <Slot index=2 />
                <Slot index=3 />
            </Group>
        </Otp>
    }
}

#[function_component(ExampleCustomStyle)]
pub fn example_custom_style() -> Html {
    const S: &str = "width:52px;height:60px;border-radius:12px;border-color:#7c3aed;";
    html! {
        <Otp max_length=6 aria_label="Custom styled OTP">
            <Group style="gap:8px;">
                <Slot index=0 style={S} />
                <Slot index=1 style={S} />
                <Slot index=2 style={S} />
            </Group>
            <Separator style="width:20px;height:3px;background:#7c3aed;" />
            <Group style="gap:8px;">
                <Slot index=3 style={S} />
                <Slot index=4 style={S} />
                <Slot index=5 style={S} />
            </Group>
        </Otp>
    }
}

#[function_component(ExampleHotp)]
pub fn example_hotp() -> Html {
    let result = use_state(|| Option::<bool>::None);
    let on_complete = {
        let result = result.clone();
        Callback::from(move |code: String| {
            let ok = validate_hotp(b"JBSWY3DPEHPK3PXP", 0, &code, 6).is_ok();
            result.set(Some(ok));
        })
    };
    html! {
        <div style="display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;">
            <Otp max_length=6 on_complete={on_complete} aria_label="HOTP validation">
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
            { match *result {
                    Some(true)  => html! { <p style="color:#22c55e;font-size:12px;margin:0;">{"✓ Valid HOTP"}</p> },
                    Some(false) => html! { <p style="color:#ef4444;font-size:12px;margin:0;">{"✗ Invalid code"}</p> },
                    None        => html! { <p style="color:#6b7280;font-size:12px;margin:0;">{"Fill 6 digits to validate"}</p> },
                } }
        </div>
    }
}

#[function_component(ExampleTotp)]
pub fn example_totp() -> Html {
    let result = use_state(|| Option::<bool>::None);
    let token_display = use_state(|| generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));

    let on_complete = {
        let result = result.clone();
        Callback::from(move |code: String| {
            let ok = validate_totp(TOTP_SECRET, unix_now_secs(), &code, 6, 30).is_ok();
            result.set(Some(ok));
        })
    };
    let refresh = {
        let token_display = token_display.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            token_display.set(generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30));
            result.set(None);
        })
    };
    html! {
        <div style="display:flex;flex-direction:column;gap:10px;align-items:center;width:100%;">
            <p style="font-size:11px;color:#6b7280;margin:0;text-align:center;">
                { "Current token (refresh then type it): " }
                <span style="color:#7c3aed;font-weight:700;letter-spacing:2px;">
                    { (*token_display).clone() }
                </span>
            </p>
            <button
                onclick={refresh}
                style="font-size:11px;padding:4px 12px;border-radius:6px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;"
            >
                { "↻ Refresh token" }
            </button>
            <Otp max_length=6 on_complete={on_complete} aria_label="TOTP validation">
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
            { match *result {
                    Some(true)  => html! { <p style="color:#22c55e;font-size:12px;margin:0;">{"✓ TOTP valid, authenticated!"}</p> },
                    Some(false) => html! { <p style="color:#ef4444;font-size:12px;margin:0;">{"✗ Token expired or wrong"}</p> },
                    None        => html! { <p style="color:#6b7280;font-size:12px;margin:0;">{"Type the shown token to verify"}</p> },
                } }
        </div>
    }
}

#[function_component(ExampleEmailVerify)]
pub fn example_email_verify() -> Html {
    let value = use_state(String::new);
    let submitted = use_state(|| false);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    let submit = {
        let value = value.clone();
        let submitted = submitted.clone();
        Callback::from(move |_: MouseEvent| {
            if value.len() == 6 {
                submitted.set(true);
            }
        })
    };
    let disabled = value.len() < 6;
    html! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <p style="font-size:12px;color:#374151;margin:0;text-align:center;">
                { "We sent a code to " }
                <strong>{ "dan@example.com" }</strong>
            </p>
            <Otp
                max_length=6
                value={(*value).clone()}
                on_change={on_change}
                aria_label="Email verification code"
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
            <button
                onclick={submit}
                disabled={disabled}
                style={format!("padding:8px 20px;border-radius:8px;background:#7c3aed;\
                       color:white;border:none;cursor:pointer;font-size:13px;font-weight:600;\
                       opacity:{};", if disabled { "0.5" } else { "1" })}
            >
                { "Verify Email" }
            </button>
            if *submitted {
                <p style="color:#22c55e;font-size:12px;margin:0;">{ "✓ Email verified!" }</p>
            }
        </div>
    }
}

#[function_component(ExampleAlphanumeric)]
pub fn example_alphanumeric() -> Html {
    html! {
        <div style="display:flex;flex-direction:column;gap:8px;align-items:center;width:100%;">
            <p style="font-size:11px;color:#6b7280;margin:0;">
                { "8-character alphanumeric recovery code" }
            </p>
            <Otp max_length=8 pattern="[A-Z0-9]" aria_label="Recovery code">
                <Group style="gap:6px;">
                    <Slot index=0 />
                    <Slot index=1 />
                    <Slot index=2 />
                    <Slot index=3 />
                </Group>
                <Separator />
                <Group style="gap:6px;">
                    <Slot index=4 />
                    <Slot index=5 />
                    <Slot index=6 />
                    <Slot index=7 />
                </Group>
            </Otp>
        </div>
    }
}

#[function_component(ExampleHeadless)]
pub fn example_headless() -> Html {
    let value = use_state(String::new);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    const SLOT: &str = "width:40px;height:40px;border-radius:4px;\
        border:1px solid #d1d5db;background:#f9fafb;\
        color:#111827;font-size:18px;font-weight:600;";
    html! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            <p style="font-size:11px;color:#6b7280;margin:0;">
                { "Headless, bring your own styles" }
            </p>
            <Otp
                max_length=6
                value={(*value).clone()}
                on_change={on_change}
                aria_label="Headless OTP"
            >
                <Group>
                    <Slot index=0 style={SLOT} />
                    <Slot index=1 style={SLOT} />
                    <Slot index=2 style={SLOT} />
                </Group>
                <Separator style="width:8px;height:2px;background:#9ca3af;" />
                <Group>
                    <Slot index=3 style={SLOT} />
                    <Slot index=4 style={SLOT} />
                    <Slot index=5 style={SLOT} />
                </Group>
            </Otp>
            <p style="font-size:12px;color:#374151;">{ format!("Typed: {:?}", *value) }</p>
        </div>
    }
}

#[function_component(ExampleTwoFaGate)]
pub fn example_two_fa_gate() -> Html {
    let access = use_state(|| false);
    let wrong = use_state(|| false);
    let expected_token = generate_totp(TOTP_SECRET, unix_now_secs(), 6, 30);

    let on_complete = {
        let access = access.clone();
        let wrong = wrong.clone();
        let expected = expected_token.clone();
        Callback::from(move |code: String| {
            if code == expected {
                access.set(true);
                wrong.set(false);
            } else {
                wrong.set(true);
            }
        })
    };
    html! {
        <div style="display:flex;flex-direction:column;gap:12px;align-items:center;width:100%;">
            if *access {
                <div
                    style="padding:16px;border-radius:8px;background:#dcfce7;border:1px solid #86efac;text-align:center;"
                >
                    <p style="color:#166534;font-weight:700;margin:0;">{ "🔓 Access Granted" }</p>
                    <p style="color:#166534;font-size:11px;margin:4px 0 0;">
                        { "2FA verified successfully" }
                    </p>
                </div>
            } else {
                <p style="font-size:12px;color:#374151;margin:0;text-align:center;">
                    { "Enter your authenticator code" }
                    <br />
                    <span style="color:#7c3aed;font-weight:700;">
                        { format!("(hint: {})", expected_token) }
                    </span>
                </p>
                <Otp
                    max_length=6
                    is_invalid={*wrong}
                    on_complete={on_complete}
                    aria_label="2FA authenticator code"
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
                if *wrong {
                    <p role="alert" style="color:#ef4444;font-size:12px;margin:0;">
                        { "✗ Wrong code. Check your authenticator app." }
                    </p>
                }
            }
        </div>
    }
}

#[function_component(LandingPage)]
pub fn landing() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "OTP RS Yew Examples" }</h1>
            <section aria-labelledby="otp-heading" class="w-full max-w-6xl mb-12">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Basic 6-Digit" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(BasicOtp)]
pub fn basic_otp() -> Html {
    html! {
        <Otp max_length={6} aria_label="Enter code">
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
}"# }
                        </pre>
                        <ExampleBasic />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Controlled Value" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(ControlledOtp)]
pub fn controlled_otp() -> Html {
    let value = use_state(String::new);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    html! {
        <Otp max_length={6} value={(*value).clone()}
             on_change={on_change} aria_label="Controlled OTP">
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
}"# }
                        </pre>
                        <ExampleControlled />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "on_complete Callback" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Slot};
use yew::prelude::*;

#[function_component(CompleteOtp)]
pub fn complete_otp() -> Html {
    let done = use_state(|| false);
    let on_complete = {
        let done = done.clone();
        Callback::from(move |_: String| done.set(true))
    };
    html! {
        <Otp max_length={4} on_complete={on_complete}
             aria_label="4-digit PIN">
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
                <Slot index={3} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExampleOnComplete />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Secondary Variant" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::common::Variant;
use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(SecondaryOtp)]
pub fn secondary_otp() -> Html {
    html! {
        <Otp max_length={6} variant={Variant::Secondary}
             aria_label="Secondary OTP">
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
}"# }
                        </pre>
                        <ExampleSecondary />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Disabled State" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(DisabledOtp)]
pub fn disabled_otp() -> Html {
    html! {
        <Otp max_length={6} is_disabled={true}
             value={"123456".to_string()}
             aria_label="Disabled OTP">
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
}"# }
                        </pre>
                        <ExampleDisabled />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Invalid / Error State" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(InvalidOtp)]
pub fn invalid_otp() -> Html {
    html! {
        <>
            <Otp max_length={6} is_invalid={true}
                 aria_label="OTP with error"
                 aria_describedby="otp-error">
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
            <p id="otp-error" role="alert">
                { "Incorrect code. Try again." }
            </p>
        </>
    }
}"# }
                        </pre>
                        <ExampleInvalid />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "4-Digit PIN" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Slot};
use yew::prelude::*;

#[function_component(PinOtp)]
pub fn pin_otp() -> Html {
    html! {
        <Otp max_length={4} aria_label="4-digit PIN">
            <Group>
                <Slot index={0} />
                <Slot index={1} />
                <Slot index={2} />
                <Slot index={3} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExamplePin />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "HOTP Validation (RFC 4226)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::common::validate_hotp;
use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(HotpOtp)]
pub fn hotp_otp() -> Html {
    let result = use_state(|| Option::<bool>::None);
    let on_complete = {
        let result = result.clone();
        Callback::from(move |code: String| {
            let ok = validate_hotp(
                b"JBSWY3DPEHPK3PXP", 0, &code, 6
            ).is_ok();
            result.set(Some(ok));
        })
    };
    html! {
        <Otp max_length={6} on_complete={on_complete}
             aria_label="HOTP code">
            <Group>
                <Slot index={0} /><Slot index={1} /><Slot index={2} />
            </Group>
            <Separator />
            <Group>
                <Slot index={3} /><Slot index={4} /><Slot index={5} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExampleHotp />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">
                            { "TOTP Validation (RFC 6238, live)" }
                        </h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::common::{generate_totp, validate_totp};
use otprs::{unix_now_secs, yew::{Otp, Group, Separator, Slot}};
use yew::prelude::*;

const SECRET: &[u8; 20] = b"12345678901234567890";

#[function_component(TotpOtp)]
pub fn totp_otp() -> Html {
    let result = use_state(|| Option::<bool>::None);
    let token = use_state(|| generate_totp(SECRET, unix_now_secs(), 6, 30));
    let on_complete = {
        let result = result.clone();
        Callback::from(move |code: String| {
            let ok = validate_totp(
                SECRET, unix_now_secs(), &code, 6, 30
            ).is_ok();
            result.set(Some(ok));
        })
    };
    let refresh = {
        let token = token.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            token.set(generate_totp(SECRET, unix_now_secs(), 6, 30));
            result.set(None);
        })
    };
    html! {
        <>
            <p>{"Token: "}<span>{(*token).clone()}</span></p>
            <button onclick={refresh}>{"↻ Refresh"}</button>
            <Otp max_length={6} on_complete={on_complete}
                 aria_label="TOTP validation">
                <Group>
                    <Slot index={0} /><Slot index={1} /><Slot index={2} />
                </Group>
                <Separator />
                <Group>
                    <Slot index={3} /><Slot index={4} /><Slot index={5} />
                </Group>
            </Otp>
        </>
    }
}"# }
                        </pre>
                        <ExampleTotp />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Email Verification Flow" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

#[function_component(EmailVerify)]
pub fn email_verify() -> Html {
    let value = use_state(String::new);
    let submitted = use_state(|| false);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    let submit = {
        let value = value.clone();
        let submitted = submitted.clone();
        Callback::from(move |_: MouseEvent| {
            if value.len() == 6 { submitted.set(true); }
        })
    };
    html! {
        <>
            <p>{ "Code sent to " }<strong>{ "dan@example.com" }</strong></p>
            <Otp max_length={6} value={(*value).clone()}
                 on_change={on_change}>
                <Group>
                    <Slot index={0} /><Slot index={1} /><Slot index={2} />
                </Group>
                <Separator />
                <Group>
                    <Slot index={3} /><Slot index={4} /><Slot index={5} />
                </Group>
            </Otp>
            <button onclick={submit}
                    disabled={value.len() < 6}>
                { "Verify Email" }
            </button>
            if *submitted { <p>{ "✓ Verified!" }</p> }
        </>
    }
}"# }
                        </pre>
                        <ExampleEmailVerify />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Alphanumeric Recovery Code" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

// 8-char GitHub-style recovery code
#[function_component(RecoveryCode)]
pub fn recovery_code() -> Html {
    html! {
        <Otp max_length={8} pattern="[A-Z0-9]"
             aria_label="Recovery code">
            <Group style="gap:6px;">
                <Slot index={0} /><Slot index={1} />
                <Slot index={2} /><Slot index={3} />
            </Group>
            <Separator />
            <Group style="gap:6px;">
                <Slot index={4} /><Slot index={5} />
                <Slot index={6} /><Slot index={7} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExampleAlphanumeric />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Headless (Bring Your Own CSS)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

// Completely unstyled, full CSS control via style props
const SLOT: &str = "width:40px;height:40px;border-radius:4px;\
    border:1px solid #d1d5db;background:#f9fafb;\
    color:#111827;font-size:18px;font-weight:600;";

#[function_component(HeadlessOtp)]
pub fn headless_otp() -> Html {
    let value = use_state(String::new);
    let on_change = {
        let v = value.clone();
        Callback::from(move |s: String| v.set(s))
    };
    html! {
        <Otp max_length={6} value={(*value).clone()}
             on_change={on_change}>
            <Group>
                <Slot index={0} style={SLOT} />
                <Slot index={1} style={SLOT} />
                <Slot index={2} style={SLOT} />
            </Group>
            <Separator style="width:8px;height:2px;background:#9ca3af;" />
            <Group>
                <Slot index={3} style={SLOT} />
                <Slot index={4} style={SLOT} />
                <Slot index={5} style={SLOT} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExampleHeadless />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Per-Slot Style" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::yew::{Otp, Group, Separator, Slot};
use yew::prelude::*;

const S: &str = "width:52px;height:60px;\
    border-radius:12px;border-color:#7c3aed;";

#[function_component(StyledOtp)]
pub fn styled_otp() -> Html {
    html! {
        <Otp max_length={6} aria_label="Custom styled OTP">
            <Group style="gap:8px;">
                <Slot index={0} style={S} />
                <Slot index={1} style={S} />
                <Slot index={2} style={S} />
            </Group>
            <Separator
                style="width:20px;height:3px;background:#7c3aed;" />
            <Group style="gap:8px;">
                <Slot index={3} style={S} />
                <Slot index={4} style={S} />
                <Slot index={5} style={S} />
            </Group>
        </Otp>
    }
}"# }
                        </pre>
                        <ExampleCustomStyle />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "2FA Security Gate" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use otprs::common::generate_totp;
use otprs::{unix_now_secs, yew::{Otp, Group, Separator, Slot}};
use yew::prelude::*;

const SECRET: &[u8; 20] = b"12345678901234567890";

#[function_component(TwoFaGate)]
pub fn two_fa_gate() -> Html {
    let access = use_state(|| false);
    let wrong = use_state(|| false);
    let token = generate_totp(SECRET, unix_now_secs(), 6, 30);
    let on_complete = {
        let access = access.clone();
        let wrong = wrong.clone();
        let t = token.clone();
        Callback::from(move |code: String| {
            if code == t { access.set(true); wrong.set(false); }
            else { wrong.set(true); }
        })
    };
    html! {
        if *access {
            <p>{"🔓 Access Granted"}</p>
        } else {
            <p>{format!("(hint: {})", token)}</p>
            <Otp max_length={6} is_invalid={*wrong}
                 on_complete={on_complete}>
                <Group>
                    <Slot index={0} /><Slot index={1} /><Slot index={2} />
                </Group>
                <Separator />
                <Group>
                    <Slot index={3} /><Slot index={4} /><Slot index={5} />
                </Group>
            </Otp>
        }
    }
}"# }
                        </pre>
                        <ExampleTwoFaGate />
                    </article>
                </div>
            </section>
        </div>
    }
}
