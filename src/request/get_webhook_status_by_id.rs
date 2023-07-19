use serde_json::json;
use crate::model::*;
use crate::SpigetClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetWebhookStatusByIdRequest<'a> {
    pub(crate) http_client: &'a SpigetClient,
    pub id: String,
}
impl<'a> GetWebhookStatusByIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<InlineResponse2003> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/webhook/status/{id}", id = self.id));
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for GetWebhookStatusByIdRequest<'a> {
    type Output = httpclient::InMemoryResult<InlineResponse2003>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}