use std::collections::HashMap;

use reqwest::{Client, Response};

#[allow(dead_code)]
pub struct ReporterWithResponse {
    pub client: Client,
}

impl ReporterWithResponse {
    
    #[allow(dead_code)]
    pub async fn report(self, uri: &str,hm: &HashMap::<String, Option<String>>) ->  Result<Response, reqwest::Error>{ 
        self.client.post(uri)
        .json(hm)
        .send().await
        
    }

    #[allow(dead_code)]
    pub async fn report_msg(self, uri: &str,hm: &HashMap::<String, Option<String>>, msg: &str) ->  Result<Response, reqwest::Error>{ 
        match self.client.post(uri)
        .json(hm)
        .send().await {
            Ok(v) => {
                println!("REPORTED TO : {} : {} : {} ", uri, v.status(), msg);
                Ok(v)
            },
            Err(e) => {
                println!("ERROR : {} :  {} : {}",uri, msg, e.to_string());
                Err(e)
            }
        }
    }

    
    #[allow(dead_code)]
    pub fn new(client: Client) -> Self { 
        Self { 
            client
        }
    }
}