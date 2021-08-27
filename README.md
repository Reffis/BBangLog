# BBangLog

Simple logging library

```toml
[dependencies]
bbanglog = "0.1.0"
```

```rs
extern crate bbanglog;

use bbanglog::*;

fn main() {
    info!("Hello, world! {}", "WTF");
    warn!("Hello, world! {}", "WTF");
    error!("Hello, world! {}", "WTF");
}
```