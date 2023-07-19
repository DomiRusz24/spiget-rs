use serde_json::json;
use crate::model::*;
use crate::SpigetClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetResourcesReviewsByResourceRequest<'a> {
    pub(crate) http_client: &'a SpigetClient,
    pub fields: Option<String>,
    pub page: Option<f64>,
    pub resource: f64,
    pub size: Option<f64>,
    pub sort: Option<String>,
}
impl<'a> GetResourcesReviewsByResourceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<ResourceReview>> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/resources/{resource}/reviews", resource = self.resource));
        if let Some(ref unwrapped) = self.fields {
            r = r.query("fields", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.size {
            r = r.query("size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.query("sort", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn fields(mut self, fields: &str) -> Self {
        self.fields = Some(fields.to_owned());
        self
    }
    pub fn page(mut self, page: f64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetResourcesReviewsByResourceRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<ResourceReview>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}