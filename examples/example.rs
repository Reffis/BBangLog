extern crate bbanglog;

use bbanglog::*;

fn main() {
    info!("Hello, world! {}", "WTF");
    warn!("Hello, world! {}", "WTF");
    error!("Hello, world! {}", "WTF");
}
