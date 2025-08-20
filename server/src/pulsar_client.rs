use std::sync::Arc;

use color_eyre::eyre::{Context, Result};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use pulsar::{Authentication, Pulsar, TokioExecutor};
use pulsar_admin_sdk::{
    apis::{
        Error,
        configuration::Configuration,
        namespaces_api::{
            NamespacesCreateNamespaceError, NamespacesGrantPermissionOnNamespaceError,
            namespaces_create_namespace, namespaces_grant_permission_on_namespace,
        },
        tenants_api::{tenants_base_create_tenant, tenants_base_get_tenants},
    },
    models::{Policies, TenantInfo},
};
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

pub async fn init_pulsar(
    settings: &Settings,
) -> Result<(Arc<Pulsar<TokioExecutor>>, Arc<PulsarAdmin>)> {
    let mut builder = Pulsar::builder(&settings.pulsar.connection_string, TokioExecutor);

    let pulsar_admin = PulsarAdmin::from_key(
        &settings.pulsar.admin_url,
        &settings.pulsar.secret_key,
        &settings.pulsar.tenant,
    )?;

    pulsar_admin.ensure_tenant_exists().await?;

    let authentication = Authentication {
        name: "token".to_string(),
        data: pulsar_admin.token.clone().into_bytes(),
    };

    builder = builder.with_auth(authentication);
    let pulsar = builder.build().await?;

    Ok((Arc::new(pulsar), pulsar_admin))
}

pub struct PulsarAdmin {
    pub tenant: String,
    pub token: String,
    pub config: Configuration,
    pub key: PulsarSecretKey,
}

pub struct PulsarSecretKey {
    pub key: EncodingKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PulsarTokenClaims {
    pub sub: String,
}

impl PulsarSecretKey {
    pub fn new(key: EncodingKey) -> Self {
        Self { key }
    }

    pub fn sign_token(&self, sub: String) -> Result<String> {
        let claims = PulsarTokenClaims { sub };
        let token = encode(&Header::new(Algorithm::RS256), &claims, &self.key)?;
        Ok(token)
    }
}

/// Custom wrapper around Pulsar Admin SDK, for the needs of Agin CI.
impl PulsarAdmin {
    pub fn from_key(base_url: &str, key: &str, tenant: &str) -> Result<Arc<Self>> {
        let key = PulsarSecretKey::new(EncodingKey::from_rsa_pem(key.as_bytes())?);

        let token = key.sign_token("admin".to_string())?;

        let config = PulsarAdmin::generate_config(base_url, &token);

        Ok(PulsarAdmin::new(tenant.to_string(), token, config, key))
    }

    fn generate_config(base_url: &str, token: &str) -> Configuration {
        Configuration {
            base_path: base_url.to_string(),
            user_agent: Some(format!("aginci-cli/{}", env!("CARGO_PKG_VERSION"))),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: Some(token.to_string()),
            api_key: None,
            ..Default::default()
        }
    }

    pub fn new(
        tenant: String,
        token: String,
        config: Configuration,
        key: PulsarSecretKey,
    ) -> Arc<Self> {
        Arc::new(Self {
            tenant,
            token,
            config,
            key,
        })
    }

    pub async fn create_namespace(
        &self,
        namespace: &str,
        body: Option<Policies>,
    ) -> Result<(), Error<NamespacesCreateNamespaceError>> {
        namespaces_create_namespace(&self.config, &self.tenant, namespace, body).await
    }

    pub async fn grant_permissions_on_namespace(
        &self,
        namespace: &str,
        role: &str,
        body: Option<Vec<String>>,
    ) -> Result<(), Error<NamespacesGrantPermissionOnNamespaceError>> {
        namespaces_grant_permission_on_namespace(&self.config, &self.tenant, namespace, role, body)
            .await
    }

    pub async fn ensure_tenant_exists(&self) -> Result<()> {
        let tenants = tenants_base_get_tenants(&self.config)
            .await
            .wrap_err("Failed to get tenants")?;

        if tenants.contains(&self.tenant) {
            return Ok(());
        }

        let body = Some(TenantInfo {
            admin_roles: Some(vec!["admin".to_string()]),
            allowed_clusters: Some(vec!["standalone".to_string()]),
        });

        tenants_base_create_tenant(&self.config, &self.tenant, body)
            .await
            .wrap_err("Failed to create tenant")
    }
}
