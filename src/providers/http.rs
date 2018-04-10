/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use providers::crawl::{CrawlError, CrawlResponse, ResponseStatus};
use hyper::Response;
use hyper::client::HttpConnector;
use hyper::Client;
use tokio_core::reactor::Core;
use futures::Future;
use futures::future::ok;
use futures::Stream;
use hyper_tls::HttpsConnector;

pub fn crawl(uri: &str) -> Result<CrawlResponse, CrawlError> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    core.run(async_crawl(uri, client))
}

pub fn async_crawl(
    uri: &str,
    http_client: Client<HttpsConnector<HttpConnector>>,
) -> impl Future<Item = CrawlResponse, Error = CrawlError> {
    let uri = uri.parse()
        .map_err(|error| CrawlError::UriError(error))
        .unwrap();
    http_client
        .get(uri)
        .map_err(|error| CrawlError::HttpError(error))
        .and_then(|response| into_string(response))
}

fn into_string(response: Response) -> impl Future<Item = CrawlResponse, Error = CrawlError> {
    let status = if response.status().is_success() {
        ResponseStatus::HttpSuccess(response.status())
    } else {
        ResponseStatus::HttpError(response.status())
    };
    response
        .body()
        .concat2()
        .and_then(move |body| {
            let content: String = String::from_utf8(body.to_vec()).unwrap_or(String::new());
            ok(CrawlResponse {
                body: content,
                status: status,
            })
        })
        .map_err(|error| CrawlError::HttpError(error))
}
