use serde::Deserialize;
use std::time::Duration;
use base64::prelude::*;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Certificate, Client,Identity, Proxy};

use super::basic_auth::BasicAuth;

#[derive(Debug)]
pub struct NamedCert {
    pub name: String, 
    pub cert: Certificate 
}


#[derive(Debug, Deserialize)]
pub struct NamedCertString {
    pub name: String, 
    pub pem: String
}

#[derive(Debug, Deserialize)]
pub struct SmscOutboundEndpoint {
    pub url: String, 
    pub timeout_ms: u64, 
    pub pem_cacerts: Vec<NamedCertString>,
    pub username: String,
    pub password: String,
    pub dlt_entity: String,
    pub identity: Option<String>,
    #[serde(default = "default_useragent")]
    pub useragent: String,
    pub accept_invalid_certs: bool,
    pub auth: Option<BasicAuth>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,

}

impl SmscOutboundEndpoint { 
    // fn add_cacert(&mut self, data: &[u8]) { 
    //     let cert: Certificate = reqwest::Certificate::from_pem(data).unwrap();
    //     self.cacerts.push(cert);
    // }

    fn basic_auth_header(username: String, password: String) -> HeaderMap{
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Basic {}", BASE64_STANDARD.encode(
                format!("{}:{}", username, password)
            )).parse().unwrap()
        );
        headers
    }

    pub fn client(&self) -> Client {

        let mut builder = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(Duration::from_millis(self.timeout_ms));

        for ncs in &self.pem_cacerts { 
            let NamedCertString {name, pem}  = ncs;
            let cert: Certificate = reqwest::Certificate::from_pem(pem.as_bytes()).unwrap();
            builder = builder.add_root_certificate(cert.clone())
        }

        if self.identity.is_some() {
            let identity: Identity = Identity::from_pem(self.identity.clone().unwrap().as_bytes()).unwrap();
            builder = builder.identity(identity);
        }

        let mut headers: HeaderMap = HeaderMap::new();
        
        if self.auth.is_some() { 
            let basic_auth = self.auth.clone().unwrap();
            headers.extend(
                Self::basic_auth_header(
                    basic_auth.username,
                    basic_auth.password)
            );
        }

        match ( self.url.starts_with("https"), self.https_proxy.is_some(), self.http_proxy.is_some()) {
            (true,true,_) =>  { 
                 match Proxy::https(self.https_proxy.as_ref().unwrap()) {
                    Ok(p) => {
                        builder = builder.proxy(p);
                    }
                    Err(e) => {
                        println!("invalid proxy settings for https_proxy: {}", e);
                    }
                }
            }, 
            (false,_,true) =>  { 
                match Proxy::https(self.http_proxy.as_ref().unwrap()) {
                    Ok(p) => {
                        builder = builder.proxy(p);
                    }
                    Err(e) => {
                        println!("invalid proxy settings for http_proxy: {}", e);
                    }
                }
            }, 
            (_,_,_) => { }
       }


        builder = builder.default_headers(headers);

        builder.build().unwrap()
    
        
    }
}






#[derive(Debug, Deserialize)]
pub struct OutboundEndpoint {
    pub url: String, 
    pub timeout_ms: u64, 
    pub pem_cacerts: Vec<NamedCertString>,
    pub auth: Option<BasicAuth>,
    pub identity: Option<String>,
    #[serde(default = "default_useragent")]
    pub useragent: String,
    pub accept_invalid_certs: bool,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
}

fn default_useragent() -> String {
    "voice-connect".to_string()
}

impl OutboundEndpoint { 
    // fn add_cacert(&mut self, data: &[u8]) { 
    //     let cert: Certificate = reqwest::Certificate::from_pem(data).unwrap();
    //     self.cacerts.push(cert);
    // }

    fn basic_auth_header(username: String, password: String) -> HeaderMap{
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Basic {}", BASE64_STANDARD.encode(
                format!("{}:{}", username, password)
            )).parse().unwrap()
        );
        headers
    }

    pub fn client(&self) -> Client {

        let mut builder = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(Duration::from_millis(self.timeout_ms));

        for ncs in &self.pem_cacerts { 
            let NamedCertString {name, pem}  = ncs;
            let cert: Certificate = reqwest::Certificate::from_pem(pem.as_bytes()).unwrap();
            builder = builder.add_root_certificate(cert.clone())
        }

        if self.identity.is_some() {
            let identity: Identity = Identity::from_pem(self.identity.clone().unwrap().as_bytes()).unwrap();
            builder = builder.identity(identity);
        }

        let mut headers: HeaderMap = HeaderMap::new();
        
        if self.auth.is_some() { 
            let basic_auth = self.auth.clone().unwrap();
            headers.extend(
                Self::basic_auth_header(
                    basic_auth.username,
                    basic_auth.password)
            );
        }

        
        match ( self.url.starts_with("https"), self.https_proxy.is_some(), self.http_proxy.is_some()) {
            (true,true,_) =>  { 
                 match Proxy::https(self.https_proxy.as_ref().unwrap()) {
                    Ok(p) => {
                        builder = builder.proxy(p);
                    }
                    Err(e) => {
                        println!("invalid proxy settings for https_proxy: {}", e);
                    }
                }
            }, 
            (false,_,true) =>  { 
                match Proxy::https(self.http_proxy.as_ref().unwrap()) {
                    Ok(p) => {
                        builder = builder.proxy(p);
                    }
                    Err(e) => {
                        println!("invalid proxy settings for http_proxy: {}", e);
                    }
                }
            }, 
            (_,_,_) => {

                builder = builder.no_proxy(p);
             }
       }

        builder = builder.default_headers(headers);

        builder.build().unwrap()
    
        
    }
}