use std::{fs::File, io::Read, sync::Arc};

use serde::Deserialize;

pub fn load_config<T: for<'a> Deserialize<'a>>(env_name: &str) -> Arc<T> {
    let config_path = std::env::var(env_name).expect(
        &format!("{} environment variable not set", env_name));
    let message = format!("Failed to open config file : {}", config_path);
    let mut config_file = File::open(config_path).expect(&message);
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data).expect("Failed to read config file");

    let config: T = serde_yaml::from_str::<T>(&config_data)
        .expect("Failed to parse config file");

    Arc::new(config)
}