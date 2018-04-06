/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern crate crawl_rs;
extern crate hyper;
extern crate serde_json;

use hyper::StatusCode;
use crawl_rs::providers::http::{get_json, get_string};
use crawl_rs::providers::crawl::CrawlResponse;

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
