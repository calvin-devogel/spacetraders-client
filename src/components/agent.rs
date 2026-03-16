use std::fs;
use iced::{widget::{column, container, text}, Element, Task, Length};

#[derive(serde::Deserialize)]
struct ApiResponse<T> {
    data: T,
}

#[derive(Debug, Clone)]
pub enum Message {}

pub struct State {
    pub agent: Agent
}

#[derive(Default, serde::Deserialize, Debug, Clone)]
pub struct Agent {
    #[allow(unused)]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: i32,
}

impl Agent {
    pub async fn new(client: &reqwest::Client) -> anyhow::Result<Agent> {
        let agent_token = fs::read_to_string(".secret")?;

        let response = client
            .get("https://api.spacetraders.io/v2/my/agent")
            .bearer_auth(agent_token)
            .send()
            .await?;

        let agent = response.json::<ApiResponse<Agent>>().await?.data;

        Ok(agent)
    }

    pub fn update(_state: &mut State, _message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(state: &State) -> Element<'_, Message> {
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
}

