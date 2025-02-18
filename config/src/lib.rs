use ko_protocol::types::config::KoConfig;
use ko_protocol::KoResult;

mod error;
use error::ConfigError;

pub fn load_file(path: &str) -> KoResult<KoConfig> {
    let file = std::fs::read_to_string(path)
        .map_err(|err| ConfigError::ErrorLoadingConfig(path.into(), err.to_string()))?;
    let config: KoConfig = toml::from_str(file.as_str())
        .map_err(|err| ConfigError::ErrorLoadingConfig(path.into(), err.to_string()))?;
    Ok(config)
}

#[test]
fn test_load_file() {
    let config = load_file("./src/config.toml").expect("load config");
    println!("config = {:?}", config);
}
