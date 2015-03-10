extern crate hyper;
use self::hyper::Client;
use self::hyper::status::StatusCode;
use self::hyper::header::ContentType;
use self::hyper::header::Location;
use self::hyper::header::HeaderFormatter;
use self::hyper::header::HeaderFormat;

extern crate core;
use self::core::num::ToPrimitive;

use std::io::Read;


use std::collections::HashMap;

#[derive(Debug)]
struct Page {
    orig_url: String,
    resolved_url: String,
    title: String
}

#[derive(Debug)]
enum Result {
    ResolveResult(Page),
    Error(i32)
}

struct Redirect (String, i32); // (url, http_code)
struct Redirects ( Vec<Redirect> );

pub fn resolve(x: &str) ->  Result{
    let (status, content_type, location, body) = http_get(x);

    println!("status:{:?}", status);
    println!("ct:{:?}", content_type );
    println!("l:{:?}", location );
    println!("body: {}", body.len());
    Result::Error(0)
}

fn http_get(x: &str) -> (Option<u16>, Option<String>, Option<String>, String) {
    let mut client = Client::new();
    let mut res = client.get(x) 
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //println!("body:{}", body);

    let ct = res.headers.get::<ContentType>().map(reprHeader);
    let l = res.headers.get::<Location>().map(reprHeader);

    (res.status.class().to_u16(), ct, l, body)
}

fn reprHeader<T: HeaderFormat>(c: &T) -> String {
    format!("{:?}", HeaderFormatter(c))
}


fn extract_title(h: i32, b: i32) -> Result {
    Result::ResolveResult(Page{ 
        orig_url: String::from_str("orig_url"), 
        resolved_url: String::from_str("resolved_url"), 
        title: String::from_str("title")})
}
