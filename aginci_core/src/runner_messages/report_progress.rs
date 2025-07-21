use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "report_type", rename_all = "lowercase")]
pub enum ProgressReport {
    Output(ProgressReportOutput),
    Exit(ProgressReportExit),
    Step(ProgressReportStep),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
    Stderr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressReportOutput {
    pub output_type: OutputType,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressReportExit {
    pub exit_code: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressReportStep {
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderedReport {
    pub ord: u64,
    pub body: ProgressReport,
}
