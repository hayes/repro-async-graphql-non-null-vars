use async_graphql::*;
use futures::executor::block_on;

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn echo(&self, value: String) -> String {
        value
    }
}

async fn run_query() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    // let value = serde_json::from_str("{ \"value\": \"hello, World!\" }").unwrap_or_default();
    // let variables = async_graphql::Variables::from_json(value);

    let req = Request::new("query ($value: String! = \"hello, World!\") { echo(value: $value) }");
    //.variables(variables);

    let res = schema.execute(req).await;

    let json = serde_json::to_string(&res);

    println!("{:?}", json);
}

fn main() {
    let future = run_query();
    block_on(future);
}
