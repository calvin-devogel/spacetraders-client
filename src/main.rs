use spacetraders::components::{agent::Agent, configuration::get_configuration, home::Home};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("failed to get configuration");
    let agent = Agent::new(&config).await.expect("failed to get agent");

    iced::application(
        move || Home::new(config.clone(), agent.clone()),
        Home::update,
        Home::view,
    )
    .run()
    .expect("failed to run app")
}
