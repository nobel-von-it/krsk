#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

const API_URL: &str = "https://crates.io/api/v1/crates";

#[derive(Deserialize, Serialize, Debug)]
struct Crateio {
    #[serde(rename = "crate")]
    pub crate_data: CrateData,
    pub keywords: Option<Vec<Value>>,
    pub versions: Vec<Value>,
}
impl Crateio {
    pub async fn get(name: &str) -> Result<Self, reqwest::Error> {
        let url = format!("{API_URL}/{name}");
        let res = reqwest::Client::new()
            .get(url)
            .header("User-Agent", "krsk/0.1.0 ya@example.com")
            .send()
            .await?;

        res.json::<crate::Crateio>().await
    }
    pub async fn get_last_features(&self) -> Option<Vec<String>> {
        self.get_features(0).await
    }
    pub async fn get_features(&self, id: u64) -> Option<Vec<String>> {
        if let Some(version) = self.get_version(id).await {
            return version.get_features().await;
        }
        None
    }
    pub async fn get_data(&self) -> &CrateData {
        &self.crate_data
    }
    pub async fn get_all_versions(&self) -> Vec<Version> {
        let mut vs = vec![];
        for v in self.versions.iter() {
            if let Ok(v) = serde_json::from_value::<Version>(v.clone()) {
                vs.push(v)
            }
        }
        vs
    }
    pub async fn get_version(&self, id: u64) -> Option<Version> {
        if let Some(value) = self.versions.get(id as usize) {
            if let Ok(version) = serde_json::from_value(value.clone()) {
                return Some(version);
            }
        }
        None
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateData {
    categories: Option<Vec<Value>>,
    pub created_at: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub downloads: u64,
    pub exact_match: bool,
    pub homepage: Option<String>,
    pub max_stable_version: Option<String>,
    pub max_version: Option<String>,
    pub name: String,
    pub recent_downloads: u64,
    pub repository: Option<String>,
    pub updated_at: String,
}
impl CrateData {
    pub async fn get_categories(&self) -> Option<Vec<String>> {
        self.categories.as_ref().map(|categories| {
            categories
                .iter()
                .map(|c| serde_json::from_value(c.clone()).unwrap_or(String::new()))
                .collect()
        })
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub checksum: String,
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub crate_size: Option<u64>,
    pub created_at: String,
    pub downloads: u64,
    features: Option<Value>,
    pub has_lib: bool,
    pub id: u64,
    pub license: Option<String>,
    pub num: String,
    pub published_by: Author,
    pub rust_version: Option<String>,
    pub updated_at: String,
    pub yanked: bool,
}
impl Version {
    pub async fn get_features(&self) -> Option<Vec<String>> {
        if let Some(features) = &self.features {
            return Some(features.as_object().unwrap().keys().cloned().collect());
        }
        None
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Author {
    pub avatar: Option<String>,
    pub id: u64,
    pub login: String,
    pub name: String,
    pub url: String,
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get() {
        let res = super::Crateio::get("tokio").await;
        assert!(res.is_ok());
        let res = super::Crateio::get("aslkdjflsdkjflskdjflskj").await;
        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_get_last_features() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let features = res.get_last_features().await;
        println!("{:?}", features);
        assert!(features.is_some())
    }

    #[tokio::test]
    async fn test_get_features() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let features = res.get_features(10).await;
        println!("{:?}", features);
        assert!(features.is_some());
        let features = res.get_features(0).await;
        println!("{:?}", features);
        assert!(features.is_some());
        let features = res.get_features(10000000).await;
        assert!(features.is_none());
    }

    #[tokio::test]
    async fn test_get_data() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let data = res.get_data().await;
        println!("{:?}", data);
    }

    #[tokio::test]
    async fn test_get_all_versions() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let versions = res.get_all_versions().await;
        assert!(!versions.is_empty());
    }

    #[tokio::test]
    async fn test_get_version() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let version = res.get_version(10).await;
        println!("{:?}", version);
        assert!(version.is_some());
        let version = res.get_version(1).await;
        println!("{:?}", version);
        assert!(version.is_some());
        let version = res.get_version(10000).await;
        println!("{:?}", version);
        assert!(version.is_none());
    }

    #[tokio::test]
    async fn test_get_categories() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let cs = res.get_data().await.get_categories().await;
        assert!(cs.clone().is_some());
        assert!(cs.unwrap().len() == 2)
    }
}
