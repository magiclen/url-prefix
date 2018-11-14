#![cfg(feature = "validator")]

extern crate url_prefix;
extern crate validators;

use url_prefix::Protocol;

use validators::http_url::HttpUrlLocalableWithProtocol;
use validators::http_ftp_url::HttpFtpUrlLocalableWithProtocol;

#[test]
fn create_prefix_with_validated_domain_lv4() {
    let user_input = validators::domain::DomainLocalhostableWithPort::from_str("magiclen.org:443").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_domain(Protocol::HTTPS, user_input.as_domain(), Some("url-prefix"));

    assert_eq!("https://magiclen.org/url-prefix", prefix);
}

#[test]
fn create_prefix_with_validated_ipv4_lv4() {
    let user_input = validators::ipv4::IPv4LocalableWithPort::from_str("127.0.0.1:443").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_ipv4(Protocol::HTTPS, user_input.as_ipv4(), Some("url-prefix"));

    assert_eq!("https://127.0.0.1/url-prefix", prefix);
}

#[test]
fn create_prefix_with_validated_ipv6_lv4() {
    let user_input = validators::ipv6::IPv6LocalableWithPort::from_str("[0000:0000:0000:0000:0000:0000:370:7348]:443").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_ipv6(Protocol::HTTPS, user_input.as_ipv6(), Some("url-prefix"));

    assert_eq!("https://[0000:0000:0000:0000:0000:0000:370:7348]/url-prefix", prefix);
}

#[test]
fn create_prefix_with_validated_host_lv4() {
    let user_input = validators::host::HostLocalable::from_str("127.0.0.1:443").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_host(Protocol::HTTPS, user_input.as_host(), Some("url-prefix"));

    assert_eq!("https://127.0.0.1/url-prefix", prefix);
}

#[test]
fn create_prefix_with_validated_http_url_lv4() {
    let user_input = HttpUrlLocalableWithProtocol::from_str("https://127.0.0.1:443/url-prefix").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_http_url(&user_input);

    assert_eq!("https://127.0.0.1/url-prefix", prefix);
}

#[test]
fn create_prefix_with_validated_http_ftp_url_lv4() {
    let user_input = HttpFtpUrlLocalableWithProtocol::from_str("ftp://127.0.0.1:21/url-prefix").unwrap();

    let prefix = url_prefix::create_prefix_with_validated_http_ftp_url(&user_input);

    assert_eq!("ftp://127.0.0.1/url-prefix", prefix);
}