//! This crate can be used to create URL prefix strings by inputting a protocol, a domain, a port number and a path.
//!
//! ## Why You Need This?
//!
//! Sometimes your web application is run on different protocols(HTTP/HTTPS) and domains. And it is boring to write some code like below to format a URL:
//!
//! ```rust,ignore
//! let mut url_prefix = String::new();
//! if is_https {
//!     url_prefix.push_str("https://");
//! } else {
//!     url_prefix.push_str("http://");
//! }
//! url_prefix.push_str(domain);
//!
//! if is_https && port != 443 || !is_https && port != 80{
//!     url_prefix.push_str(":");
//!     url_prefix.push_str(&port.to_string());
//! }
//! ```
//!
//! Instead, you can easily use this crate to create URL prefix strings. For examples,
//!
//! ```rust
//! extern crate url_prefix;
//!
//! let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", None, None);
//!
//! assert_eq!("https://magiclen.org", prefix);
//! ```
//!
//! ```rust
//! extern crate url_prefix;
//!
//! let prefix = url_prefix::create_prefix(url_prefix::Protocol::HTTPS, "magiclen.org", Some(8100), Some("url-prefix"));
//!
//! assert_eq!("https://magiclen.org:8100/url-prefix", prefix);
//! ```
//!
//! ## Validators Support
//!
//! `Validators` is a crate which can help you validate user input, in order to create a safe URL prefix.
//!
//! To use with `Validators` support, you have to enable the **validator** feature for this crate.
//!
//!
//! ```toml
//! [dependencies.url-prefix]
//! version = "*"
//! features = ["validator"]
//! ```
//! And the `create_prefix_with_validated_domain`, `create_prefix_with_validated_ipv4`, `create_prefix_with_validated_ipv6`, `create_prefix_with_validated_host` functions are available.
//!
//! For example,
//!
//! ```rust,ignore
//! extern crate url_prefix;
//!
//! let user_input = url_prefix::validators::domain::DomainLocalhostableWithPort::from_str("magiclen.org:443").unwrap();
//!
//! let prefix = url_prefix::create_prefix_with_validated_domain(url_prefix::Protocol::HTTPS, user_input.as_domain(), Some("url-prefix"));
//!
//! assert_eq!("https://magiclen.org/url-prefix", prefix);
//! ```

#[cfg(feature = "validator")]
pub extern crate validators;

#[cfg(feature = "validator")]
use validators::domain::Domain;

#[cfg(feature = "validator")]
use validators::host::Host;

#[cfg(feature = "validator")]
use validators::ipv4::IPv4;

#[cfg(feature = "validator")]
use validators::ipv6::IPv6;

#[cfg(feature = "validator")]
use validators::http_url::HttpUrlLocalableWithProtocol;

macro_rules! impl_protocol {
    ( $($protocol:ident, $name:expr, $port:expr), * ) => {
        /// A set of protocols for URLs.
        pub enum Protocol{
            $(
                $protocol,
            )+
            /// Your own custom protocol created by giving a name and a default port number.
            Custom(String, u16)
        }

        impl Protocol{
            pub fn get_default_from_string(s: String) -> Option<Self>{
                let lowered_case = s.to_lowercase();
                match lowered_case.as_str() {
                    $(
                        $name => Some(Protocol::$protocol),
                    )+
                    _ => None
                }
            }

            pub fn get_default_from_str(s: &str) -> Option<Self>{
                let lowered_case = s.to_lowercase();
                match lowered_case.as_str() {
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

impl_protocol!(
    HTTP, "http", 80,
    HTTPS, "https", 443,
    FTP, "ftp", 21
);

/// Create a URL prefix string.
pub fn create_prefix(protocol: Protocol, domain: &str, port: Option<u16>, path: Option<&str>) -> String {
    let protocol_name = protocol.get_name();

    let mut prefix = format!("{}://{}", protocol_name, domain);

    if let Some(port) = port {
        let protocol_port = protocol.get_default_port();
        if port != protocol_port {
            prefix.push(':');
            prefix.push_str(&port.to_string());
        }
    }

    if let Some(path) = path {
        if !path.starts_with("/") {
            prefix.push('/');
        }
        prefix.push_str(&path);
    }

    prefix
}

#[cfg(feature = "validator")]
/// Create a safe URL prefix string.
pub fn create_prefix_with_validated_domain(protocol: Protocol, domain: &Domain, path: Option<&str>) -> String {
    let port = domain.get_port();

    let domain = domain.get_full_domain_without_port();

    create_prefix(protocol, domain, port, path)
}

#[cfg(feature = "validator")]
/// Create a safe URL prefix string.
pub fn create_prefix_with_validated_ipv4(protocol: Protocol, ipv4: &IPv4, path: Option<&str>) -> String {
    let port = ipv4.get_port();

    let ipv4 = ipv4.get_full_ipv4_without_port();

    create_prefix(protocol, ipv4, port, path)
}

#[cfg(feature = "validator")]
/// Create a safe URL prefix string.
pub fn create_prefix_with_validated_ipv6(protocol: Protocol, ipv6: &IPv6, path: Option<&str>) -> String {
    let port = ipv6.get_port();

    let ipv6 = ipv6.get_full_ipv6_without_port();

    let ipv6 = format!("[{}]", ipv6);

    create_prefix(protocol, &ipv6, port, path)
}

#[cfg(feature = "validator")]
/// Create a safe URL prefix string.
pub fn create_prefix_with_validated_host(protocol: Protocol, host: &Host, path: Option<&str>) -> String {
    match host {
        Host::Domain(domain) => {
            create_prefix_with_validated_domain(protocol, domain, path)
        }
        Host::IPv4(ipv4) => {
            create_prefix_with_validated_ipv4(protocol, ipv4, path)
        }
        Host::IPv6(ipv6) => {
            create_prefix_with_validated_ipv6(protocol, ipv6, path)
        }
    }
}

#[cfg(feature = "validator")]
/// Create a safe URL prefix string.
pub fn create_prefix_with_validated_http_url(http_url: &HttpUrlLocalableWithProtocol) -> String {
    let protocol = if http_url.is_https() {
        Protocol::HTTPS
    } else {
        Protocol::HTTP
    };

    create_prefix_with_validated_host(protocol, http_url.get_host(), http_url.get_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_prefix_lv1_1() {
        let prefix = create_prefix(Protocol::HTTP, "magiclen.org", None, None);

        assert_eq!("http://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv1_2() {
        let prefix = create_prefix(Protocol::HTTPS, "magiclen.org", None, None);

        assert_eq!("https://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv1_3() {
        let prefix = create_prefix(Protocol::FTP, "magiclen.org", None, None);

        assert_eq!("ftp://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv2_1() {
        let prefix = create_prefix(Protocol::HTTP, "magiclen.org", Some(8000), None);

        assert_eq!("http://magiclen.org:8000", prefix);
    }

    #[test]
    fn create_prefix_lv2_2() {
        let prefix = create_prefix(Protocol::HTTPS, "magiclen.org", Some(8100), None);

        assert_eq!("https://magiclen.org:8100", prefix);
    }

    #[test]
    fn create_prefix_lv2_3() {
        let prefix = create_prefix(Protocol::FTP, "magiclen.org", Some(8200), None);

        assert_eq!("ftp://magiclen.org:8200", prefix);
    }

    #[test]
    fn create_prefix_lv3_1() {
        let prefix = create_prefix(Protocol::HTTP, "magiclen.org", Some(80), None);

        assert_eq!("http://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv3_2() {
        let prefix = create_prefix(Protocol::HTTPS, "magiclen.org", Some(443), None);

        assert_eq!("https://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv3_3() {
        let prefix = create_prefix(Protocol::FTP, "magiclen.org", Some(21), None);

        assert_eq!("ftp://magiclen.org", prefix);
    }

    #[test]
    fn create_prefix_lv4_1() {
        let prefix = create_prefix(Protocol::HTTP, "magiclen.org", Some(80), Some("url-prefix"));

        assert_eq!("http://magiclen.org/url-prefix", prefix);
    }

    #[test]
    fn create_prefix_lv4_2() {
        let prefix = create_prefix(Protocol::HTTPS, "magiclen.org", Some(8100), Some("url-prefix"));

        assert_eq!("https://magiclen.org:8100/url-prefix", prefix);
    }

    #[cfg(feature = "validator")]
    #[test]
    fn create_prefix_with_validated_domain_lv4() {
        let user_input = validators::domain::DomainLocalhostableWithPort::from_str("magiclen.org:443").unwrap();

        let prefix = create_prefix_with_validated_domain(Protocol::HTTPS, user_input.as_domain(), Some("url-prefix"));

        assert_eq!("https://magiclen.org/url-prefix", prefix);
    }

    #[cfg(feature = "validator")]
    #[test]
    fn create_prefix_with_validated_ipv4_lv4() {
        let user_input = validators::ipv4::IPv4LocalableWithPort::from_str("127.0.0.1:443").unwrap();

        let prefix = create_prefix_with_validated_ipv4(Protocol::HTTPS, user_input.as_ipv4(), Some("url-prefix"));

        assert_eq!("https://127.0.0.1/url-prefix", prefix);
    }

    #[cfg(feature = "validator")]
    #[test]
    fn create_prefix_with_validated_ipv6_lv4() {
        let user_input = validators::ipv6::IPv6LocalableWithPort::from_str("[0000:0000:0000:0000:0000:0000:370:7348]:443").unwrap();

        let prefix = create_prefix_with_validated_ipv6(Protocol::HTTPS, user_input.as_ipv6(), Some("url-prefix"));

        assert_eq!("https://[0000:0000:0000:0000:0000:0000:370:7348]/url-prefix", prefix);
    }

    #[cfg(feature = "validator")]
    #[test]
    fn create_prefix_with_validated_host_lv4() {
        let user_input = validators::host::HostLocalable::from_str("127.0.0.1:443").unwrap();

        let prefix = create_prefix_with_validated_host(Protocol::HTTPS, user_input.as_host(), Some("url-prefix"));

        assert_eq!("https://127.0.0.1/url-prefix", prefix);
    }

    #[cfg(feature = "validator")]
    #[test]
    fn create_prefix_with_validated_http_url_lv4() {
        let user_input = HttpUrlLocalableWithProtocol::from_str("https://127.0.0.1:443/url-prefix").unwrap();

        let prefix = create_prefix_with_validated_http_url(&user_input);

        assert_eq!("https://127.0.0.1/url-prefix", prefix);
    }
}
