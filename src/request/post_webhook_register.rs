use serde_json::json;
use crate::model::*;
use crate::SpigetClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostWebhookRegisterRequest<'a> {
    pub(crate) http_client: &'a SpigetClient,
}
impl<'a> PostWebhookRegisterRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<InlineResponse2002> {
        let mut r = self.http_client.client.post("/webhook/register");
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostWebhookRegisterRequest<'a> {
    type Output = httpclient::InMemoryResult<InlineResponse2002>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}