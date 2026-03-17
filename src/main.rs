use spacetraders::components::{agent::Agent, configuration::get_configuration, home::Home, theme};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("failed to get configuration");
    let agent = Agent::new(&config).await.expect("failed to get agent");

    iced::application(
        move || Home::new(config.clone(), agent.clone()),
        Home::update,
        Home::view,
    )
    .subscription(Home::subscription)
    .theme(|_state: &Home| theme::crt_theme()) // hmmmmmm
    .default_font(iced::Font::MONOSPACE)
    .window(iced::window::Settings {
        size: iced::Size::new(1024.0, 768.0),
        ..Default::default()
    })
    .run()
    .expect("failed to run app")
}
