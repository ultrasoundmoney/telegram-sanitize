use telegram_sanitize::{fits_send_message_text_limit_estimate, markdown_v2};

fn main() {
    let builder_id = "beaver_build[prod]";
    let network = "mainnet";

    let message = format!(
        "*builder demoted*\n\
         builder\\_id: {}\n\
         network: {}\n\
         reason: {}",
        markdown_v2::inline_code(builder_id),
        markdown_v2::inline_code(network),
        markdown_v2::text("simulation failed: invalid merkle root"),
    );

    assert!(fits_send_message_text_limit_estimate(&message));
    println!("{message}");
}
