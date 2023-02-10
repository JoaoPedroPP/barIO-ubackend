use std::env;
use bytes::Bytes;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    access_token: String,
}

struct RequestToken {
    apikey: String,
    grant_type: String
}

fn get_token() -> Result<Token, reqwest::Error> {
    let url = env::var("IBM_TOKEN_URL").expect("IBM_TOKEN_URL not set");
    let apikey = env::var("IBM_APIKEY").expect("IBM_APIKEY not set");
    let mut payload = HashMap::new();
    payload.insert("apikey", apikey);
    payload.insert("grant_type", "urn:ibm:params:oauth:grant-type:apikey".to_string());
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .form(&payload)
        .send();
    
    match resp {
        Ok(token) => token.json::<Token>(),
        Err(error) => Err(error),
    }
}

pub fn upload_image(key: String, payload: Bytes) -> Result<(), String> {
    match get_token() {
        Ok(token) => {
            let cos = env::var("COS_URL").expect("COS_URL not set");
            let bucket = env::var("COS_BUCKET").expect("COS_BUCKET not set");
            let url = format!("{url}/{bucket}/{key}", url=cos, bucket=bucket, key=key);
            let client = reqwest::blocking::Client::new();
            let resp = client.put(url)
                .bearer_auth(token.access_token)
                .body(payload)
                .send();
            
            match resp {
                Ok(_data) => Ok(()),
                Err(error) => {
                    println!("API request not successfull: {}", error);
                    Err("Response COS error".to_string())
                },
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
            Err("Response token error".to_string())
        },
    }
}