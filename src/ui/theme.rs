use bevy::prelude::*;

// Professional Dark Theme Palette (Inspired by modern IDEs & macOS)
pub const BG_BASE: Color = Color::rgb(0.07, 0.07, 0.09);      // Deep dark background
pub const BG_SIDEBAR: Color = Color::rgb(0.10, 0.10, 0.13);   // Slightly lighter for sidebar
pub const SURFACE: Color = Color::rgb(0.15, 0.15, 0.19);      // Task cards
pub const SURFACE_HOVER: Color = Color::rgb(0.20, 0.20, 0.25); // Hover state for cards

pub const ACCENT: Color = Color::rgb(0.45, 0.35, 0.90);       // Vibrant Purple/Blue accent
pub const ACCENT_HOVER: Color = Color::rgb(0.55, 0.45, 0.95); 

pub const TEXT_PRIMARY: Color = Color::rgb(0.90, 0.90, 0.95); // Bright text
pub const TEXT_MUTED: Color = Color::rgb(0.50, 0.50, 0.55);   // Muted text for completed tasks

pub const BORDER: Color = Color::rgb(0.20, 0.20, 0.25);       // Subtle borders
pub const SUCCESS: Color = Color::rgb(0.20, 0.80, 0.40);      // Green for completed checkboxes
