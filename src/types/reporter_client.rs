
use std::time::Duration;

use base64::prelude::*;
use reqwest::{header::{HeaderMap, AUTHORIZATION}, Certificate, Client,Identity};

use super::basic_auth::BasicAuth;


pub struct ReporterClientConfig { 
    pub accept_invalid_certs: bool, 
    pub basic_auth: Option<BasicAuth>,
    pub timeout: Duration,
    pub headers: HeaderMap,
    pub cacerts: Vec<Certificate>,
    pub identity: Option<Identity>
}

impl ReporterClientConfig { 
    
    fn add_cacert(&mut self, data: &[u8]) { 
        let cert: Certificate = reqwest::Certificate::from_pem(data).unwrap();
        self.cacerts.push(cert);
    }

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

    pub fn client(mut self) -> Client {

        let mut builder = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .timeout(self.timeout);

        for cert in &self.cacerts { 
            builder = builder.add_root_certificate(cert.clone())
        }

        if self.identity.is_some() {
            builder = builder.identity(self.identity.clone().unwrap());
        }
        
        if self.basic_auth.is_some() { 
            let basic_auth = self.basic_auth.clone().unwrap();
            self.headers.extend(
                Self::basic_auth_header(
                    basic_auth.username,
                    basic_auth.password)
            );
        }

        builder = builder.default_headers(self.headers.clone());

        builder.build().unwrap()
    
        
    }

}