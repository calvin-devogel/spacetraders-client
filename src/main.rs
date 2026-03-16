use std::fs;
use std::fmt;
use iced::{widget::{column, container, text}, Element, Task, Length};

#[derive(serde::Deserialize)]
struct ApiResponse<T> {
    data: T,
}

#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut State, _message: Message) -> Task<Message> {
    Task::none()
}

struct State {
    agent: Agent
}

#[derive(Default, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Agent {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: i32
}

impl Agent {
    pub async fn init(client: &reqwest::Client) -> anyhow::Result<Agent> {
        let agent_token = fs::read_to_string(".secret")?;
        let response = client
            .get("https://api.spacetraders.io/v2/my/agent")
            .bearer_auth(agent_token)
            .send()
            .await?;

        let agent = response.json::<ApiResponse<Agent>>().await?.data;

        Ok(agent)
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Agent Id: {}, Symbol: {}, Headquarters: {}, Credits: {}, Starting Faction: {}, Ship Count: {}",
            self.account_id, self.symbol, self.headquarters, self.credits, self.starting_faction, self.ship_count)
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let content = column![
        text(format!("Symbol:           {}", state.agent.symbol)),
        text(format!("Headquarters:     {}", state.agent.headquarters)),
        text(format!("Credits:          {}", state.agent.credits)),
        text(format!("Starting Faction: {}", state.agent.starting_faction)),
        text(format!("Fleet Size:       {}", state.agent.ship_count)),
    ]
    .spacing(8);

    container(content)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let agent = Agent::init(&client).await.expect("failed to get agent");

    iced::application(move || State { agent: agent.clone() }, update, view).run().expect("failed to run app")
}