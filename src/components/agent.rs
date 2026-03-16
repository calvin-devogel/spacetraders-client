use iced::{widget::{column, container, text}, Element, Task, Length};
use spacetraders_sdk::apis::{agents_api::get_agent, configuration::Configuration};

#[derive(Debug, Clone)]
pub enum Message {}

pub struct State {
    pub agent: Agent
}

#[derive(Default, serde::Deserialize, Debug, Clone)]
pub struct Agent {
    #[allow(unused)]
    account_id: Option<String>,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: i32,
}

impl From<Box<spacetraders_sdk::models::Agent>> for Agent {
    fn from(sdk_agent: Box<spacetraders_sdk::models::Agent>) -> Self {
        Agent {
            account_id: sdk_agent.account_id,
            symbol: sdk_agent.symbol,
            headquarters: sdk_agent.headquarters,
            credits: sdk_agent.credits,
            starting_faction: sdk_agent.starting_faction,
            ship_count: sdk_agent.ship_count
        }
    }
}

impl Agent {
    pub async fn new(config: &Configuration) -> anyhow::Result<Agent> {
        let agent_request = get_agent(config, "LIGHT_BASIN").await?;
        let agent = agent_request.data.into();

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

