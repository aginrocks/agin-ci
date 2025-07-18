use async_trait::async_trait;

pub struct GitProviderCreateOptions {
    pub token: String,
    pub base_url: Option<String>,
}

#[async_trait]
pub trait GitProvider {
    fn new(options: GitProviderCreateOptions) -> Self;
}
