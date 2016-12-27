use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::fs;
use idna::domain_to_ascii;
use std::ascii::AsciiExt;
extern crate regex;
use regex::Regex;
extern crate fern;

use hyper::Client;
use std::io::prelude::*;
use hyper;
static PUBLIC_SUFFIX_LIST_URLS: &'static [&'static str] =
    &["https://publicsuffix.org/list/public_suffix_list.dat",
      "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat"];
const SCHEME_RE: &'static str =
    r"^([abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-.]+:)?//";
const IP_RE: &'static str = r"(\d{1,3}\.){3}\d{1,3}";



pub struct TldExtract {
    cache_file_default: String,
    cache_file: String,
    tld_list: Vec<String>,
}



impl TldExtract {
    pub fn new(cache_file: Option<String>, fetch: Option<bool>) -> TldExtract {
        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.push(".tld_set");

        let cache_file = match cache_file {
            Some(expr) => expr,
            None => path.to_str().unwrap().to_string(),
        };

        let mut tldext = TldExtract {
            cache_file_default: path.to_str().unwrap().to_string(),
            cache_file: cache_file,
            tld_list: Vec::new(),
        };

        match fetch {
            Some(fetch) => if fetch {},
            None => {}
        };

        if path.exists() && fs::metadata(path.clone()).unwrap().len() > 0 {
            println!("Found cache file at location {:?}", path);
            let mut f = File::open(path).unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s);
            tldext.tld_list = s.split("\n").map(|x| x.clone().to_string()).collect();

        } else {
            tldext.fetch();
        }
        return tldext;
    }

    pub fn extract(self, url: &str) -> (String, String, String) {
        let re = Regex::new(SCHEME_RE).unwrap();
        // let before = "https://user:pass@xn--xample-hva.xn--wgbl6a:8000/dash/dash/shit.\
        //               html#link1";
        // let before = "http://mak.my";
        let after = re.replace_all(url, "");
        let netloc = after.split("/").collect::<Vec<&str>>()[0]
                            .split("?")
                            .collect::<Vec<&str>>()[0]
                        .split("#")
                        .collect::<Vec<&str>>()[0]
                    .rsplit("@")
                    .collect::<Vec<&str>>()[0]
                .split(":")
                .collect::<Vec<&str>>()[0]
            .trim_right_matches('.');
        let labels = netloc.split(".");
        let mut translations: Vec<String> = Vec::new();
        for label in labels {
            if label == "" {
                continue;
            }
            let mut translation: String = "".to_string();
            if label.starts_with("xn--") && label.is_ascii() {
                translation = domain_to_ascii(label).unwrap();
            } else {
                translation = label.to_string();
            }
            translations.push(translation.to_lowercase());
        }

        let labels = netloc.split(".").collect::<Vec<&str>>();
        let suffix_index = self.find_suffix(translations);
        let regestered_domain = labels[0..suffix_index].join(".");
        let suffix = labels[suffix_index..labels.len()].join(".");
        let re_ip = Regex::new(IP_RE).unwrap();
        if suffix.is_empty() && !netloc.is_empty() && re_ip.is_match(netloc) {
            return ("".to_string(), netloc.to_string(), "".to_string());
        }
        let d = regestered_domain.split(".").collect::<Vec<&str>>();
        let subdomain = d[0..d.len() - 1].join(".");
        let domain = d[d.len() - 1..d.len()].join(".");
        return (subdomain, domain, suffix);
    }
    pub fn fetch(&mut self) {
        let client = Client::new();

        for url in PUBLIC_SUFFIX_LIST_URLS {
            println!("Connection to {:?}", url);
            let mut res = client.get(*url).send().unwrap();
            if res.status == hyper::Ok {
                println!("Downloaded public suffix list form {:?}", url);
                let mut body = String::new();
                res.read_to_string(&mut body);
                let tlds = body.split("\n");
                print!("Writing valid none emptiy suffixes to file {:?}",
                       self.cache_file_default);
                let mut f = match File::create(self.cache_file_default.clone()) {
                    Err(e) => {
                        println!("Couldn't open/create file {:?}", self.cache_file_default);
                        return;
                    }
                    Ok(f) => f,
                };
                for tld in tlds {
                    if tld.starts_with("//") || tld.is_empty() {
                        continue;
                    }
                    self.tld_list.push(tld.to_string());
                    f.write_all(format!("{}\n", tld).as_bytes());
                }
                break;
            }
        }

    }
    pub fn find_suffix(self, lowerSPL: Vec<String>) -> usize {
        for i in 0..lowerSPL.len() {
            let maybe_tld = lowerSPL[i..lowerSPL.len()].join(".");
            let exception_tld = format!("!{}", maybe_tld);
            if self.tld_list.contains(&exception_tld) {
                return i + 1;
            }
            if self.tld_list.contains(&maybe_tld) {
                return i;
            }
            let wildcard_tld = format!("*.{}", lowerSPL[(i + 1)..lowerSPL.len()].join("."));
            if self.tld_list.contains(&wildcard_tld) {
                return i;
            }
        }
        lowerSPL.len()
    }
}
