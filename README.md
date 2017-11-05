# tldextract-rs
[![Travis](https://img.shields.io/travis/msmakhlouf/tldextract-rs.svg?style=flat-square)](https://travis-ci.org/msmakhlouf/tldextract-rs) [![license](https://img.shields.io/github/license/msmakhlouf/tldextract-rs.svg?style=flat-square)]()

A rustacean implementation of tldextract highly inspired by python's module [tldextract](https://github.com/john-kurkowski/tldextract) by [John Kurkowski](https://github.com/john-kurkowski).
A tool to accurately separate the TLD from the registered domain and subdomains of a URL, using the Public Suffix List.
This relys on the [Public Suffix List](http://www.publicsuffix.org/) to identify gTLDs and ccTLDs correctly as oppose to spliting the URL string by dots and guessing where the parts of interest to you are.

## Requirements

Rust 1.13.0 or later is required.

On OS X and Windows, you may need to install the openssl runtime and headers to get the rust-openssl dependency to build. Instructions for that can be found [here](https://github.com/sfackler/rust-openssl#building).

## Installation

tldextract is available on crates.io. To use tldextract in your Rust program built with Cargo, add it as a dependency as follows:
```toml
    [dependencies]
    tldextract = {"*"}
```

## Usage

Below is the simplest way to use tldextract:
```rust
    extern crate tldextract;
    use tldextract::tldextract::extract::TldExtract;
    
    fn main() {
        let url = "http://news.forums.bbc.co.uk";
        // Initiate a new TldExtract
        let tld_extractor = TldExtract::new(None, None);
        // call extract passing the URL string and get a tuple of results
        let (subdomain, domain, suffix) = tld_extractor.extract(url);
        println!("Subdomains: {:?}, Domain: {:?}, Suffix: {:?}",
                 subdomain,
                 domain,
                 suffix);
    }
```
