/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use providers::crawl::{CrawlError, CrawlResponse};
use hyper::Response;
use hyper::Uri;
use hyper::client::HttpConnector;
use serde_json::Value;
use hyper::Error;
use hyper::Client;
use tokio_core::reactor::Core;
use futures::Future;
use futures::future::ok;
use futures::Stream;
use serde_json;

pub fn get_json(uri: String) -> Result<CrawlResponse<Value>, CrawlError> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = uri.parse()
        .or_else(|error| Err(CrawlError::UriError(error)))?;
    let work = get(uri, &client)
        .map_err(|error| CrawlError::HttpError(error))
        .and_then(|response| into_json_response(response));
    core.run(work)
}

pub fn get_string(uri: String) -> Result<CrawlResponse<String>, CrawlError> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = uri.parse()
        .or_else(|error| Err(CrawlError::UriError(error)))?;
    let work = get(uri, &client)
        .map_err(|error| CrawlError::HttpError(error))
        .and_then(|response| into_string_response(response));
    core.run(work)
}

fn get(uri: Uri, client: &Client<HttpConnector>) -> impl Future<Item = Response, Error = Error> {
    client.get(uri)
}

fn into_json_response(
    response: Response,
) -> impl Future<Item = CrawlResponse<Value>, Error = CrawlError> {
    let status = response.status();
    response
        .body()
        .concat2()
        .and_then(move |body| {
            let content: Value =
                serde_json::from_slice(&body).unwrap_or(serde_json::from_str(r#"{}"#).unwrap());
            ok(CrawlResponse {
                body: content,
                status: status,
            })
        })
        .map_err(|error| CrawlError::HttpError(error))
}

fn into_string_response(
    response: Response,
) -> impl Future<Item = CrawlResponse<String>, Error = CrawlError> {
    let status = response.status();
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

#[cfg(test)]
mod client_tests {
    use hyper::StatusCode;
    use providers::http::{get_json, get_string, CrawlResponse};

    #[test]
    fn get_json_with_valid_url() {
        let uri = "http://httpbin.org/ip".to_string();
        let CrawlResponse { body, status } = get_json(uri).unwrap();

        assert_eq!(
            StatusCode::Ok,
            status,
            "JSON status is supposed to be StatusCode::Ok, got : {} \n {:?} \n",
            status,
            body
        );
    }

    #[test]
    fn get_string_with_valid_url() {
        let uri = "http://httpbin.org/ip".to_string();
        let CrawlResponse { body, status } = get_string(uri).unwrap();
        assert_eq!(
            StatusCode::Ok,
            status,
            "JSON status is supposed to be StatusCode::Ok, got : {} \n {:?} \n",
            status,
            body
        );
    }
}
