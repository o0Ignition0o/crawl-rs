extern crate crawl_rs;
extern crate hyper;
extern crate serde_json;

use hyper::StatusCode;
use crawl_rs::http::client::{get_json, get_string, CrawlResponse};

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
