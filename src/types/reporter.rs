use std::collections::HashMap;

use reqwest::{Client, Response};
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReporterWithResponse {
    pub client: Client,
}

impl ReporterWithResponse {
    #[allow(dead_code)]
    pub async fn report(
        self,
        uri: &str,
        hm: &HashMap<String, Option<String>>,
    ) -> Result<Response, reqwest::Error> {
        self.client.post(uri).json(hm).send().await
    }

    #[allow(dead_code)]
    pub async fn report_msg(
        self,
        uri: &str,
        hm: &HashMap<String, Option<String>>,
        msg: &str,
    ) -> Result<Response, reqwest::Error> {
        match self.client.post(uri).json(hm).send().await {
            Ok(v) => {
                println!("REPORTED TO : {} : {} : {} ", uri, v.status(), msg);
                Ok(v)
            }
            Err(e) => {
                println!("ERROR : {} :  {} : {}", uri, msg, e.to_string());
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn report_msg_ext<T>(
        self,
        uri: &str,
        body: &T,
        msg: &str,
    ) -> Result<Response, reqwest::Error>
    where
        T: Serialize + std::fmt::Debug + Clone,
    {
        match self.client.post(uri).json(body).send().await {
            Ok(v) => {
                println!("REPORTED TO : {} : {} : {} ", uri, v.status(), msg);
                Ok(v)
            }
            Err(e) => {
                println!("ERROR : {} :  {} : {}", uri, msg, e.to_string());
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
