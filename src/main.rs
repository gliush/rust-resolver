#![feature(core)]
#![feature(io)]
#![feature(collections)]

mod resolver;

fn main() {
    println!("result:{:?}", resolver::resolve("http://ya.ru"));
}
