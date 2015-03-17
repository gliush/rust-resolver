extern crate regex;
use self::regex::Regex;

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

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.orig_url == other.orig_url
            && self.resolved_url == other.resolved_url
            && self.title == other.title
            && self.redirects == other.redirects
    }
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

#[cfg(not(test))]
fn http_get(x: &str) -> (Option<u16>, Option<String>, Option<String>, Option<String>) {
    hyper_lib::http_get(x)
}

#[cfg(test)]
fn http_get(x: &str) -> (Option<u16>, Option<String>, Option<String>, Option<String>) {
    let html_content_type = Some(String::from_str("text/html; charset=UTF-8"));
    let loc_example_com = Some(String::from_str("http://example.com"));
    let body1 = Some(String::from_str(""));
    
    match x {
        "example.com" => (Some(200), html_content_type, None, body1),
        "redir1_to_example.com" => (Some(300), None, loc_example_com, None),
        _ => (None, None, None, None)
    }
}

fn resolve_ll(attempt: u8, x: &str, mut res: Box<Page>) ->  Result<Box<Page>, String>{
    println!("resolve_ll({}, {}, {:?})", attempt, x, res);
    match Url::parse(x) {
        Ok(_) => {
            let (status, content_type, location, body) = http_get(x);

            let body_len = body.as_ref().map(String::len);
            println!(" resolve({}) -> at:{} st:{:?} ct:{:?} l:{:?} r:{} body:{:?}", 
                x, attempt, status, content_type, location, format!("{:?}", res.redirects), body_len);

            match status {
                Some(200) => { 
                    res.resolved_url = Some(String::from_str(x));
                    res.title = body.and_then(|s| {extract_title(&s)});
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

 
fn extract_title(body: &str) -> Option<String> {
    meta_value(body, "og:title")
        .or_else(|| {html_value(body, "title")})
}

fn meta_value(body: &str, tag: &'static str) -> Option<String> {
    //// do not allow newlines in content, trim
    let value_quoted1  = "'\\s*([^'\n]+?)\\s*'";
    let value_quoted2  = "\"\\s*([^\"\n]+?)\\s*\"";
    let value_unquoted = "\\s*([^\\s'\"\n>][^>\n]*?)";
    let str_re = String::from_str("(?ims)<meta\\s+property=['\"]?") + tag + "['\"]?\\s+"
            + "content=(" + value_quoted1 + "|" + value_quoted2 + "|" + value_unquoted + ")\\s*/?>";

    let re = Regex::new(&str_re).unwrap();
    match re.captures(body) {
        None => None,
        Some(caps) => caps.at(1).map(|s| {String::from_str(s)})
    }
}

fn html_value(body: &str, tag: &'static str) -> Option<String> {
    let str_re = String::from_str("(?ims)<") + tag + ">\\s*(.+?)\\s*</" + tag + ">";
    let re = Regex::new(&str_re).unwrap();
    match re.captures(body) {
        None => None,
        Some(caps) => caps.at(1).map(|s| {String::from_str(s)})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn check_redirect() {
        let r : Result<Box<super::Page>, String> = Err(String::from_str("hello"));
        println!("res: {:?}", super::http_get("redir1_to_example.com"));
        assert_eq!(resolve("redir1_to_example.com"), r);
    }
}
