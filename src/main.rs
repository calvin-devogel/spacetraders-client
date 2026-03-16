use spacetraders::components::{Agent, State, get_configuration};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("failed to get configuration");
    let agent = Agent::new(&config).await.expect("failed to get agent");

    iced::application(move ||
        State { agent: agent.clone() }, Agent::update, Agent::view)
        .run()
        .expect("failed to run app")
}