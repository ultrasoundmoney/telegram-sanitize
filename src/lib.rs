//! Pure string sanitation helpers for Telegram Bot API messages.
//!
//! Pick the helper that matches the exact `parse_mode` you send to Telegram:
//!
//! - Use [`plain_text`] when omitting `parse_mode`. This is the safest choice
//!   for operational alerts that do not need rich formatting.
//! - Use [`markdown_v2`] helpers only when sending `parse_mode = "MarkdownV2"`.
//! - Use [`html`] helpers only when sending `parse_mode = "HTML"`.
//! - Prefer plain text for critical alerts unless formatting materially improves
//!   readability.
//! - Normalize or truncate caller-specific fields before sanitation when
//!   desired.
//!
//! Helpers return complete fragments that can be inserted directly into a Rust
//! `format!` string. They do not send Telegram messages, build HTTP payloads,
//! format whole alerts, or truncate fragments.
//!
//! Telegram documents `sendMessage.text` as 1-4096 characters after entity
//! parsing. This crate exposes [`fits_send_message_text_limit_estimate`] as a
//! guardrail, but final message sizing remains the caller's responsibility.
//!
//! # Examples
//!
//! Plain text alerts are the best default for rare operational failures:
//!
//! ```
//! use telegram_sanitize::{fits_send_message_text_limit_estimate, plain_text};
//!
//! let error_summary: String = "db\nunavailable".replace(char::is_control, " ");
//! let message = format!(
//!     "cielago alert: relay submissions disabled\nlabel: {}\nerror: {}",
//!     plain_text("rbx-prod-mainnet"),
//!     plain_text(&error_summary),
//! );
//!
//! assert!(fits_send_message_text_limit_estimate(&message));
//! ```
//!
//! For MarkdownV2, sanitize dynamic fragments with the MarkdownV2 helpers:
//!
//! ```
//! use telegram_sanitize::markdown_v2;
//!
//! let message = format!(
//!     "*builder demoted*\nslot: {}\nerror:\n{}",
//!     markdown_v2::inline_code("12345"),
//!     markdown_v2::code_block("simulation failed:\ninvalid `root`"),
//! );
//! ```
//!
//! For HTML, combine static markup with sanitized dynamic fragments:
//!
//! ```
//! use telegram_sanitize::html;
//!
//! let message = format!(
//!     "<b>builder demoted</b>\nbuilder_id: {}\nerror:\n{}",
//!     html::code("beaverbuild <prod>"),
//!     html::pre("invalid <root> & upstream rejected payload"),
//! );
//! ```

/// Telegram's documented `sendMessage.text` character limit.
///
/// The Bot API applies this limit after entity parsing. The estimate helpers in
/// this crate count raw Rust `char`s and should be treated as guardrails.
pub const SEND_MESSAGE_TEXT_MAX_CHARS: usize = 4096;

/// Count Unicode scalar values in a string.
///
/// This is the same unit as `str::chars().count()`, not bytes and not
/// Telegram's post-entity parsing count.
pub fn raw_char_count(input: &str) -> usize {
    input.chars().count()
}

/// Estimate whether a complete message fits Telegram's `sendMessage.text`
/// length limit.
///
/// This checks `input.chars().count() <= 4096`. It is only an estimate for
/// messages sent with `parse_mode`, because Telegram applies the limit after
/// entity parsing. Use this as a guardrail, not proof that Telegram will accept
/// the message.
pub fn fits_send_message_text_limit_estimate(input: &str) -> bool {
    raw_char_count(input) <= SEND_MESSAGE_TEXT_MAX_CHARS
}

/// Return text unchanged for messages sent without `parse_mode`.
pub fn plain_text(input: &str) -> String {
    input.to_string()
}

/// Helpers for messages sent with `parse_mode = "MarkdownV2"`.
///
/// These helpers are for dynamic fragments only. Static MarkdownV2 markup in
/// the surrounding message remains the caller's responsibility.
pub mod markdown_v2 {
    /// Escape dynamic text for normal MarkdownV2 text context.
    ///
    /// The returned string does not include formatting delimiters.
    pub fn text(input: &str) -> String {
        escape_markdown_v2_text(input)
    }

    /// Return a complete MarkdownV2 inline-code fragment.
    ///
    /// The returned string includes surrounding backticks.
    pub fn inline_code(input: &str) -> String {
        format!("`{}`", escape_markdown_v2_code(input))
    }

    /// Return a complete MarkdownV2 fenced code block.
    ///
    /// The returned string includes opening and closing fences.
    pub fn code_block(input: &str) -> String {
        format!("```\n{}\n```", escape_markdown_v2_code(input))
    }

    fn escape_markdown_v2_text(input: &str) -> String {
        let mut escaped = String::with_capacity(input.len());

        for c in input.chars() {
            if matches!(
                c,
                '\\' | '_'
                    | '*'
                    | '['
                    | ']'
                    | '('
                    | ')'
                    | '~'
                    | '`'
                    | '>'
                    | '#'
                    | '+'
                    | '-'
                    | '='
                    | '|'
                    | '{'
                    | '}'
                    | '.'
                    | '!'
            ) {
                escaped.push('\\');
            }

            escaped.push(c);
        }

        escaped
    }

    fn escape_markdown_v2_code(input: &str) -> String {
        let mut escaped = String::with_capacity(input.len());

        for c in input.chars() {
            if matches!(c, '`' | '\\') {
                escaped.push('\\');
            }

            escaped.push(c);
        }

        escaped
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn text_escapes_all_markdown_v2_special_characters() {
            let input = "_*[]()~`>#+-=|{}.!\\";
            let expected = "\\_\\*\\[\\]\\(\\)\\~\\`\\>\\#\\+\\-\\=\\|\\{\\}\\.\\!\\\\";

            assert_eq!(text(input), expected);
        }

        #[test]
        fn text_preserves_normal_whitespace() {
            assert_eq!(text("hello\nworld\tagain"), "hello\nworld\tagain");
        }

        #[test]
        fn inline_code_wraps_and_escapes_backticks_and_backslashes() {
            assert_eq!(
                inline_code(r#"a `tick` and \ slash"#),
                r#"`a \`tick\` and \\ slash`"#
            );
        }

        #[test]
        fn code_block_wraps_and_escapes_backticks_and_backslashes() {
            assert_eq!(
                code_block("line 1\n```rust\n\\"),
                "```\nline 1\n\\`\\`\\`rust\n\\\\\n```"
            );
        }
    }
}

/// Helpers for messages sent with `parse_mode = "HTML"`.
///
/// These helpers are for dynamic fragments only. Static Telegram HTML markup in
/// the surrounding message remains the caller's responsibility.
pub mod html {
    /// Escape dynamic text for an HTML text context.
    ///
    /// The returned string does not include formatting tags.
    pub fn text(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    /// Return a complete Telegram HTML `<code>` fragment.
    pub fn code(input: &str) -> String {
        format!("<code>{}</code>", text(input))
    }

    /// Return a complete Telegram HTML `<pre>` fragment.
    pub fn pre(input: &str) -> String {
        format!("<pre>{}</pre>", text(input))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn text_escapes_html_entities() {
            assert_eq!(text("a & b < c > d"), "a &amp; b &lt; c &gt; d");
        }

        #[test]
        fn code_returns_complete_fragment() {
            assert_eq!(code("a & b"), "<code>a &amp; b</code>");
        }

        #[test]
        fn pre_returns_complete_fragment() {
            assert_eq!(pre("a < b"), "<pre>a &lt; b</pre>");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_preserves_input() {
        assert_eq!(plain_text("hello _* <world>"), "hello _* <world>");
    }

    #[test]
    fn raw_char_count_counts_unicode_scalars_not_bytes() {
        assert_eq!(raw_char_count("aé🦀"), 3);
        assert_eq!("aé🦀".len(), 7);
    }

    #[test]
    fn estimate_accepts_boundary() {
        let message = "x".repeat(SEND_MESSAGE_TEXT_MAX_CHARS);

        assert!(fits_send_message_text_limit_estimate(&message));
    }

    #[test]
    fn estimate_rejects_over_boundary() {
        let message = "x".repeat(SEND_MESSAGE_TEXT_MAX_CHARS + 1);

        assert!(!fits_send_message_text_limit_estimate(&message));
    }
}
