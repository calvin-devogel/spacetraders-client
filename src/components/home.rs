use iced::{
    Element, Length, Task, Color,
    widget::{
        Column, column, container, row, scrollable, text, button, stack, opaque,
        mouse_area, Space
    },
};

use spacetraders_sdk::apis::{
    configuration::Configuration, contracts_api::get_contracts, fleet_api::get_my_ships,
    systems_api::get_system_waypoints,
};

use spacetraders_sdk::models::{Contract, Ship, Waypoint};

use crate::components::agent;

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    ShipsLoaded(Result<Vec<Ship>, String>),
    ContractsLoaded(Result<Vec<Contract>, String>),
    WaypointsLoaded(Result<Vec<Waypoint>, String>),
    SelectContract(usize),
    CloseOverlay,
}

pub struct Home {
    pub config: Configuration,
    pub agent: agent::Agent,
    pub ships: Vec<Ship>,
    pub contracts: Vec<Contract>,
    pub waypoints: Vec<Waypoint>,
    pub error: Option<String>,
    pub selected_contract: Option<usize>,
}

impl Home {
    pub fn new(config: Configuration, agent: agent::Agent) -> (Self, Task<Message>) {
        let state = Home {
            config,
            agent,
            ships: vec![],
            contracts: vec![],
            waypoints: vec![],
            error: None,
            selected_contract: None,
        };
        (state, Task::done(Message::Init))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {
                let config_ships = self.config.clone();
                let config_contract = self.config.clone();
                let config_waypoints = self.config.clone();
                let system = headquarters_to_system(&self.agent.headquarters);

                Task::batch([
                    Task::perform(
                        async move {
                            get_my_ships(&config_ships, None, None)
                                .await
                                .map(|result| result.data)
                                .map_err(|e| e.to_string())
                        },
                        Message::ShipsLoaded,
                    ),
                    Task::perform(
                        async move {
                            get_contracts(&config_contract, None, None)
                                .await
                                .map(|result| result.data)
                                .map_err(|e| e.to_string())
                        },
                        Message::ContractsLoaded,
                    ),
                    Task::perform(
                        async move {
                            get_system_waypoints(
                                &config_waypoints,
                                &system,
                                None,
                                Some(20),
                                None,
                                None,
                            )
                            .await
                            .map(|result| result.data)
                            .map_err(|e| e.to_string())
                        },
                        Message::WaypointsLoaded,
                    ),
                ])
            }

            Message::ShipsLoaded(result) => {
                match result {
                    Ok(ships) => self.ships = ships,
                    Err(e) => self.error = Some(e),
                }
                Task::none()
            }

            Message::ContractsLoaded(result) => {
                match result {
                    Ok(contracts) => self.contracts = contracts,
                    Err(e) => self.error = Some(e),
                }
                Task::none()
            }

            Message::WaypointsLoaded(result) => {
                match result {
                    Ok(waypoints) => self.waypoints = waypoints,
                    Err(e) => self.error = Some(e),
                }
                Task::none()
            }

            Message::SelectContract(i) => {
                self.selected_contract = Some(i);
                Task::none()
            }

            Message::CloseOverlay => {
                self.selected_contract = None;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let agent_panel = container(
            column![
                text("[ Agent ]").size(18),
                text(format!("Symbol: {}", self.agent.symbol)),
                text(format!("HQ: {}", self.agent.headquarters)),
                text(format!("Credits: {:>12}", self.agent.credits)),
                text(format!("Faction: {}", self.agent.starting_faction)),
                text(format!("Fleet Size: {}", self.agent.ship_count)),
            ]
            .spacing(6),
        )
        .padding(16);

        let ships_panel = container(
            column![
                text("[ Fleet ]").size(18),
                scrollable(ship_list(&self.ships)),
            ]
            .spacing(6),
        )
        .padding(16)
        .width(Length::Fill);

        let contracts_panel = container(
            column![
                text("[ Contracts ]").size(18),
                scrollable(contract_list(&self.contracts)),
            ]
            .spacing(6),
        )
        .padding(16)
        .width(Length::Fill);

        let waypoints_panel = container(
            column![
                text(format!(
                    "[ Waypoints - {} ]",
                    headquarters_to_system(&self.agent.headquarters)
                ))
                .size(18),
                scrollable(waypoint_list(&self.waypoints)),
            ]
            .spacing(6),
        )
        .padding(16)
        .width(Length::Fill);

        let content = column![
            agent_panel,
            row![ships_panel, contracts_panel, waypoints_panel].spacing(8),
        ]
        .spacing(12);

        if let Some(err) = &self.error {
            column![text(format!("Error: {err}")), content]
                .spacing(8)
                .into()
        } else {
            if let Some(idx) = self.selected_contract {
                let contract = &self.contracts[idx];
                stack![
                    content,
                    opaque(
                        mouse_area(
                            container(Space::new())
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .style(|_theme| container::Style {
                                    background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                                    ..Default::default()
                                })
                        ).on_press(Message::CloseOverlay)
                    ),
                    opaque(
                        container(contract_detail(contract))
                            .center(Length::Fill)
                    )
                ]
                .into()
            } else {
                content.into()
            }
        }
    }
}

fn ship_list(ships: &[Ship]) -> Column<'_, Message> {
    ships.iter().fold(Column::new().spacing(8), |col, ship| {
        col.push(text(format!(
            "{}", ship.symbol
        )).size(16)).push(text(format!("{} @ {}",
        ship.registration.role, ship.nav.waypoint_symbol)).size(12))
    })
}

fn contract_list(contracts: &[Contract]) -> Column<'_, Message> {
    contracts.iter().enumerate().fold(Column::new().spacing(8), |col, (i, c) | {
        let status = if c.fulfilled { "fullfilled" } else if c.accepted { "active" } else { "avaiable" };
        col.push(
            button(text(format!("[{}] {} - {:?}", status, c.faction_symbol, c.r#type)))
                .on_press(Message::SelectContract(i))
                .style(button::text)
        )
    })
}

fn waypoint_list(waypoints: &[Waypoint]) -> Column<'_, Message> {
    waypoints.iter().fold(Column::new().spacing(4), |col, w| {
        col.push(text(format!("{} ({:?})", w.symbol, w.r#type)))
    })
}

fn headquarters_to_system(hq: &str) -> String {
    hq.rsplitn(2, '-').last().unwrap_or(hq).to_string()
}

fn contract_detail(contract: &Contract) -> Column<'_, Message> {
    let mut col = column![
        text(format!("Faction: {}", contract.faction_symbol)),
        text(format!("Type: {:?}", contract.r#type)),
        text(format!("Status: {}", if contract.fulfilled { "Fulfilled" } else if contract.accepted { "Active" } else { "Available" })),
        text(format!("Deadline: {}", contract.terms.deadline)),
        text(format!("On Accept:  {} credits", contract.terms.payment.on_accepted)),
        text(format!("On Fulfill: {} credits", contract.terms.payment.on_fulfilled)),
    ]
    .spacing(6);

    if let Some(deliveries) = &contract.terms.deliver {
        col = col.push(text("Deliveries: ").size(14));
        for d in deliveries {
            col = col.push(text(format!(
                "   {} → {} ({}/{})",
                d.trade_symbol, d.destination_symbol, d.units_fulfilled, d.units_required
            )));
        }
    }
    col.push(button(text("Close")).on_press(Message::CloseOverlay))
}