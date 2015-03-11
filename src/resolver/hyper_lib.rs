use std::io::Read;

//extern crate url;
//use self::url::Url;

extern crate hyper;
use self::hyper::Client;
//use self::hyper::Url;
//use self::hyper::client::Request;
//use self::hyper::status::StatusCode;
use self::hyper::header::ContentType;
use self::hyper::header::Location;
use self::hyper::header::HeaderFormatter;
use self::hyper::header::HeaderFormat;

extern crate core;
use self::core::num::ToPrimitive;

pub fn http_get(url: &str) -> (Option<u16>, Option<String>, Option<String>, String) {
    let mut client = Client::new();
    let mut res = client.get(url) 
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //println!("body:{}", body);

    let ct = res.headers.get::<ContentType>().map(repr_header);
    let l = res.headers.get::<Location>().map(repr_header);

    (res.status.class().to_u16(), ct, l, body)
}

fn repr_header<T: HeaderFormat>(c: &T) -> String {
    format!("{:?}", HeaderFormatter(c))
}
