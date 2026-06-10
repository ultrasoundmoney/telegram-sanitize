use telegram_sanitize::{SEND_MESSAGE_TEXT_MAX_CHARS, fits_send_message_text_limit_estimate};

fn main() {
    let message = "x".repeat(SEND_MESSAGE_TEXT_MAX_CHARS);

    assert!(fits_send_message_text_limit_estimate(&message));
}
