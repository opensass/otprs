// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Visual variant for the [`Otp`] container and its [`Slot`] children.
///
/// Selects between a shadowed primary style suitable for standalone usage and a
/// shadow-free secondary style intended for placement inside surface components.
///
/// # Default
///
/// [`Variant::Primary`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use otprs::Variant;
///
/// let class = Variant::Primary.to_container_class();
/// assert_eq!(class, "otp");
///
/// let class = Variant::Secondary.to_container_class();
/// assert_eq!(class, "otp otp--secondary");
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    /// Standard styling with shadow. Default for standalone use.
    #[default]
    Primary,

    /// Lower-emphasis variant without shadow. Suitable for use inside surfaces.
    Secondary,
}

impl Variant {
    /// Returns the BEM CSS class string for the OTP container.
    pub fn to_container_class(self) -> &'static str {
        match self {
            Self::Primary => "otp",
            Self::Secondary => "otp otp--secondary",
        }
    }

    /// Returns the BEM CSS class string for an individual OTP slot.
    pub fn to_slot_class(self) -> &'static str {
        match self {
            Self::Primary => "otp__slot",
            Self::Secondary => "otp__slot otp__slot--secondary",
        }
    }
}

/// Controls the virtual keyboard type shown on mobile devices for OTP input.
///
/// # Default
///
/// [`InputMode::Numeric`] is the default, showing a number pad.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum InputMode {
    /// Shows a numeric keypad (digits 0-9). Best for digit-only OTPs.
    #[default]
    Numeric,

    /// Shows a full text keyboard. Use with letter-only patterns.
    Text,

    /// Shows a full text keyboard with fast access to digits and letters.
    AlphaNumeric,
}

impl InputMode {
    /// Returns the HTML `inputmode` attribute value string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Text => "text",
            Self::AlphaNumeric => "text",
        }
    }
}

/// Regex-like character filter constants for the `pattern` prop.
///
/// These are re-exported for convenience so callers do not need to hard-code
/// the patterns inline.
pub mod pattern {
    /// Allows only ASCII digits (0-9).
    pub const DIGITS_ONLY: &str = r"[0-9]";

    /// Allows only ASCII alphabetic characters (a-z, A-Z).
    pub const CHARS_ONLY: &str = r"[a-zA-Z]";

    /// Allows ASCII digits and alphabetic characters.
    pub const DIGITS_AND_CHARS: &str = r"[0-9a-zA-Z]";
}

/// Errors returned by OTP validation routines.
///
/// Produced by [`validate_hotp`] and [`validate_totp`] when validating the
/// typed code against a shared secret.
#[derive(Debug, Clone, PartialEq)]
pub enum OtpValidationError {
    /// The provided code has an unexpected number of characters.
    InvalidLength {
        /// How many characters the code actually contains.
        got: usize,
        /// How many characters were expected.
        expected: usize,
    },

    /// The provided code contains a character that is not a digit (0-9).
    InvalidCharacter(char),

    /// The TOTP code did not match within the allowed time window.
    TotpExpired,

    /// The HOTP code did not match for the provided counter value.
    HotpCounterMismatch,
}

impl core::fmt::Display for OtpValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength { got, expected } => {
                write!(f, "OTP length mismatch: got {got}, expected {expected}")
            }
            Self::InvalidCharacter(c) => write!(f, "OTP contains invalid character: {c:?}"),
            Self::TotpExpired => write!(f, "TOTP code has expired or is invalid"),
            Self::HotpCounterMismatch => write!(f, "HOTP code does not match for given counter"),
        }
    }
}

/// Build a TOTP instance for a raw secret slice. Internal helper.
fn make_totp(secret: &[u8], digits: u8, period: u64) -> totp_rs::Totp {
    use totp_rs::{Builder, Secret};
    Builder::new()
        .with_secret(Secret::from(secret))
        .with_digits(digits)
        .with_step_duration(period)
        .build_noncompliant()
}

/// Computes the HOTP value for the given secret, counter, and digit length.
///
/// Delegates to [`totp_rs`] for RFC 4226 compliance. HOTP counter mode is
/// modelled as a TOTP with `step_duration = 1` evaluated at `t = counter`.
///
/// # Arguments
///
/// * `secret`: Raw bytes of the shared secret (Base32-decoded before passing).
/// * `counter`: HOTP counter value.
/// * `digits`: Number of OTP digits (6-8).
pub fn hotp(secret: &[u8], counter: u64, digits: u32) -> String {
    make_totp(secret, digits as u8, 1)
        .generate(counter)
        .to_string()
}

/// Validates a user-provided HOTP code against the shared secret and counter.
///
/// Returns `Ok(())` when the code matches, or an [`OtpValidationError`].
pub fn validate_hotp(
    secret: &[u8],
    counter: u64,
    code: &str,
    digits: u32,
) -> Result<(), OtpValidationError> {
    if code.len() != digits as usize {
        return Err(OtpValidationError::InvalidLength {
            got: code.len(),
            expected: digits as usize,
        });
    }
    for c in code.chars() {
        if !c.is_ascii_digit() {
            return Err(OtpValidationError::InvalidCharacter(c));
        }
    }
    if make_totp(secret, digits as u8, 1)
        .generate(counter)
        .to_string()
        == code
    {
        Ok(())
    } else {
        Err(OtpValidationError::HotpCounterMismatch)
    }
}

/// Validates a user-provided TOTP code against the shared secret and current
/// Unix timestamp, delegating to [`totp_rs`] for RFC 6238 compliance.
///
/// Uses a ±1 step tolerance window to handle clock skew.
///
/// # Arguments
///
/// * `secret`: Raw secret bytes (decode Base32 before calling).
/// * `unix_now`: Current Unix timestamp in **seconds**, on WASM use
///   `(js_sys::Date::now() / 1000.0) as u64`.
/// * `code`: User-entered string.
/// * `digits`: Expected code length (usually 6).
/// * `period`: TOTP step in seconds (usually 30).
pub fn validate_totp(
    secret: &[u8],
    unix_now: u64,
    code: &str,
    digits: u32,
    period: u64,
) -> Result<(), OtpValidationError> {
    if code.len() != digits as usize {
        return Err(OtpValidationError::InvalidLength {
            got: code.len(),
            expected: digits as usize,
        });
    }
    for c in code.chars() {
        if !c.is_ascii_digit() {
            return Err(OtpValidationError::InvalidCharacter(c));
        }
    }
    let totp = make_totp(secret, digits as u8, period);
    let step = unix_now / period;
    for delta in 0u64..=2 {
        let t = step.saturating_sub(1).wrapping_add(delta) * period;
        if totp.generate(t).to_string() == code {
            return Ok(());
        }
    }
    Err(OtpValidationError::TotpExpired)
}

/// Generates the current TOTP token for a raw secret slice.
///
/// On WASM, pass `(js_sys::Date::now() / 1000.0) as u64` for `unix_now`.
pub fn generate_totp(secret: &[u8], unix_now: u64, digits: u32, period: u64) -> String {
    make_totp(secret, digits as u8, period)
        .generate(unix_now)
        .to_string()
}

/// Validates a TOTP code using a fixed-size 20-byte stack secret, no heap required.
///
/// WASM-friendly: pass `(js_sys::Date::now() / 1000.0) as u64` for `unix_now`.
pub fn validate_totp_stack(
    secret: &[u8; 20],
    unix_now: u64,
    code: &str,
    digits: u32,
    period: u64,
) -> Result<(), OtpValidationError> {
    validate_totp(secret.as_slice(), unix_now, code, digits, period)
}

/// Generates the current TOTP token for a fixed-size 20-byte stack secret.
///
/// On WASM, pass `(js_sys::Date::now() / 1000.0) as u64`.
pub fn generate_totp_stack(secret: &[u8; 20], unix_now: u64, digits: u32, period: u64) -> String {
    generate_totp(secret.as_slice(), unix_now, digits, period)
}

/// Returns the current Unix timestamp in seconds, using `js_sys::Date`.
///
/// This is the canonical WASM-safe clock. It matches what you'd pass to
/// [`generate_totp`] and [`validate_totp`] in browser environments.
///
/// # Example
///
/// ```rust,ignore
/// use otprs::common::{generate_totp, unix_now_secs};
///
/// const SECRET: &[u8] = b"JBSWY3DPEHPK3PXP";
/// let token = generate_totp(SECRET, unix_now_secs(), 6, 30);
/// ```
#[cfg(any(feature = "lep", feature = "dio", feature = "yew"))]
pub fn unix_now_secs() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

/// Returns the base inline CSS for the OTP container element.
pub fn base_otp_style() -> &'static str {
    "display: inline-flex; align-items: center; gap: 12px; position: relative;"
}

/// Returns the base inline CSS for an OTP inner container element.
pub fn base_otp_inner_style() -> &'static str {
    "display: inline-flex; align-items: center; gap: 8px;"
}

/// Returns the base inline CSS for an individual OTP slot.
pub fn base_slot_style() -> &'static str {
    "display: inline-flex; align-items: center; justify-content: center; \
width: 52px; height: 52px; min-width: 52px; min-height: 52px; flex-shrink: 0; \
border-radius: 12px; border: 2px solid #3f3f46; background-color: #18181b; \
color: #fafafa; font-size: 22px; font-weight: 700; font-family: 'Inter', ui-monospace, monospace; \
line-height: 1; letter-spacing: 0; cursor: text; outline: none; \
transition: border-color 0.15s ease, background-color 0.15s ease, box-shadow 0.15s ease, transform 0.1s ease; \
position: relative; overflow: hidden; user-select: none; box-sizing: border-box;"
}

/// Returns the CSS applied to the active (focused) OTP slot.
pub fn slot_active_style() -> &'static str {
    "border-color: #7c3aed; box-shadow: 0 0 0 3px rgba(124,58,237,0.25);"
}

/// Returns the CSS applied to a filled OTP slot (has a character).
pub fn slot_filled_style() -> &'static str {
    "border-color: #6d28d9; background-color: #1c1c27;"
}

/// Returns the CSS applied to an invalid OTP slot.
pub fn slot_invalid_style() -> &'static str {
    "border-color: #dc2626; box-shadow: 0 0 0 3px rgba(220,38,38,0.2); animation: otp-shake 0.35s ease;"
}

/// Returns the CSS applied to a disabled OTP slot.
pub fn slot_disabled_style() -> &'static str {
    "border-color: #27272a; background-color: #111113; color: #52525b; cursor: not-allowed; opacity: 0.6;"
}

/// Returns the CSS for the blinking caret element inside an active empty slot.
pub fn caret_style() -> &'static str {
    "position: absolute; width: 2px; height: 60%; background-color: #7c3aed; border-radius: 1px; animation: otp-blink 1s step-start infinite;"
}

/// Returns the enter animation CSS applied momentarily when a digit is entered.
pub fn slot_enter_animation_style() -> &'static str {
    "animation: otp-pop 0.18s ease; border-color: #7c3aed;"
}

/// Returns the base inline CSS for the visual separator between slot groups.
pub fn base_separator_style() -> &'static str {
    "width: 12px; height: 2px; background-color: #3f3f46; border-radius: 2px; flex-shrink: 0;"
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
