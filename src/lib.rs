mod graphql;

pub use graphql::search_pages::SearchPagesSearchPages as Page;
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    Http(surf::Error),
    #[error("graphql error: {}", .0.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))]
    GraphQL(Vec<graphql_client::Error>),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<surf::Error> for Error {
    fn from(e: surf::Error) -> Self {
        Self::Http(e)
    }
}

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

pub struct Client<T: HttpClient> {
    client: surf::Client<T>,
}

#[cfg(feature = "h1-client")]
impl Client<H1Client> {
    pub fn new() -> Self {
        Self::with_client(H1Client::new())
    }
}

#[cfg(feature = "native-client")]
impl Client<NativeClient> {
    pub fn new() -> Self {
        Self::with_client(NativeClient::new())
    }
}

impl<C: HttpClient> Client<C> {
    pub fn with_client(client: C) -> Self {
        let client = surf::Client::with_client(client);
        Self { client }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Page>> {
        use graphql::{search_pages::*, SearchPages};

        let query = query.to_string();
        let variables = Variables { query };
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
