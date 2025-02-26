use anyhow::{anyhow, Result};
use sqlx::{prelude::FromRow, query, query_as};
use tabby_db_macros::query_paged_as;

use crate::{DbConn, SQLXResultExt};

#[derive(FromRow)]
pub struct GithubRepositoryProviderDAO {
    pub id: i64,
    pub display_name: String,
    pub application_id: String,
    pub secret: String,
    pub access_token: Option<String>,
}

#[derive(FromRow)]
pub struct GithubProvidedRepositoryDAO {
    pub id: i64,
    pub vendor_id: String,
    pub github_repository_provider_id: i64,
    pub name: String,
    pub git_url: String,
    pub active: bool,
}

impl DbConn {
    pub async fn create_github_provider(
        &self,
        name: String,
        application_id: String,
        secret: String,
    ) -> Result<i64> {
        let res = query!("INSERT INTO github_repository_provider (display_name, application_id, secret) VALUES ($1, $2, $3);",
            name,
            application_id,
            secret
        ).execute(&self.pool).await.unique_error("GitHub Application ID already exists")?;
        Ok(res.last_insert_rowid())
    }

    pub async fn get_github_provider(&self, id: i64) -> Result<GithubRepositoryProviderDAO> {
        let provider = query_as!(
            GithubRepositoryProviderDAO,
            "SELECT id, display_name, application_id, secret, access_token FROM github_repository_provider WHERE id = ?;",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(provider)
    }

    pub async fn delete_github_provider(&self, id: i64) -> Result<()> {
        let res = query!("DELETE FROM github_repository_provider WHERE id = ?;", id)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() != 1 {
            return Err(anyhow!("No github provider details to delete"));
        }
        Ok(())
    }

    pub async fn update_github_provider_access_token(
        &self,
        id: i64,
        access_token: String,
    ) -> Result<()> {
        let res = query!(
            "UPDATE github_repository_provider SET access_token = ? WHERE id = ?",
            access_token,
            id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() != 1 {
            return Err(anyhow!(
                "The specified Github repository provider does not exist"
            ));
        }

        Ok(())
    }

    pub async fn update_github_provider(
        &self,
        id: i64,
        display_name: String,
        application_id: String,
        secret: Option<String>,
    ) -> Result<()> {
        let secret = match secret {
            Some(secret) => secret,
            None => self.get_github_provider(id).await?.secret,
        };

        let res = query!(
            "UPDATE github_repository_provider SET display_name = ?, application_id = ?, secret = ? WHERE id = ?;",
            display_name,
            application_id,
            secret,
            id
        )
        .execute(&self.pool)
        .await
        .unique_error("A provider with that application ID already exists")?;

        if res.rows_affected() != 1 {
            return Err(anyhow!("Provider does not exist"));
        }

        Ok(())
    }

    pub async fn list_github_repository_providers(
        &self,
        ids: Vec<i64>,
        limit: Option<usize>,
        skip_id: Option<i32>,
        backwards: bool,
    ) -> Result<Vec<GithubRepositoryProviderDAO>> {
        let condition = (!ids.is_empty()).then(|| {
            let ids = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("id in ({ids})")
        });
        let providers = query_paged_as!(
            GithubRepositoryProviderDAO,
            "github_repository_provider",
            [
                "id",
                "display_name",
                "application_id",
                "secret",
                "access_token"
            ],
            limit,
            skip_id,
            backwards,
            condition
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(providers)
    }

    pub async fn create_github_provided_repository(
        &self,
        github_provider_id: i64,
        vendor_id: String,
        name: String,
        git_url: String,
    ) -> Result<i64> {
        let res = query!("INSERT INTO github_provided_repositories (github_repository_provider_id, vendor_id, name, git_url) VALUES (?, ?, ?, ?)",
            github_provider_id, vendor_id, name, git_url).execute(&self.pool).await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn delete_github_provided_repository(&self, id: i64) -> Result<()> {
        let res = query!("DELETE FROM github_provided_repositories WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if res.rows_affected() != 1 {
            return Err(anyhow!("Repository not found"));
        }
        Ok(())
    }

    pub async fn list_github_provided_repositories(
        &self,
        provider_ids: Vec<i64>,
        limit: Option<usize>,
        skip_id: Option<i32>,
        backwards: bool,
    ) -> Result<Vec<GithubProvidedRepositoryDAO>> {
        let provider_ids = provider_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let repos = query_paged_as!(
            GithubProvidedRepositoryDAO,
            "github_provided_repositories",
            [
                "id",
                "vendor_id",
                "name",
                "git_url",
                "active",
                "github_repository_provider_id"
            ],
            limit,
            skip_id,
            backwards,
            (!provider_ids.is_empty())
                .then(|| format!("github_repository_provider_id IN ({provider_ids})"))
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(repos)
    }

    pub async fn update_github_provided_repository_active(
        &self,
        id: i64,
        active: bool,
    ) -> Result<()> {
        let not_active = !active;
        let res = query!(
            "UPDATE github_provided_repositories SET active = ? WHERE id = ? AND active = ?",
            active,
            id,
            not_active
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() != 1 {
            return Err(anyhow!("Repository active status was not changed"));
        }

        Ok(())
    }
}
