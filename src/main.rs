#![feature(core)]
#![feature(io)]
#![feature(collections)]

mod resolver;

fn main() {
    //println!("result:{:?}", resolver::resolver_lib::resolve("http://ya.ru"));
    println!("result:{:?}", resolver::resolver_lib::resolve("http://bit.ly/1bh0k2I"));
}
