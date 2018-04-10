/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::io::ErrorKind;
use hyper;
use std::string;
use hyper::StatusCode;
use serde_json;

#[derive(Debug)]
pub struct CrawlResponse {
    pub body: String,
    pub status: ResponseStatus,
}

#[derive(Debug)]
pub enum ResponseStatus {
    HttpSuccess(StatusCode),
    HttpError(StatusCode),
    Success(),
}

#[derive(Debug)]
pub enum CrawlError {
    UriError(hyper::error::UriError),
    HttpError(hyper::Error),
    StringParseError(string::FromUtf8Error),
    JsonParseError(serde_json::Error),
    IoError(ErrorKind),
}
