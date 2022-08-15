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
        let url = self.clean_url(url.as_bytes().to_vec()).unwrap_or(vec![]);
        let response = self.get(std::str::from_utf8(&url).unwrap()).await;
        match response {
            Ok(r) => Ok(r.url().to_owned()),
            Err(e) => Err(Error::ExpandError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base::BaseShortener,
        bitly::{self},
    };
    use std::collections::BTreeMap;

    const TOKEN: &str = "TEST_TOKEN";
    const SHORTEN: &str = "http://bit.ly/test";
    const EXPANDED: &str = "http://www.test.com";

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_without_scheme() {
        let bitly = bitly::Shortener {
            timeout: 2,
            verify: true,
            proxies: BTreeMap::new(),
        };
        let bitly = bitly.expand("bit.ly/TEST").await.unwrap();
        assert_eq!(bitly.as_str(), "https://www.autosouk.com/News/templates/CarReviewPricePhotoArticlesDubai.Aspx?articleid=50&zoneid=3");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_bitly_with_scheme() {
        let bitly = bitly::Shortener {
            timeout: 2,
            verify: true,
            proxies: BTreeMap::new(),
        };
        let bitly = bitly.expand("http://bit.ly/TEST").await.unwrap();
        assert_eq!(bitly.as_str(), "https://www.autosouk.com/News/templates/CarReviewPricePhotoArticlesDubai.Aspx?articleid=50&zoneid=3");
    }
}
