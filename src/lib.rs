//! [`SpigetClient`](struct.SpigetClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct SpigetClient {
    pub client: httpclient::Client,
}
impl SpigetClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new()
                .base_url(
                    std::env::var("SPIGET_BASE_URL")
                        .expect("Missing environment variable SPIGET_BASE_URL")
                        .as_str(),
                ),
        }
    }
}
impl SpigetClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Author List

Get a list of available authors
Note: This only includes members involved with resources, either being their author or having reviewed a resource
*/
    pub fn get_authors(&self) -> request::GetAuthorsRequest {
        request::GetAuthorsRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Author Details

Get details about an author
*/
    pub fn get_authors_by_author(
        &self,
        author: f64,
    ) -> request::GetAuthorsByAuthorRequest {
        request::GetAuthorsByAuthorRequest {
            http_client: &self,
            author,
        }
    }
    /**Author resources

Get an author's resources
*/
    pub fn get_authors_resources_by_author(
        &self,
        author: f64,
    ) -> request::GetAuthorsResourcesByAuthorRequest {
        request::GetAuthorsResourcesByAuthorRequest {
            http_client: &self,
            author,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Author reviews

Get an author's reviews left on resources
*/
    pub fn get_authors_reviews_by_author(
        &self,
        author: f64,
    ) -> request::GetAuthorsReviewsByAuthorRequest {
        request::GetAuthorsReviewsByAuthorRequest {
            http_client: &self,
            author,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Category List

Get a list of categories
*/
    pub fn get_categories(&self) -> request::GetCategoriesRequest {
        request::GetCategoriesRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Category Details

Get details about a category
*/
    pub fn get_categories_by_category(
        &self,
        category: f64,
    ) -> request::GetCategoriesByCategoryRequest {
        request::GetCategoriesByCategoryRequest {
            http_client: &self,
            category,
        }
    }
    /**Category Resources

Get the resources in a category
*/
    pub fn get_categories_resources_by_category(
        &self,
        category: f64,
    ) -> request::GetCategoriesResourcesByCategoryRequest {
        request::GetCategoriesResourcesByCategoryRequest {
            http_client: &self,
            category,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Resource List

Get a list of available resources (premium and free)
*/
    pub fn get_resources(&self) -> request::GetResourcesRequest {
        request::GetResourcesRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Resources for Versions

Get resources for the specified version(s)
*/
    pub fn get_resources_for_by_version(
        &self,
        version: &str,
    ) -> request::GetResourcesForByVersionRequest {
        request::GetResourcesForByVersionRequest {
            http_client: &self,
            fields: None,
            method: None,
            page: None,
            size: None,
            sort: None,
            version: version.to_owned(),
        }
    }
    /**Free Resource List

Get a list of available free resources
*/
    pub fn get_resources_free(&self) -> request::GetResourcesFreeRequest {
        request::GetResourcesFreeRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**New Resources

Get all new resources
*/
    pub fn get_resources_new(&self) -> request::GetResourcesNewRequest {
        request::GetResourcesNewRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Premium Resource List

Get a list of available premium resources
*/
    pub fn get_resources_premium(&self) -> request::GetResourcesPremiumRequest {
        request::GetResourcesPremiumRequest {
            http_client: &self,
            fields: None,
            page: None,
            size: None,
            sort: None,
        }
    }
    /**Resource Details

Get a resource by its ID
*/
    pub fn get_resources_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesByResourceRequest {
        request::GetResourcesByResourceRequest {
            http_client: &self,
            resource,
        }
    }
    /**Resource Author

Get the resource author
*/
    pub fn get_resources_author_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesAuthorByResourceRequest {
        request::GetResourcesAuthorByResourceRequest {
            http_client: &self,
            resource,
        }
    }
    /**Resource Download

Download a resource
This either redirects to spiget's CDN server (cdn.spiget.org) for a direct download of files hosted on spigotmc.org or to the URL of externally hosted resources
The `external` field of a resource should be checked before downloading, to not receive any unexpected data
*/
    pub fn get_resources_download_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesDownloadByResourceRequest {
        request::GetResourcesDownloadByResourceRequest {
            http_client: &self,
            resource,
        }
    }
    /**Resource Reviews

Get reviews of a resource
*/
    pub fn get_resources_reviews_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesReviewsByResourceRequest {
        request::GetResourcesReviewsByResourceRequest {
            http_client: &self,
            fields: None,
            page: None,
            resource,
            size: None,
            sort: None,
        }
    }
    /**Resource Updates

Get updates of a resource
*/
    pub fn get_resources_updates_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesUpdatesByResourceRequest {
        request::GetResourcesUpdatesByResourceRequest {
            http_client: &self,
            fields: None,
            page: None,
            resource,
            size: None,
            sort: None,
        }
    }
    /**Latest Resource Update

Get the latest resource update
*/
    pub fn get_resources_updates_latest_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesUpdatesLatestByResourceRequest {
        request::GetResourcesUpdatesLatestByResourceRequest {
            http_client: &self,
            fields: None,
            page: None,
            resource,
            size: None,
            sort: None,
        }
    }
    /**Resource Versions

Get versions of a resource
*/
    pub fn get_resources_versions_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesVersionsByResourceRequest {
        request::GetResourcesVersionsByResourceRequest {
            http_client: &self,
            fields: None,
            page: None,
            resource,
            size: None,
            sort: None,
        }
    }
    /**Latest Resource Version

Get the latest resource version
*/
    pub fn get_resources_versions_latest_by_resource(
        &self,
        resource: f64,
    ) -> request::GetResourcesVersionsLatestByResourceRequest {
        request::GetResourcesVersionsLatestByResourceRequest {
            http_client: &self,
            resource,
        }
    }
    /**Resource Version

Get a specific resource version by its ID
*/
    pub fn get_resources_versions_by_version(
        &self,
        resource: f64,
        version: f64,
    ) -> request::GetResourcesVersionsByVersionRequest {
        request::GetResourcesVersionsByVersionRequest {
            http_client: &self,
            resource,
            version,
        }
    }
    /**Resource Version Download

Download a specific resource version

Note: This only redirects to the stored download location and might not download a file (i.e. for external resources)
*/
    pub fn get_resources_versions_download_by_version(
        &self,
        resource: f64,
        version: &str,
    ) -> request::GetResourcesVersionsDownloadByVersionRequest {
        request::GetResourcesVersionsDownloadByVersionRequest {
            http_client: &self,
            resource,
            version: version.to_owned(),
        }
    }
    /**Author Search

Search authors
*/
    pub fn get_search_authors_by_query(
        &self,
        query: &str,
    ) -> request::GetSearchAuthorsByQueryRequest {
        request::GetSearchAuthorsByQueryRequest {
            http_client: &self,
            field: None,
            fields: None,
            page: None,
            query: query.to_owned(),
            size: None,
            sort: None,
        }
    }
    /**Resource Search

Search resources
*/
    pub fn get_search_resources_by_query(
        &self,
        query: &str,
    ) -> request::GetSearchResourcesByQueryRequest {
        request::GetSearchResourcesByQueryRequest {
            http_client: &self,
            field: None,
            fields: None,
            page: None,
            query: query.to_owned(),
            size: None,
            sort: None,
        }
    }
    /**API Status

Get the API status
*/
    pub fn get_status(&self) -> request::GetStatusRequest {
        request::GetStatusRequest {
            http_client: &self,
        }
    }
    /**Delete Webhook

Delete a Webhook
*/
    pub fn delete_webhook_delete_by_secret(
        &self,
        id: &str,
        secret: &str,
    ) -> request::DeleteWebhookDeleteBySecretRequest {
        request::DeleteWebhookDeleteBySecretRequest {
            http_client: &self,
            id: id.to_owned(),
            secret: secret.to_owned(),
        }
    }
    /**Webhook events

Get a list of available events
*/
    pub fn get_webhook_events(&self) -> request::GetWebhookEventsRequest {
        request::GetWebhookEventsRequest {
            http_client: &self,
        }
    }
    /**Register Webhook

Register a new Webhook

Use this form to easily register a new one: https://spiget.org/webhook/
*/
    pub fn post_webhook_register(&self) -> request::PostWebhookRegisterRequest {
        request::PostWebhookRegisterRequest {
            http_client: &self,
        }
    }
    /**Webhook Status

Get the status of a Webhook
*/
    pub fn get_webhook_status_by_id(
        &self,
        id: &str,
    ) -> request::GetWebhookStatusByIdRequest {
        request::GetWebhookStatusByIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
}