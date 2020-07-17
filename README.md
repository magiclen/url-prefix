URL Prefix
====================

[![Build Status](https://travis-ci.org/magiclen/url-prefix.svg?branch=master)](https://travis-ci.org/magiclen/url-prefix)

This crate can be used to create URL prefix strings by inputting a protocol, a domain, a port number and a path without additional parsing.

## Why We Need This?

Sometimes our web applications are run on different protocols(HTTP/HTTPS) and domains. And it is boring to write some code like below to format a URL:

```rust
let mut url_prefix = String::new();
if is_https {
    url_prefix.push_str("https://");
} else {
    url_prefix.push_str("http://");
}
url_prefix.push_str(domain);

if is_https && port != 443 || !is_https && port != 80 {
    url_prefix.push_str(":");
    url_prefix.push_str(&port.to_string());
}
```

Instead, we can easily use this crate to create URL prefix strings. For examples,

```rust
extern crate url_prefix;

let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", None, None);

assert_eq!("https://magiclen.org", prefix);
```

```rust
extern crate url_prefix;

let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", Some(8100), Some("url-prefix"));

assert_eq!("https://magiclen.org:8100/url-prefix", prefix);
```

## Crates.io

https://crates.io/crates/url-prefix

## Documentation

https://docs.rs/url-prefix

## License

[MIT](LICENSE)