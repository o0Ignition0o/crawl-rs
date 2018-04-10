/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use providers::crawl::{CrawlError, CrawlResponse, ResponseStatus};
use futures::Future;
use futures::future::ok;
use tokio_core::reactor::Core;

use std::fs::File;
use std::io::{BufReader, Read};

pub fn crawl(file_path: &str) -> Result<CrawlResponse, CrawlError> {
    let mut core = Core::new().unwrap();
    core.run(async_crawl(file_path))
}

pub fn async_crawl(file_path: &str) -> impl Future<Item = CrawlResponse, Error = CrawlError> {
    open(file_path).and_then(|file| into_string(file))
}

fn open(path: &str) -> impl Future<Item = File, Error = CrawlError> {
    let file = File::open(path).unwrap();
    ok(file)
}

fn into_string(open_file: File) -> impl Future<Item = CrawlResponse, Error = CrawlError> {
    let mut buf_reader = BufReader::new(open_file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    ok(CrawlResponse {
        body: content,
        status: ResponseStatus::Success(),
    })
}
