use std::io::Read;

//extern crate url;
//use self::url::Url;

extern crate hyper;
use self::hyper::Client;
use self::hyper::client::RedirectPolicy;
//use self::hyper::Url;
//use self::hyper::client::Request;
//use self::hyper::status::StatusCode;


use self::hyper::mime::Mime;
use self::hyper::mime::TopLevel::Text;
use self::hyper::mime::SubLevel::Html;

use self::hyper::header::qitem;
use self::hyper::header::Accept;
use self::hyper::header::Connection;
use self::hyper::header::UserAgent;
use self::hyper::header::ConnectionOption;

use self::hyper::header::ContentType;
use self::hyper::header::Location;
use self::hyper::header::HeaderFormatter;
use self::hyper::header::HeaderFormat;

extern crate core;
use self::core::num::ToPrimitive;

pub fn http_get(url: &str) -> (Option<u16>, Option<String>, Option<String>, String) {
    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);
    let mut res = client.get(url) 
        .header(Accept(vec![
            qitem(Mime(Text, Html, vec![])),
            ]))
        .header(Connection(vec![ConnectionOption::Close]))
        .header(UserAgent("Mozilla/5.0 (X11; CrOS x86_64 6158.70.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/38.0.2125.110 Safari/537.36".to_string()))
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //println!("body:{}", body);
    //println!("res.status:{:?}", res.status);
    //println!("res.headers:{:?}", res.headers);
    //println!("res.version:{:?}", res.version);

    let ct = res.headers.get::<ContentType>().map(repr_header);
    let l = res.headers.get::<Location>().map(repr_header);

    (res.status.class().to_u16(), ct, l, body)
}

fn repr_header<T: HeaderFormat>(c: &T) -> String {
    format!("{:?}", HeaderFormatter(c))
}
