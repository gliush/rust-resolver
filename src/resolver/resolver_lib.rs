extern crate url;
use self::url::Url;
//use self::url::ParseResult;
//use self::url::ParseError;

use resolver::hyper_lib;

#[derive(Debug)]
struct Page {
    orig_url: String,
    resolved_url: Option<String>,
    title: Option<String>,
    redirects: Vec<String>
}

pub fn resolve(x: &str) ->  Result<Box<Page>, String>{
    let res = Box::new(Page {
        orig_url: String::from_str(x),
        resolved_url: None,
        title: None,
        redirects: Vec::new()
    });
    resolve_ll(0, x, res)
}

fn resolve_ll(attempt: u8, x: &str, mut res: Box<Page>) ->  Result<Box<Page>, String>{
    match Url::parse(x) {
        Ok(_) => {
            let (status, content_type, location, body) = hyper_lib::http_get(x);

            println!(" resolve({}) -> at:{} st:{:?} ct:{:?} l:{:?} r:{} body:{}", 
                x, attempt, status, content_type, location, format!("{:?}", res.redirects), body.len());

            match status {
                Some(200) => { 
                    res.resolved_url = Some(String::from_str(x));
                    Ok(res)
                },
                Some(300) => {
                    res.redirects.push(String::from_str(x));
                    match location {
                        Some(l) => {
                            let url = reconstruct_url(l, x);
                            resolve_ll(attempt+1, &url, res)
                        },
                        None => {
                            let msg = format!("url {} is a redirects while location is not set", x);
                            println!("{}", msg);
                            res.resolved_url = Some(String::from_str(x));
                            Ok(res)
                        }
                    }
                },
                _ => Ok(res)
            }
        },
        Err(e) => {
            let msg = format!("Malformed URL: {} ({})", x, e);
            println!("{}", msg);
            Err(msg)
        }
    }
}

fn reconstruct_url(location: String, prev_url: &str) ->  String {
    match location {
        _ if &location[0..4] == "http" => location,
        _ if location.as_bytes()[0] == b'/' && &prev_url[0..3] == "http" => {
            let mut new_url = String::from_str(prev_url);
            new_url.push_str(location.as_slice());
            new_url
        },
        _ => String::from_str(prev_url)
    }
}
