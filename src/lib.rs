mod graphql;

pub use graphql::search_pages::SearchPagesSearchPages as Page;
pub use graphql::search_pages::SearchPagesSearchPagesWikidotInfo as WikidotInfo;
pub use graphql::search_pages::SearchPagesSearchPagesWikidotInfoCreatedBy as CreatedBy;

use graphql_client::GraphQLQuery;

use http_client::HttpClient;

#[cfg(feature = "native-client")]
use http_client::native::NativeClient;

#[cfg(feature = "h1-client")]
use http_client::h1::H1Client;

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

    pub async fn search(&self, query: &str) -> surf::Result<Vec<Page>> {
        use graphql::{search_pages::*, SearchPages};

        let query = query.to_string();
        let variables = Variables { query };
        let body = SearchPages::build_query(variables);

        let resp: ResponseData = self
            .client
            .post("https://api.crom.avn.sh/")
            .body_json(&body)?
            .await?
            .body_json()
            .await?;

        Ok(resp.search_pages)
    }
}
