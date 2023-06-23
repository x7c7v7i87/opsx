use std::sync::Arc;

use axum::{extract::Query, http::HeaderMap, Extension};

use crate::json::PayloadJson;
use crate::{hmac, payload::Payload, ps::Params,gitlab::Payload as GitlabPayload};

pub async fn index()-> &'static str {
    "cfg is null.."
}

pub async fn root(
    cfg: Extension<Arc<Vec<PayloadJson>>>,
    headers: HeaderMap,
    Query(params): Query<Params>,
    body: String,
) -> &'static str {
    if params.keyword.is_none() {
        return "Hello, World from Axum!";
    }
    let keys = params.keyword.unwrap();
    if cfg.len() != 0 {
        for v in cfg.iter() {
            if keys == v.name {
                if v.git_type == "github" {
                    github(headers, body, v.clone()).await;
                    return "ok";
                } else if v.git_type == "gitlab" {
                    gitlab(headers, body, v.clone()).await;
                    return "ok";
                } else {
                    return "fail..";
                }
            }
        }
    }
    "cfg is null.."
}

pub async fn github(headers: HeaderMap, body: String, cfg: PayloadJson) -> &'static str {
    let check_bl = hmac::github_check(headers.clone(), body.clone(), cfg.security_key.clone());
    if check_bl == false {
        return "check fail..";
    }
    // println!("cfg: {:?}", cfg);
    // println!("headers: {:?}", headers);
    // println!("body: {:?}", body);
    let payload = Payload::from_json(body.as_str());
    match payload {
        Ok(req) => {
            if req.ref_field.is_none() {
                return "is refe err";
            }
            let branch_string = format!("{}{}", "refs/heads/", cfg.git_branch);
            let branch_check = branch_string.as_str();

            let ref_field = req.ref_field.unwrap();

            if ref_field.as_str() != branch_check {
                return "is refe err";
            }

            tokio::task::spawn(Params::cmd(cfg.ext_script.clone()));
            "success.."
        }
        Err(e) => {
            println!("payload: {:?}", e);
            "fail.."
        }
    }
}


pub async fn gitlab(headers: HeaderMap, body: String, cfg: PayloadJson) -> &'static str {
    println!("cfg: {:?}", cfg);
    println!("headers: {:?}", headers);
    println!("body: {:?}", body);

    let check_bl = hmac::gitlab_check(headers.clone(), body.clone(), cfg.security_key.clone());
    if check_bl == false {
        return "check fail..";
    }
   
    let payload = GitlabPayload::from_json(body.as_str());
    match payload {
        Ok(req) => {
            if req.ref_field.is_none() {
                return "is refe err";
            }
            let branch_string = format!("{}{}", "refs/heads/", cfg.git_branch);
            let branch_check = branch_string.as_str();

            let ref_field = req.ref_field.unwrap();

            if ref_field.as_str() != branch_check {
                return "is refe err";
            }

            tokio::task::spawn(Params::cmd(cfg.ext_script.clone()));
            "success.."
        }
        Err(e) => {
            println!("payload: {:?}", e);
            "fail.."
        }
    }
}
