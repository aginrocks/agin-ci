use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "report_type", rename_all = "lowercase")]
pub enum ProgressReport {
    Output(ProgressReportOutput),
    Exit(ProgressReportExit),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
    Stderr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressReportOutput {
    pub output_type: OutputType,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressReportExit {
    pub exit_code: i32,
}
