use spacetraders_sdk::apis::configuration::Configuration;
use std::fs;

pub fn get_configuration() -> Result<Configuration, anyhow::Error> {
    let agent_token = fs::read_to_string(".secret")?;

    let configuration = Configuration {
        bearer_access_token: Some(agent_token),
        ..Default::default()
    };

    Ok(configuration)
}
