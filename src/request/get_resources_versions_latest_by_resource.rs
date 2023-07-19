use serde_json::json;
use crate::model::*;
use crate::SpigetClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetResourcesVersionsLatestByResourceRequest<'a> {
    pub(crate) http_client: &'a SpigetClient,
    pub resource: f64,
}
impl<'a> GetResourcesVersionsLatestByResourceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ResourceVersion> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/resources/{resource}/versions/latest", resource = self.resource
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetResourcesVersionsLatestByResourceRequest<'a> {
    type Output = httpclient::InMemoryResult<ResourceVersion>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}