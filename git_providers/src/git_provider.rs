use async_trait::async_trait;
use color_eyre::eyre::Result;
use octocrab::models::repos::Content;

pub struct GitProviderCreateOptions {
    pub token: String,
    pub base_url: Option<String>,
}

#[async_trait]
pub trait GitProvider {
    fn new(options: GitProviderCreateOptions) -> Result<Self>
    where
        Self: Sized;
    async fn get_folder_contents(
        &self,
        owner: String,
        repo: String,
        path: String,
        r#ref: String,
    ) -> Result<Vec<Content>>;
}
