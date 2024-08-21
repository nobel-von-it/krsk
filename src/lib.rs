use serde::{Deserialize, Serialize};
use serde_json::Value;

const API_URL: &str = "https://crates.io/api/v1/crates";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crateio {
    #[serde(rename = "crate")]
    crate_data: CrateData,
    versions: Vec<Value>,
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
    pub async fn new_features(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        if let Some(version) = self.versions.first() {
            let version: Version = serde_json::from_value(version.clone())?;

            if let Some(features) = version.features {
                return Ok(Some(
                    features.as_object().unwrap().keys().cloned().collect(),
                ));
            }
        }

        Ok(None)
    }
    pub async fn features_by_version(
        &self,
        id: u64,
    ) -> Result<Option<Vec<String>>, serde_json::Error> {
        if let Some(features) = self.versions.get(id as usize - 1) {
            let features: Version = serde_json::from_value(features.clone())?;
            if let Some(features) = features.features {
                return Ok(Some(
                    features.as_object().unwrap().keys().cloned().collect(),
                ));
            }
        }
        Ok(None)
    }
    pub async fn get_data(&self) -> &CrateData {
        &self.crate_data
    }
    pub async fn get_all_versions(&self) -> Result<Vec<Version>, serde_json::Error> {
        self.versions
            .iter()
            .map(|v| serde_json::from_value(v.clone()))
            .collect::<Result<Vec<Version>, serde_json::Error>>()
    }
    pub async fn get_version(&self, id: u64) -> Result<Option<Version>, serde_json::Error> {
        if let Some(version) = self.versions.get(id as usize - 1) {
            return Ok(Some(serde_json::from_value(version.clone())?));
        }
        Ok(None)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateData {
    pub name: String,
    pub categories: Option<Vec<Value>>,
    pub description: String,
    pub repository: Option<String>,
    pub downloads: Option<u64>,
    pub max_version: Option<String>,
    pub max_stable_version: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub checksum: String,
    pub crate_size: Option<u64>,
    pub created_at: String,
    pub num: String,
    pub rust_version: Option<String>,
    pub license: Option<String>,
    pub features: Option<Value>,
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
    async fn test_new_features() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let features = res.new_features().await;
        // println!("{:?}", features);
        assert!(features.is_ok())
    }

    #[tokio::test]
    async fn test_features_by_version() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let features = res.features_by_version(10).await;
        println!("{:?}", features);
        assert!(features.is_ok());
        let features = res.features_by_version(10000000).await;
        assert!(features.is_ok() && features.unwrap().is_none());
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
        println!(
            "{:?}",
            versions
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.num.clone())
                .collect::<Vec<String>>()
        );
        assert!(versions.is_ok())
    }

    #[tokio::test]
    async fn test_get_version() {
        let res = super::Crateio::get("tokio").await.unwrap();
        let version = res.get_version(10).await;
        println!("{:?}", version);
        assert!(version.is_ok() && version.unwrap().is_some());
        let version = res.get_version(10000).await;
        println!("{:?}", version);
        assert!(version.is_ok() && version.unwrap().is_none());
    }
}
