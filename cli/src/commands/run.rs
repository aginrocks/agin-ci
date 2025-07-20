use miette::Result;

use crate::{Cli, SelectOrgArgs, SelectProjectArgs};

pub async fn handle_run(
    cli: &Cli,
    workflow: String,
    org: SelectOrgArgs,
    project: SelectProjectArgs,
    local: bool,
) -> Result<()> {
    Ok(())
}
