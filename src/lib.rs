//! API client for [Crom](https://crom.avn.sh/), a scraper for the [SCP
//! Wiki](http://www.scpwiki.com/) and related projects.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod graphql;

pub use graphql::search_pages::SearchPagesSearchPages as Page;
pub use graphql::search_pages::SearchPagesSearchPagesAlternateTitles as PageAlternateTitle;
pub use graphql::search_pages::SearchPagesSearchPagesWikidotInfo as WikidotInfo;
pub use graphql::search_pages::SearchPagesSearchPagesWikidotInfoCreatedBy as CreatedBy;

use graphql_client::{GraphQLQuery, Response};
use http_client::HttpClient;
use std::result::Result as StdResult;
use thiserror::Error;

#[cfg(feature = "native-client")]
use http_client::native::NativeClient;

#[cfg(feature = "h1-client")]
use http_client::h1::H1Client;

/// Error type.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP error.
    #[error("http error: {0}")]
    Http(surf::Error),
    /// GraphQL error.
    #[error("graphql error: {}", .0.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))]
    GraphQL(Vec<graphql_client::Error>),
    /// JSON error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// IO error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<surf::Error> for Error {
    fn from(e: surf::Error) -> Self {
        Self::Http(e)
    }
}

/// Result type.
pub type Result<T> = StdResult<T, Error>;

fn res_from_resp<T>(r: Response<T>) -> Result<T> {
    match r {
        Response {
            errors: Some(errors),
            ..
        } => Err(Error::GraphQL(errors)),
        Response {
            data: Some(data),
            errors: None,
        } => Ok(data),
        _ => Err(Error::GraphQL(vec![])),
    }
}

/// API client.
pub struct Client<T: HttpClient> {
    client: surf::Client<T>,
}

#[cfg(feature = "h1-client")]
impl Client<H1Client> {
    /// Create a new client with default settings.
    pub fn new() -> Self {
        Self::with_client(H1Client::new())
    }
}

#[cfg(feature = "native-client")]
impl Client<NativeClient> {
    /// Create a new client with default settings.
    pub fn new() -> Self {
        Self::with_client(NativeClient::new())
    }
}

impl<C: HttpClient> Client<C> {
    /// Create a customized new client.
    pub fn with_client(client: C) -> Self {
        let client = surf::Client::with_client(client);
        Self { client }
    }

    /// Search pages.
    pub async fn search(&self, query: &str, base_urls: Option<Vec<String>>) -> Result<Vec<Page>> {
        use graphql::{search_pages::*, SearchPages};

        let query = query.to_string();
        let filter = SearchPagesFilter {
            any_base_url: base_urls,
        };
        let variables = Variables { query, filter };
        let body = SearchPages::build_query(variables);

        let resp: Response<ResponseData> = self
            .client
            .post("https://api.crom.avn.sh/")
            .body_json(&body)?
            .await?
            .body_json()
            .await?;

        Ok(res_from_resp(resp)?.search_pages)
    }
}
