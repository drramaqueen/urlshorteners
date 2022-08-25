// use std::collections::BTreeMap;
// use std::str::FromStr;
// use std::collections::HashMap;
use async_trait::async_trait;
use url::Url;

use crate::error::Error;
use crate::error::Result;

#[async_trait]
pub trait BaseShortener {
    async fn get(&self, url: &str) -> Result<reqwest::Response> {
        match reqwest::get(url).await {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e);
                Err(Error::ResponseError(e.to_string()))
            }
        }
    }

    async fn post(&self, url: &str, data: Vec<u8>) -> Result<()> {
        let client = reqwest::Client::new();
        match client.post(url).body(data).send().await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(Error::ResponseError(e.to_string()))
            }
        }
    }

    async fn expand(&self, url: &str) -> Result<Url> {
        let response = self.get(url).await;
        match response {
            Ok(r) => Ok(r.url().to_owned()),
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }

    fn clean_url(&self, url: Vec<u8>) -> Result<Vec<u8>> {
        let str_from_url = match std::str::from_utf8(&url) {
            Ok(s) => s,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };
        let new_url: Vec<u8>;
        if !url.starts_with(b"http://") && !url.starts_with(b"https://") {
            new_url = ("http://".to_string() + str_from_url).as_bytes().to_vec();
        } else {
            new_url = str_from_url.as_bytes().to_vec();
        }
        let new_url = match Url::parse(std::str::from_utf8(&new_url).unwrap()) {
            Ok(_) => new_url,
            Err(e) => return Err(Error::BadUrl(e.to_string())),
        };

        Ok(new_url)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base::BaseShortener,
        bitly::{self},
    };
    use std::collections::BTreeMap;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_expand() {
        let b = bitly::Shortener {
            timeout: 2,
            verify: true,
            proxies: BTreeMap::new(),
        };
        let url = b.expand("https://bit.ly/TEST").await.unwrap();
        assert_eq!(url.as_str(), "https://www.autosouk.com/News/templates/CarReviewPricePhotoArticlesDubai.Aspx?articleid=50&zoneid=3");
    }
}
