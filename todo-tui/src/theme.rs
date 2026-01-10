//! Nord theme colors for the TUI
//!
//! Based on the Nord color palette: https://www.nordtheme.com/

use ratatui::style::Color;

/// Nord Polar Night - Dark backgrounds
pub mod polar_night {
    use super::*;

    /// Darkest background
    pub const NORD0: Color = Color::Rgb(46, 52, 64);
    /// Lighter background
    pub const NORD1: Color = Color::Rgb(59, 66, 82);
    /// Selection background
    pub const NORD2: Color = Color::Rgb(67, 76, 94);
    /// Bright background / Comments
    pub const NORD3: Color = Color::Rgb(76, 86, 106);
}

/// Nord Snow Storm - Light text colors
pub mod snow_storm {
    use super::*;

    /// Main text
    pub const NORD4: Color = Color::Rgb(216, 222, 233);
    /// Brightest text
    pub const NORD6: Color = Color::Rgb(236, 239, 244);
}

/// Nord Frost - Accent colors (blue/cyan)
pub mod frost {
    use super::*;

    /// Frozen polar water
    pub const NORD7: Color = Color::Rgb(143, 188, 187);
    /// Pure ice
    pub const NORD8: Color = Color::Rgb(136, 192, 208);
}

/// Nord Aurora - Semantic colors
pub mod aurora {
    use super::*;

    /// Red - Errors, urgent
    pub const NORD11: Color = Color::Rgb(191, 97, 106);
    /// Orange - Warnings
    pub const NORD12: Color = Color::Rgb(208, 135, 112);
    /// Yellow - Caution
    pub const NORD13: Color = Color::Rgb(235, 203, 139);
    /// Green - Success, completed
    pub const NORD14: Color = Color::Rgb(163, 190, 140);
    /// Purple - Special
    pub const NORD15: Color = Color::Rgb(180, 142, 173);
}

// Semantic aliases for easier use
pub use polar_night::NORD0 as BG_DARK;
pub use polar_night::NORD1 as BG_LIGHT;
pub use polar_night::NORD2 as SELECTION;
pub use polar_night::NORD3 as BORDER;

pub use snow_storm::NORD4 as TEXT;
pub use snow_storm::NORD6 as TEXT_HIGHLIGHT;

pub use frost::NORD8 as ACCENT;

pub use aurora::NORD11 as PRIORITY_A;
pub use aurora::NORD12 as PRIORITY_B;
pub use aurora::NORD13 as PRIORITY_C;
pub use aurora::NORD14 as COMPLETED;
pub use aurora::NORD15 as PROJECT;
