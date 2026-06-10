# telegram-sanitize

Pure, dependency-free string sanitation helpers for Telegram Bot API messages.

This crate does not send messages, build JSON payloads, format whole alerts, or
truncate fragments. It only turns dynamic strings into fragments that are safe to
insert into a Telegram message for the parse mode you selected.

## Which Helper Should I Use?

- Use `plain_text` when omitting `parse_mode`. This is the safest choice for
  operational alerts that do not need rich formatting.
- Use `markdown_v2::*` only when sending `parse_mode = "MarkdownV2"`.
- Use `html::*` only when sending `parse_mode = "HTML"`.
- Prefer plain text for critical alerts unless formatting materially improves
  readability.
- Normalize or truncate caller-specific fields before sanitation when desired.

## Examples

Plain text alert:

```rust
use telegram_sanitize::{fits_send_message_text_limit_estimate, plain_text};

let label = plain_text("rbx-prod-mainnet");
let error = "db\nunavailable".replace(char::is_control, " ");

let message = format!(
    "cielago alert: relay submissions disabled\n\
     label: {label}\n\
     error: {}",
    plain_text(&error),
);

assert!(fits_send_message_text_limit_estimate(&message));
```

MarkdownV2 error block:

```rust
use telegram_sanitize::markdown_v2;

let message = format!(
    "*builder demoted*\nslot: {}\nerror:\n{}",
    markdown_v2::inline_code("12345"),
    markdown_v2::code_block("simulation failed: invalid merkle root"),
);
```

HTML fragments:

```rust
use telegram_sanitize::html;

let message = format!(
    "<b>builder demoted</b>\nbuilder_id: {}\nerror:\n{}",
    html::code("beaverbuild"),
    html::pre("simulation failed: invalid <root>"),
);
```
