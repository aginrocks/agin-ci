use indicatif::ProgressStyle;
use inquire::ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use owo_colors::OwoColorize;

pub fn make_link(text: &str, url: &str) -> String {
    let visible_text = text.replace(' ', "\u{00A0}");

    format!(
        "\x1b]8;;{url}\x1b\\{}\x1b]8;;\x1b\\",
        visible_text.bright_blue().underline().bold()
    )
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        use tracing::{Level, event};
        event!(target: "success", Level::INFO, $($arg)*);
    };
}

pub fn get_render_config() -> RenderConfig<'static> {
    let highlight_color = Color::DarkGreen;

    let grey = Color::Rgb {
        r: 100,
        g: 100,
        b: 100,
    };

    let mut render_config = RenderConfig::default();
    render_config.prompt = StyleSheet::new().with_attr(Attributes::BOLD);
    render_config.prompt_prefix = Styled::new("→").with_fg(Color::LightBlue);
    render_config.answered_prompt_prefix = Styled::new("").with_fg(Color::LightGreen);
    render_config.placeholder = StyleSheet::new().with_fg(grey);
    render_config.selected_option = Some(StyleSheet::new().with_fg(highlight_color));
    render_config.highlighted_option_prefix = Styled::new("→").with_fg(highlight_color);
    render_config.selected_checkbox = Styled::new("☑").with_fg(highlight_color);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");
    render_config.option_index_prefix = IndexPrefix::Simple;
    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("✖").with_fg(Color::LightRed));
    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(highlight_color);

    render_config.help_message = StyleSheet::new()
        .with_fg(grey)
        .with_attr(Attributes::ITALIC);

    render_config
}

pub fn get_spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim}{spinner:.bold.blue} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
}

pub fn colored_exit_code(content: &str, status: i32) -> String {
    if status == 0 {
        content.green().bold().to_string()
    } else {
        content.red().bold().to_string()
    }
}
