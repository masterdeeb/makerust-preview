use bevy::prelude::*;

// Modern Dark Theme (Catppuccin Mocha inspired)
pub const BG_APP: Color = Color::rgb(0.118, 0.118, 0.180); // #1e1e2e
pub const BG_SIDEBAR: Color = Color::rgb(0.094, 0.094, 0.145); // #181825
pub const BG_CARD: Color = Color::rgb(0.192, 0.196, 0.267); // #313244
pub const BG_CARD_HOVER: Color = Color::rgb(0.271, 0.278, 0.353); // #45475a

pub const ACCENT_PRIMARY: Color = Color::rgb(0.537, 0.706, 0.980); // #89b4fa
pub const ACCENT_PRIMARY_HOVER: Color = Color::rgb(0.706, 0.824, 1.0); // Lighter blue

pub const TEXT_MAIN: Color = Color::rgb(0.804, 0.839, 0.957); // #cdd6f4
pub const TEXT_MUTED: Color = Color::rgb(0.651, 0.678, 0.784); // #a6adc8

pub const SUCCESS: Color = Color::rgb(0.651, 0.890, 0.631); // #a6e3a1
pub const DANGER: Color = Color::rgb(0.953, 0.545, 0.659); // #f38ba8
pub const DANGER_HOVER: Color = Color::rgb(1.0, 0.65, 0.75);

// UI Styling Constants
pub const RADIUS_CARD: Val = Val::Px(12.0);
pub const RADIUS_BUTTON: Val = Val::Px(8.0);
pub const PADDING_CARD: Val = Val::Px(16.0);
