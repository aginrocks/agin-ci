use owo_colors::OwoColorize;
use std::fmt;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{
        FmtContext,
        format::{FormatEvent, FormatFields, Writer},
    },
    registry::LookupSpan,
};

pub struct EventFormatter;

pub fn in_scope<S, N>(ctx: &FmtContext<'_, S, N>, target: &str) -> bool
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    if let Some(scope) = ctx.event_scope() {
        let in_target_span = scope.from_root().any(|span| span.name() == target);

        return in_target_span;
    }
    false
}

impl<S, N> FormatEvent<S, N> for EventFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let has_ansi = writer.has_ansi_escapes();
        let target = event.metadata().target();
        let level = event.metadata().level();

        let prefix = match target {
            "success" => {
                if has_ansi {
                    "success:".green().bold().to_string()
                } else {
                    "success:".to_string()
                }
            }
            _ => match *level {
                Level::WARN => {
                    if has_ansi {
                        "warning:".yellow().bold().to_string()
                    } else {
                        "warning:".to_string()
                    }
                }
                Level::ERROR => {
                    if has_ansi {
                        "error:".red().bold().to_string()
                    } else {
                        "error:".to_string()
                    }
                }
                Level::INFO => {
                    if has_ansi {
                        "→".bright_blue().bold().to_string()
                    } else {
                        "→".to_string()
                    }
                }
                _ => {
                    if has_ansi {
                        format!("{}", format!("{level}:").dimmed().bold())
                    } else {
                        format!("{level}:")
                    }
                }
            },
        };

        if in_scope(ctx, "aginci_cli::nested") {
            write!(writer, "  ")?;
        }

        if in_scope(ctx, "aginci_cli::nested::nested") {
            write!(writer, "  ")?;
        }

        write!(writer, "{prefix} ")?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
