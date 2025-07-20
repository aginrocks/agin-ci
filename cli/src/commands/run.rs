use aginci_core::workflow::read_workflow_by_name;
use miette::{Result, miette};

use crate::{
    Cli, SelectOrgArgs, SelectProjectArgs, config::init_config, errors::LocalOrgProjectSpecified,
};

pub async fn handle_run(
    cli: &Cli,
    workflow: String,
    org: SelectOrgArgs,
    project: SelectProjectArgs,
    local: bool,
) -> Result<()> {
    match local {
        true => {
            if org.org.is_some() || project.project.is_some() {
                return Err(LocalOrgProjectSpecified.into());
            }

            // TODO: Add better workflow error handling
            let workflow = read_workflow_by_name(workflow)
                .await
                .map_err(|_| miette!("Failed to read workflow"))?;
        }
        false => {
            let _config = init_config().await?;
        }
    }

    Ok(())
}
