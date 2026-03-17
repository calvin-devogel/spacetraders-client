use iced::theme::Palette;
use iced::widget::{button, container};
use iced::{Border, Color, Theme};

pub mod palette {
    use iced::Color;
    // black, surface, white, dim white, border, selected item, alert
    pub const VOID: Color = Color {
        r: 0.00,
        g: 0.00,
        b: 0.00,
        a: 1.0,
    };
    pub const SURFACE: Color = Color {
        r: 0.05,
        g: 0.02,
        b: 0.02,
        a: 1.0,
    };
    pub const PHOSPHOR: Color = Color {
        r: 0.93,
        g: 0.93,
        b: 0.93,
        a: 1.0,
    };
    pub const PHOSPHOR_DIM: Color = Color {
        r: 0.45,
        g: 0.45,
        b: 0.45,
        a: 1.0,
    };
    pub const BORDER: Color = Color {
        r: 0.40,
        g: 0.06,
        b: 0.06,
        a: 1.0,
    };
    pub const AMBER: Color = Color {
        r: 1.00,
        g: 0.75,
        b: 0.00,
        a: 1.0,
    };
    pub const ALERT: Color = Color {
        r: 1.00,
        g: 0.00,
        b: 0.00,
        a: 1.0,
    };
}

pub fn crt_theme() -> Theme {
    Theme::custom(
        "CRT".to_string(),
        Palette {
            background: palette::VOID,
            text: palette::PHOSPHOR,
            primary: palette::PHOSPHOR,
            success: palette::PHOSPHOR_DIM,
            warning: palette::AMBER,
            danger: palette::ALERT,
        },
    )
}

// containers

pub fn panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(palette::SURFACE.into()),
        border: Border {
            color: palette::PHOSPHOR,
            width: 2.0,
            radius: 0.0.into(),
        },
        text_color: Some(palette::PHOSPHOR),
        ..Default::default()
    }
}

pub fn panel_inset(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(palette::VOID.into()),
        border: Border {
            color: palette::PHOSPHOR_DIM,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(palette::PHOSPHOR_DIM),
        ..Default::default()
    }
}

pub fn panel_active(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(palette::SURFACE.into()),
        border: Border {
            color: palette::AMBER,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(palette::AMBER),
        ..Default::default()
    }
}

pub fn overlay(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.80,
            }
            .into(),
        ),
        ..Default::default()
    }
}

// buttons
pub fn crt_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: palette::PHOSPHOR_DIM,
            border: Border::default(),
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(palette::BORDER.into()),
            text_color: palette::PHOSPHOR,
            border: Border {
                color: palette::BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(palette::PHOSPHOR_DIM.into()),
            text_color: palette::VOID,
            border: Border::default(),
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color {
                a: 0.25,
                ..palette::PHOSPHOR_DIM
            },
            border: Border::default(),
            ..Default::default()
        },
    }
}

// primary action button (confirm/accept/etc.)
pub fn crt_button_amber(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: Color {
                a: 0.75,
                ..palette::AMBER
            },
            border: Border {
                color: Color {
                    a: 0.40,
                    ..palette::AMBER
                },
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(
                Color {
                    a: 0.15,
                    ..palette::AMBER
                }
                .into(),
            ),
            text_color: palette::AMBER,
            border: Border {
                color: palette::AMBER,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(palette::AMBER.into()),
            text_color: palette::VOID,
            border: Border::default(),
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color {
                a: 0.25,
                ..palette::AMBER
            },
            border: Border::default(),
            ..Default::default()
        },
    }
}
