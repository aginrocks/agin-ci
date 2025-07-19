use std::fmt::Formatter;

use miette::Diagnostic;
use owo_colors::OwoColorize;

use crate::utils::make_link;

pub struct ErrorReportHandler;

impl ErrorReportHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ErrorReportHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl miette::ReportHandler for ErrorReportHandler {
    fn debug(&self, error: &dyn Diagnostic, f: &mut Formatter<'_>) -> core::fmt::Result {
        let code = error.code().unwrap_or_else(|| Box::new("unknown"));

        let report_url = error.url().map(|url| url.to_string());

        writeln!(f, "{} {error}", "error:".red().bold())?;

        write!(
            f,
            "{} {}",
            "  code:".dimmed(),
            match report_url {
                Some(url) => {
                    format!(
                        "{} {}",
                        make_link(&code.to_string(), &url),
                        "(control-click for more info)".dimmed()
                    )
                }
                None => code.bright_blue().to_string(),
            }
        )?;

        if let Some(help) = error.help() {
            write!(f, "\n  {} {help}", "help:".cyan())?;
        }

        Ok(())
    }
}
