
use std::collections::HashMap;

use onelib::{utils::load_config, verify_certs};
use reqwest::Certificate;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct NamedCertString(String, String);

#[derive(Debug)]
struct NamedCert(String, Certificate);

#[derive(Debug, Deserialize)]
struct Config {
    pub pem_cacerts: Vec<HashMap<String, String>>, 
    #[serde(skip)]
    pub cacerts: Vec<NamedCert>
}

#[tokio::main]
async fn main() {
    let config = load_config::<Config>("CONFIG_PATH");
    assert_eq!(config.pem_cacerts.len(),0);
    

}