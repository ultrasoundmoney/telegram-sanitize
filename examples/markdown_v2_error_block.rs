use telegram_sanitize::{fits_send_message_text_limit_estimate, markdown_v2};

fn main() {
    let error = "simulation failed:\ninvalid `merkle` root from \\ upstream";
    let message = format!(
        "*builder demoted*\nslot: {}\nerror:\n{}",
        markdown_v2::inline_code("12345"),
        markdown_v2::code_block(error),
    );

    assert!(fits_send_message_text_limit_estimate(&message));
    println!("{message}");
}
