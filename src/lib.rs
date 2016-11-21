extern crate hyper;
extern crate regex;
extern crate idna;
pub mod tldextract;
use tldextract::extract::TldExtract;

#[test]
fn multi_level_sub_domain_two_part_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://news.forums.bbc.co.uk");

    assert_eq!(domain, "bbc".to_owned());
    assert_eq!(subdomain, "news.forums".to_owned());
    assert_eq!(suffix, "co.uk".to_owned());
}

#[test]
fn single_level_sub_domain_simple_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://www.google.com");

    assert_eq!(domain, "google".to_owned());
    assert_eq!(subdomain, "www".to_owned());
    assert_eq!(suffix, "com".to_owned());
}

#[test]
fn no_sub_domain_simple_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://google.com");

    assert_eq!(domain, "google".to_owned());
    assert_eq!(subdomain, "".to_owned());
    assert_eq!(suffix, "com".to_owned());
}

#[test]
fn https_no_sub_domain_simple_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("https://google.com");

    assert_eq!(domain, "google".to_owned());
    assert_eq!(subdomain, "".to_owned());
    assert_eq!(suffix, "com".to_owned());
}


#[test]
fn no_sub_domain_not_a_valid_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://google.notavalidsuffix");

    assert_eq!(domain, "notavalidsuffix".to_owned());
    assert_eq!(subdomain, "google".to_owned());
    assert_eq!(suffix, "".to_owned());
}

#[test]
fn ip_url() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://127.0.0.1:8080/deployed/");

    assert_eq!(domain, "127.0.0.1".to_owned());
    assert_eq!(subdomain, "".to_owned());
    assert_eq!(suffix, "".to_owned());
}

#[test]
fn http_single_level_sub_domain_two_part_suffix() {
    let mut TldExtractor = TldExtract::new(None, None);
    let (subdomain, domain, suffix) = TldExtractor.extract("http://www.worldbank.org.kg/");

    assert_eq!(domain, "worldbank".to_owned());
    assert_eq!(subdomain, "www".to_owned());
    assert_eq!(suffix, "org.kg".to_owned());
}
