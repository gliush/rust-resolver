extern crate url;
use self::url::Url;
//use self::url::ParseResult;
//use self::url::ParseError;

use resolver::hyper_lib;

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
        Ok(_) => {
            let (status, content_type, location, body) = hyper_lib::http_get(x);

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
