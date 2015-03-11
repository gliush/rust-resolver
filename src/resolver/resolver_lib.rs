extern crate url;
use self::url::Url;
//use self::url::ParseResult;
//use self::url::ParseError;

use resolver::hyper_lib;

#[derive(Debug)]
struct Redirect (String, i32); // (url, http_code)
#[derive(Debug)]
struct Redirects ( Vec<Redirect> );

#[derive(Debug)]
struct Page {
    orig_url: String,
    resolved_url: Option<String>,
    title: Option<String>,
    redirects: Redirects
}

pub fn resolve(x: &str) ->  Result<Box<Page>, String>{
    let res = Box::new(Page {
        orig_url: String::from_str(x),
        resolved_url: None,
        title: None,
        redirects: Redirects(Vec::new())
    });
    resolve_ll(0, x, res)
}

fn resolve_ll(attempt: u8, x: &str, mut res: Box<Page>) ->  Result<Box<Page>, String>{
    match Url::parse(x) {
        Ok(_) => {
            let (status, content_type, location, body) = hyper_lib::http_get(x);

            let resolved_url = match status {
                Some(200) => Some(String::from_str(x)), // ok
                _ => None
            };
            println!(" resolve({}) -> at:{} status:{:?} ct:{:?} l:{:?} body:{}", 
                x, attempt, status, content_type, location, body.len());
            res.resolved_url = resolved_url;
            Ok(res) 
        },
        Err(e) => {
            let msg = format!("Malformed URL: {} ({})", x, e);
            println!("{}", msg);
            Err(msg)
        }
    }
}
