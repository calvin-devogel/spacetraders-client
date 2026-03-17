use iced::{
    Element, Length, Task,
    alignment::Horizontal,
    widget::{
        Column, Space, button, column, container, mouse_area, opaque, row, scrollable, shader,
        stack, text,
    },
};

use spacetraders_sdk::apis::{
    configuration::Configuration, contracts_api::get_contracts, fleet_api::get_my_ships,
    systems_api::get_system_waypoints,
};

use spacetraders_sdk::models::{Contract, Ship, Waypoint};

use crate::components::{agent, crt_shader::CrtOverlay, theme};

const WAYPOINTS_PER_PAGE: usize = 8;

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    ShipsLoaded(Result<Vec<Ship>, String>),
    ContractsLoaded(Result<Vec<Contract>, String>),
    WaypointsLoaded(Result<Vec<Waypoint>, String>),
    SelectContract(usize),
    CloseOverlay,
    NextWaypointsPage,
    PrevWaypointsPage,
    Tick(std::time::Instant),
}

pub struct Home {
    pub config: Configuration,
    pub agent: agent::Agent,
    pub ships: Vec<Ship>,
    pub contracts: Vec<Contract>,
    pub waypoints: Vec<Waypoint>,
    pub error: Option<String>,
    pub selected_contract: Option<usize>,
    pub waypoints_page: usize,
    pub start_time: std::time::Instant,
    pub time: f32,
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
            waypoints_page: 0,
            start_time: std::time::Instant::now(),
            time: 0.0,
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

            Message::NextWaypointsPage => {
                let max_page = self.waypoints.len().saturating_sub(1) / WAYPOINTS_PER_PAGE;
                if self.waypoints_page < max_page {
                    self.waypoints_page += 1;
                }
                Task::none()
            }

            Message::PrevWaypointsPage => {
                self.waypoints_page = self.waypoints_page.saturating_sub(1);
                Task::none()
            }

            Message::Tick(_) => {
                self.time = self.start_time.elapsed().as_secs_f32();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let agent_panel = container(
            column![
                row![
                    text("SYMBOL:"),
                    text(self.agent.symbol.as_str())
                        .width(Length::Fill)
                        .align_x(Horizontal::Right)
                ],
                row![
                    text("HQ:"),
                    text(self.agent.headquarters.as_str())
                        .width(Length::Fill)
                        .align_x(Horizontal::Right)
                ],
                row![
                    text("CREDITS:"),
                    text(format!("{}", self.agent.credits))
                        .width(Length::Fill)
                        .align_x(Horizontal::Right)
                ],
                row![
                    text("FACTION:"),
                    text(self.agent.starting_faction.as_str())
                        .width(Length::Fill)
                        .align_x(Horizontal::Right)
                ],
                row![
                    text("FLEET:"),
                    text(format!("{}", self.agent.ship_count))
                        .width(Length::Fill)
                        .align_x(Horizontal::Right)
                ],
            ]
            .spacing(6),
        )
        .style(theme::panel)
        .padding(16)
        .width(Length::Fixed(256.0));

        let ships_panel = container(
            column![
                text("[ FLEET ]").size(18),
                scrollable(ship_list(&self.ships)),
            ]
            .spacing(6),
        )
        .style(theme::panel)
        .padding(16)
        .width(Length::Fill);

        let contracts_panel = container(
            column![
                text("[ CONTRACTS ]").size(18),
                scrollable(contract_list(&self.contracts)),
            ]
            .spacing(6),
        )
        .style(theme::panel)
        .padding(16)
        .width(Length::Fill);

        let total_waypoint_pages =
            (self.waypoints.len() + WAYPOINTS_PER_PAGE - 1) / WAYPOINTS_PER_PAGE;
        let waypoints_start = self.waypoints_page * WAYPOINTS_PER_PAGE;
        let waypoints_slice = &self.waypoints
            [waypoints_start..(waypoints_start + WAYPOINTS_PER_PAGE).min(self.waypoints.len())];

        let waypoints_nav = row![
            button(text(" [ < ]"))
                .on_press_maybe((self.waypoints_page > 0).then_some(Message::PrevWaypointsPage))
                .style(theme::crt_button),
            text(format!(
                "{}/{}",
                self.waypoints_page + 1,
                total_waypoint_pages.max(1)
            )),
            button(text("[ > ]"))
                .on_press_maybe(
                    (self.waypoints_page + 1 < total_waypoint_pages)
                        .then_some(Message::NextWaypointsPage)
                )
                .style(theme::crt_button)
        ]
        .spacing(8);

        let waypoints_panel = container(
            column![
                text(format!(
                    "[ WAYPOINTS - {} ]",
                    headquarters_to_system(&self.agent.headquarters)
                ))
                .size(18),
                waypoint_list(waypoints_slice),
                waypoints_nav,
            ]
            .spacing(6),
        )
        .style(theme::panel)
        .padding(16)
        .width(Length::Fill);

        let content = column![
            agent_panel,
            row![ships_panel, contracts_panel, waypoints_panel].spacing(8),
        ]
        .spacing(12)
        .padding(24);

        let crt_overlay = || {
            shader::<Message, CrtOverlay>(CrtOverlay { time: self.time })
                .width(Length::Fill)
                .height(Length::Fill)
        };

        if let Some(err) = &self.error {
            stack![
                column![text(format!("Error: {err}")), content].spacing(8),
                crt_overlay()
            ]
            .into()
        } else {
            if let Some(idx) = self.selected_contract {
                let contract = &self.contracts[idx];
                stack![
                    content,
                    opaque(
                        mouse_area(
                            container(Space::new())
                                .style(theme::overlay)
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                        .on_press(Message::CloseOverlay)
                    ),
                    opaque(
                        container(contract_detail(contract))
                            .style(theme::overlay)
                            .center(Length::Fill)
                    ),
                    crt_overlay()
                ]
                .into()
            } else {
                stack![content, crt_overlay()].into()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::window::frames().map(Message::Tick)
    }
}

fn ship_list(ships: &[Ship]) -> Column<'_, Message> {
    ships.iter().fold(Column::new().spacing(8), |col, ship| {
        col.push(text(format!("{}", ship.symbol)).size(16)).push(
            text(format!(
                "{} @ {}",
                ship.registration.role, ship.nav.waypoint_symbol
            ))
            .size(12),
        )
    })
}

fn contract_list(contracts: &[Contract]) -> Column<'_, Message> {
    contracts
        .iter()
        .enumerate()
        .fold(Column::new().spacing(8), |col, (i, c)| {
            let status = if c.fulfilled {
                "fullfilled"
            } else if c.accepted {
                "active"
            } else {
                "avaiable"
            };
            col.push(
                button(text(format!(
                    "[{}] {} - {:?}",
                    status, c.faction_symbol, c.r#type
                )))
                .on_press(Message::SelectContract(i))
                .style(theme::crt_button),
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
        text(format!(
            "Status: {}",
            if contract.fulfilled {
                "Fulfilled"
            } else if contract.accepted {
                "Active"
            } else {
                "Available"
            }
        )),
        text(format!("Deadline: {}", contract.terms.deadline)),
        text(format!(
            "On Accept:  {} credits",
            contract.terms.payment.on_accepted
        )),
        text(format!(
            "On Fulfill: {} credits",
            contract.terms.payment.on_fulfilled
        )),
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
    col.push(
        button(text("[ CLOSE ]"))
            .on_press(Message::CloseOverlay)
            .style(theme::crt_button_amber),
    )
}
