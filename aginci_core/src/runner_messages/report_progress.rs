use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "report_type", rename_all = "lowercase")]
pub enum ProgressReport {
    Output(ProgressReportOutput),
    Exit(PorgressReportExit),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
    Stderr,
}

#[derive(Serialize, Deserialize)]
pub struct ProgressReportOutput {
    pub output_type: OutputType,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct PorgressReportExit {
    pub exit_code: i32,
}
