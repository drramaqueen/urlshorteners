use crate::base::BaseShortener;
use crate::error::Error;
use crate::error::Result;
use async_trait::async_trait;
use std::collections::BTreeMap;
use url::Url;

pub struct Shortener {
    pub timeout: u32,
    pub verify: bool,
    pub proxies: BTreeMap<String, String>,
}

#[async_trait]
impl BaseShortener for Shortener {
    async fn expand(&self, url: &str) -> Result<Url> {
        let response = self.get(url).await;
        match response {
            Ok(r) => Ok(r.url().to_owned()),
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }
}
