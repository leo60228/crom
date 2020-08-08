use chrono::prelude::*;
use graphql_client::GraphQLQuery;
use url::Url;

type URL = Url;
type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/search_pages.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct SearchPages;
