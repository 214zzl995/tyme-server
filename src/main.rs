
#[macro_use]
extern crate lazy_static;

mod ssl_publish;
mod async_subscribe_v5;

fn main() {
    env_logger::init();
    // ssl_publish::publish().unwrap();
    async_subscribe_v5::subscribe().unwrap();
    println!("Hello, world!");
}
