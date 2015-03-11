use std::io::Read;

extern crate url;
//use self::url::Url;
//use self::url::ParseResult;
//use self::url::ParseError;

extern crate hyper;
use self::hyper::Client;
use self::hyper::Url;
//use self::hyper::client::Request;
//use self::hyper::status::StatusCode;
use self::hyper::header::ContentType;
use self::hyper::header::Location;
use self::hyper::header::HeaderFormatter;
use self::hyper::header::HeaderFormat;

extern crate core;
use self::core::num::ToPrimitive;

#[derive(Debug)]
struct Page {
    orig_url: String,
    resolved_url: String,
    title: String
}

//#[derive(Debug)]
//enum Result {
//    ResolveResult(Page),
//    Error(i32)
//}

//struct Redirect (String, i32); // (url, http_code)
//struct Redirects ( Vec<Redirect> );

pub fn resolve(x: &str) ->  Result<Page, String>{
    match Url::parse(x) {
        Ok(url) => {
            let (status, content_type, location, body) = http_get(url);

            println!("status:{:?}", status);
            println!("ct:{:?}", content_type );
            println!("l:{:?}", location );
            println!("body: {}", body.len());
            Ok(Page {
                orig_url: String::from_str(""),
                resolved_url: String::from_str(""), 
                title: String::from_str("")
            })},
        Err(e) => {
            let msg = format!("Malformed URL: {} ({})", x, e);
            println!("{}", msg);
            Err(msg)
        }
    }
}

fn http_get(url: Url) -> (Option<u16>, Option<String>, Option<String>, String) {
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


//fn extract_title(h: i32, b: i32) -> Result {
//    Result::ResolveResult(Page{ 
//        orig_url: String::from_str("orig_url"), 
//        resolved_url: String::from_str("resolved_url"), 
//        title: String::from_str("title")})
//}
