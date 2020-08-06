use graphql_client::GraphQLQuery;

type URL = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/search_pages.graphql"
)]
pub struct SearchPages;
