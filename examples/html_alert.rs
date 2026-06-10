use telegram_sanitize::{fits_send_message_text_limit_estimate, html};

fn main() {
    let builder_id = "beaverbuild <prod>";
    let error = "simulation failed: invalid <root> & upstream said no";

    let message = format!(
        "<b>builder demoted</b>\n\
         builder_id: {}\n\
         error:\n{}",
        html::code(builder_id),
        html::pre(error),
    );

    assert!(fits_send_message_text_limit_estimate(&message));
    println!("{message}");
}
