use reqwest::Certificate;
use types::outbound::NamedCertString;

pub mod types;
pub mod utils;

pub fn verify_certs(certs: &Vec<NamedCertString>) -> bool { 

    for ncs in certs.iter() { 

        let NamedCertString{ name, pem }   = ncs;

        match Certificate::from_pem(pem.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("invalid certificate : name = {} , error =  {} ", name, e);
                return false;
            }
        }
    }
    true
}

