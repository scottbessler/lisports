use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub async fn ensure_dir(&self) -> Result<(), AppError> {
        tokio::fs::create_dir_all(&self.root)
            .await
            .map_err(AppError::cache)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, AppError> {
        let path = self.path_for(key)?;
        match tokio::fs::read_to_string(&path).await {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(value) => Ok(Some(value)),
                Err(err) => {
                    tracing::warn!(
                        cache_path = %path.display(),
                        error = %err,
                        "ignoring stale or invalid cache file"
                    );
                    let _ = tokio::fs::remove_file(&path).await;
                    Ok(None)
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(AppError::cache(err)),
        }
    }

    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T) -> Result<(), AppError> {
        self.ensure_dir().await?;
        let path = self.path_for(key)?;
        let tmp_path = temp_path(&path);
        let bytes = serde_json::to_vec(value).map_err(AppError::parse)?;
        tokio::fs::write(&tmp_path, bytes)
            .await
            .map_err(AppError::cache)?;
        tokio::fs::rename(&tmp_path, &path)
            .await
            .map_err(AppError::cache)?;
        Ok(())
    }

    fn path_for(&self, key: &str) -> Result<PathBuf, AppError> {
        Ok(self.root.join(format!("{}.json", sanitize_cache_key(key)?)))
    }
}

pub fn sanitize_cache_key(key: &str) -> Result<String, AppError> {
    if key.is_empty() {
        return Err(AppError::BadRequest(
            "cache key cannot be empty".to_string(),
        ));
    }
    if key
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, ':' | '-' | '_'))
    {
        Ok(key.to_string())
    } else {
        Err(AppError::BadRequest(
            "cache key contains unsafe characters".to_string(),
        ))
    }
}

fn temp_path(path: &Path) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    path.with_extension(format!("json.tmp-{nanos}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_allows_existing_key_shape() {
        assert_eq!(
            sanitize_cache_key("day:2026-04-26").unwrap(),
            "day:2026-04-26"
        );
    }

    #[test]
    fn cache_key_rejects_path_segments() {
        assert!(sanitize_cache_key("../secret").is_err());
        assert!(sanitize_cache_key("day/2026-04-26").is_err());
    }

    #[tokio::test]
    async fn invalid_cache_json_is_treated_as_miss() {
        let root = std::env::temp_dir().join(format!(
            "lisports-cache-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let cache = Cache::new(&root);
        cache.ensure_dir().await.unwrap();
        tokio::fs::write(root.join("day:2026-04-26.json"), r#"{"scoreboard":{}}"#)
            .await
            .unwrap();

        let result = cache
            .get_json::<crate::models::Scoreboard>("day:2026-04-26")
            .await
            .unwrap();

        assert!(result.is_none());
        assert!(!root.join("day:2026-04-26.json").exists());
        let _ = tokio::fs::remove_dir_all(root).await;
    }
}
