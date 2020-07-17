/*!
# URL Prefix

This crate can be used to create URL prefix strings by inputting a protocol, a domain, a port number and a path without additional parsing.

## Why We Need This?

Sometimes our web applications are run on different protocols(HTTP/HTTPS) and domains. And it is boring to write some code like below to format a URL:

```rust,ignore
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

let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", None, None::<String>);

assert_eq!("https://magiclen.org", prefix);
```

```rust
extern crate url_prefix;

let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", Some(8100), Some("url-prefix"));

assert_eq!("https://magiclen.org:8100/url-prefix", prefix);
```
*/

#![no_std]

#[macro_use]
extern crate alloc;

extern crate cow_utils;
extern crate slash_formatter;

use core::fmt::Write;

use alloc::string::String;

use cow_utils::CowUtils;

macro_rules! impl_protocol {
    ( $($protocol:ident, $name:expr, $port:expr); * $(;)* ) => {
        /// A set of protocols for URLs.
        #[derive(Debug, Clone)]
        pub enum Protocol {
            $(
                $protocol,
            )+
            /// Your own custom protocol created by giving a name and a default port number.
            Custom(String, u16)
        }

        impl Protocol{
            pub fn get_default_from_str<S: AsRef<str>>(s: S) -> Option<Self>{
                let lowered_case = s.as_ref().cow_to_lowercase();

                match lowered_case.as_ref() {
                    $(
                        $name => Some(Protocol::$protocol),
                    )+
                    _ => None
                }
            }

            pub fn get_default_port(&self) -> u16 {
                match self {
                    $(
                        Protocol::$protocol => $port,
                    )+
                    Protocol::Custom(_, port) => *port
                }
            }

            pub fn get_name(&self) -> &str {
                match self {
                    $(
                        Protocol::$protocol => $name,
                    )+
                    Protocol::Custom(name, _) => &name
                }
            }
        }
    };
}

impl_protocol! {
    HTTP, "http", 80;
    HTTPS, "https", 443;
    FTP, "ftp", 21;
    WS, "ws", 80;
    WSS, "wss", 443;
}

/// Create a URL prefix string.
pub fn create_prefix<S: AsRef<str>, P: AsRef<str>>(
    protocol: Protocol,
    domain: S,
    port: Option<u16>,
    path: Option<P>,
) -> String {
    let protocol_name = protocol.get_name();

    let mut prefix = format!("{}://{}", protocol_name, domain.as_ref());

    if let Some(port) = port {
        let protocol_port = protocol.get_default_port();
        if port != protocol_port {
            prefix.write_fmt(format_args!(":{}", port)).unwrap();
        }
    }

    if let Some(path) = path {
        let path = path.as_ref();

        slash_formatter::concat_with_slash_in_place(&mut prefix, path);
    }

    prefix
}