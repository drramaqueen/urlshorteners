pub mod base;
pub mod bitly;
pub mod error;
#[cfg(test)]
// use std::collections::BTreeMap;
mod tests {

    use crate::{
        base::BaseShortener,
        bitly::{self},
    };
    use std::collections::BTreeMap;

    #[tokio::test(flavor = "multi_thread")]

    async fn it_works() {
        let b = bitly::Shortener {
            timeout: 2,
            verify: true,
            proxies: BTreeMap::new(),
        };
        b.expand("https://bit.ly/TEST").await;
        b.clean_url("google.com".as_bytes().to_vec());
        b.clean_url("google".as_bytes().to_vec());
    }
}
