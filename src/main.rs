#![feature(core)]
#![feature(io)]
#![feature(collections)]

mod resolver;

fn main() {
    println!("result:{:?}", resolver::resolver_lib::resolve("http://ya.ru"));
}
