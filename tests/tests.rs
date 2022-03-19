use url_prefix::Protocol;

#[test]
fn create_prefix_lv1_1() {
    let prefix = url_prefix::create_prefix(Protocol::HTTP, "magiclen.org", None, None::<String>);

    assert_eq!("http://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv1_2() {
    let prefix = url_prefix::create_prefix(Protocol::HTTPS, "magiclen.org", None, None::<String>);

    assert_eq!("https://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv1_3() {
    let prefix = url_prefix::create_prefix(Protocol::FTP, "magiclen.org", None, None::<String>);

    assert_eq!("ftp://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv2_1() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTP, "magiclen.org", Some(8000), None::<String>);

    assert_eq!("http://magiclen.org:8000", prefix);
}

#[test]
fn create_prefix_lv2_2() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTPS, "magiclen.org", Some(8100), None::<String>);

    assert_eq!("https://magiclen.org:8100", prefix);
}

#[test]
fn create_prefix_lv2_3() {
    let prefix =
        url_prefix::create_prefix(Protocol::FTP, "magiclen.org", Some(8200), None::<String>);

    assert_eq!("ftp://magiclen.org:8200", prefix);
}

#[test]
fn create_prefix_lv3_1() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTP, "magiclen.org", Some(80), None::<String>);

    assert_eq!("http://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv3_2() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTPS, "magiclen.org", Some(443), None::<String>);

    assert_eq!("https://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv3_3() {
    let prefix = url_prefix::create_prefix(Protocol::FTP, "magiclen.org", Some(21), None::<String>);

    assert_eq!("ftp://magiclen.org", prefix);
}

#[test]
fn create_prefix_lv4_1() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTP, "magiclen.org", Some(80), Some("url-prefix"));

    assert_eq!("http://magiclen.org/url-prefix", prefix);
}

#[test]
fn create_prefix_lv4_2() {
    let prefix =
        url_prefix::create_prefix(Protocol::HTTPS, "magiclen.org", Some(8100), Some("url-prefix"));

    assert_eq!("https://magiclen.org:8100/url-prefix", prefix);
}
