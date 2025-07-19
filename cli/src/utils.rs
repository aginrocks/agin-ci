use owo_colors::OwoColorize;

pub fn make_link(text: &str, url: &str) -> String {
    let visible_text = text.replace(' ', "\u{00A0}");

    format!(
        "\x1b]8;;{url}\x1b\\{}\x1b]8;;\x1b\\",
        visible_text.bright_blue().underline().bold()
    )
}

// pub fn
