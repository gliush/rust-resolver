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
                    let msg = format!("url {} is a redirects while location is not set", x);
                    assert!(location.is_some(), msg);
                    let l = location.unwrap();
                    res.redirects.push(String::from_str(x));
                    resolve_ll(attempt+1, &l, res)
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
