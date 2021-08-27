extern crate bbanglog;

use bbanglog::*;

fn main() {
    info!("Hello, world! {}", "hehe");
    warn!("Hello, world! {}", "boi");
    error!("Hello, world! {}", "error");
}
