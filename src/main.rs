use spacetraders::components::{Agent, State};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let agent = Agent::new(&client).await.expect("failed to get agent");

    iced::application(move ||
        State { agent: agent.clone() }, Agent::update, Agent::view)
        .run()
        .expect("failed to run app")
}