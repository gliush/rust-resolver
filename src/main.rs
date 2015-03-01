#![feature(phase)]

//extern crate http;
extern crate url;
extern crate regex;

#[phase(plugin)]
extern crate regex_macros;

use http::client::RequestWriter;
use http::method::Get;
use std::{os, str};
use url::{Url, ParseError};
use std::io::{BufferedReader, File, IoError};

#[deriving(Show)]
enum GetTitleError {
  InvalidUrl(ParseError),
  RequestSendFailed(IoError),
  ResponseReadFailed(IoError),
  BodyDecodeFailed,
  ParseFailed
}

fn main() {
  let args = os::args();
  match args.len() {
    0 => unreachable!(),
    2 => {
      let path = Path::new(args[1].as_slice());
      let mut file = BufferedReader::new(File::open(&path));
      for line in file.lines() {
        let tmp = line.unwrap();
        let url = tmp.trim();
        let text = match get_title(url.as_slice()) {
          Ok(title) => title,
          Err(err) => format!("NO TITLE ({})", err).to_string(),
        };
        println!("<li><a href=\"{}\">{}</a></li>", url, text);
      }
    },
    _ => println!("Usage: {} <file>", args[0]),
  };
}

fn get_title(url: &str) -> Result<String, GetTitleError> {
  let body = try!(get_html(url));
  let re = regex!(r"(?is)<title>(.*?)</title>");
  match re.captures(body.as_slice()) {
    Some(cap) => Ok(cap.at(1).to_string()),
    None => Err(ParseFailed),
  }
}

fn get_html(url: &str) -> Result<String, GetTitleError> {
  let url = match Url::parse(url) {
    Ok(u) => u,
    Err(e) => return Err(InvalidUrl(e)),
  };

  let request: RequestWriter = RequestWriter::new(Get, url).unwrap();

  let mut response = match request.read_response() {
    Ok(resp) => resp,
    Err( (_, err) ) => return Err(RequestSendFailed(err)),
  };

  match response.read_to_end() {
    Ok(body) => {
      match str::from_utf8(body.as_slice()) {
        Some(rslt) => Ok(rslt.to_string()),
        None => Err(BodyDecodeFailed),
      }
    },
    Err(err) => Err(ResponseReadFailed(err)),
  }
}


//fn main() {
//    println!("Hello, world!");
//}
