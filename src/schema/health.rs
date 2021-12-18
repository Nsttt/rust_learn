use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Return `true` to know the GraphQL server is reachable.
    async fn heatlh(&self) -> bool {
        true
    }
}
