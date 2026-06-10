use telegram_sanitize::{fits_send_message_text_limit_estimate, plain_text};

fn main() {
    let label = plain_text("rbx-prod-mainnet");
    let raw_error = "db\nunavailable\rwith\tcontrols";
    let error = raw_error.replace(char::is_control, " ");
    let error_summary: String = error.chars().take(512).collect();

    let message = format!(
        "cielago alert: relay submissions disabled\n\
         label: {label}\n\
         aggregation_slot: 12345\n\
         reason: bundle tracer failed to flush records to postgres\n\
         error: {}\n\
         relay submissions disabled until process restart",
        plain_text(&error_summary)
    );

    assert!(fits_send_message_text_limit_estimate(&message));
    println!("{message}");
}
