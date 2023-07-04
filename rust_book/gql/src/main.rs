use graphql_client::{ GraphQLQuery, reqwest::post_graphql };

fn main() {
    println!("Hello, world!");
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "queries/user_push_tokens.graphql",
    schema = "queries/schema.json",
    response_derives = "Debug"
)]
pub struct UserPushTokens;

