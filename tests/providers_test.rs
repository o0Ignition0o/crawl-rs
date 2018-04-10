/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern crate crawl_rs;
extern crate hyper;
extern crate serde_json;

use hyper::StatusCode;
use crawl_rs::providers::{file, http};
use crawl_rs::providers::crawl::{CrawlResponse, ResponseStatus};
use std::fs::File;
use std::io::Write;
use std::fs::remove_file;

#[test]
fn crawl_from_valid_url() {
    let uri = "https://httpbin.org/user-agent";
    let CrawlResponse { body, status } = http::crawl(&uri).unwrap();
    match status {
        ResponseStatus::HttpSuccess(code) => assert_eq!(
            code,
            StatusCode::Ok,
            "JSON status is supposed to be StatusCode::Ok, got : {:?} \n {:?} \n",
            status,
            body
        ),
        wrong_status => panic!(
            "Did not get the right response status {:?} \n {:?} \n",
            wrong_status, body
        ),
    }
}

#[test]
fn crawl_from_valid_file() {
    let file_name = "test.txt";
    let expected_string = b"Hello world!";
    let mut file = File::create(file_name).unwrap();
    file.write_all(expected_string).unwrap();
    let body = file::crawl(&file_name).unwrap().body;
    remove_file(file_name).unwrap();
    assert_eq!(
        body, "Hello world!",
        "The file body did not match 'Hello World!', got : {}",
        body
    );
}
