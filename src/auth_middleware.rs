use actix_web::dev::{ServiceRequest, RequestHead};
use actix_web::Error;
use hmac::Hmac;
use log::*;
use crate::jwt::verification;
use futures::TryFutureExt;

pub fn validator(
    req: &RequestHead
) -> bool {
    let q = req.headers().get("Authorization");
    println!("{:?}", q);
    match  q {
        Some(k) => {
            let k: Vec<&str> = k.to_str().unwrap().split(" ").collect();
            println!("{:?}", q);
            match k.get(1) {
                None => false,
                Some(s) => {
                    match verification(String::from(*s)) {
                        Some(_) => true,
                        None => false
                    }
                }
            }
        },
        None => false
    }
}

#[cfg(test)]
mod validator_test {
    use super::*;
    use std::cell::RefCell;
    use actix_web::http::HeaderMap;

    #[test]
    fn test_validator() {
        let mut header = HeaderMap::new();
        header.insert("Authorization".parse().unwrap(),
                      "Bearer eyJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiaG93YXJkZ2o5NCJ9.Grmk1t0AZ-GH0PD_c3IWltIaseqvnb3SHuNl-3V5tSU".parse().unwrap());
        let mut t = RequestHead::default();
        t.headers = header;
        assert!(validator(&t));
    }
}