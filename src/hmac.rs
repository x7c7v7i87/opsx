use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use axum::http::HeaderMap;
use std::str;


type HmacSha256 = Hmac<Sha256>;

pub fn github_check(headers: HeaderMap, body: String,security_key:String) -> bool {
    let asx = headers
        .get("x-hub-signature-256")
        .unwrap()
        .to_str()
        .unwrap();
    let items: Vec<&str> = asx.split("=").collect();

    if items.len() != 2 && items[1].is_empty() {
        return false;
    }

    let signature = items[1];
    let signature = hex::decode(signature).unwrap();
    let security = security_key.as_str();
    let keys = security.as_bytes();

    let mut mac = HmacSha256::new_from_slice(keys).expect("HMAC can take key of any size");
    mac.update(&body.as_bytes());
    let check = mac.verify_slice(&signature);

    match check {
        Ok(()) => {
            println!("check ok");
            true
        }
        Err(e) => {
            println!("check: {:?}", e);
            false
        }
    }
}



pub fn gitlab_check(headers: HeaderMap, _body: String,security_key:String) -> bool {
    let asx = headers
        .get("x-gitlab-token")
        .unwrap()
        .to_str()
        .unwrap();

    let security = security_key.as_str();
    if asx == security {
        return true;
    }else{
        return false;
    }
}
