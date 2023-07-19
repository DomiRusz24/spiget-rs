use serde_json::json;
use crate::model::*;
use crate::SpigetClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCategoriesByCategoryRequest<'a> {
    pub(crate) http_client: &'a SpigetClient,
    pub category: f64,
}
impl<'a> GetCategoriesByCategoryRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Category> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/categories/{category}", category = self.category));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetCategoriesByCategoryRequest<'a> {
    type Output = httpclient::InMemoryResult<Category>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}