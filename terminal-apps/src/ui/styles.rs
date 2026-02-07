/// Styling system for terminal UI
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fg_color: u8,
    pub bg_color: u8,
    pub bold: bool,
    pub dimmed: bool,
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: Style,
    pub secondary: Style,
    pub accent: Style,
    pub success: Style,
    pub error: Style,
    pub warning: Style,
    pub info: Style,
    pub background: Style,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg_color: 15,  // White
            bg_color: 0,   // Black
            bold: false,
            dimmed: false,
        }
    }
}

impl ColorScheme {
    /// Warp theme color scheme
    pub fn warp() -> Self {
        Self {
            primary: Style {
                fg_color: 51,   // Cyan
                bg_color: 16,   // Dark background
                bold: true,
                dimmed: false,
            },
            secondary: Style {
                fg_color: 243,  // Gray
                bg_color: 16,
                bold: false,
                dimmed: false,
            },
            accent: Style {
                fg_color: 51,   // Cyan accent
                bg_color: 16,
                bold: true,
                dimmed: false,
            },
            success: Style {
                fg_color: 46,   // Green
                bg_color: 16,
                bold: true,
                dimmed: false,
            },
            error: Style {
                fg_color: 196,  // Red
                bg_color: 16,
                bold: true,
                dimmed: false,
            },
            warning: Style {
                fg_color: 226,  // Yellow
                bg_color: 16,
                bold: true,
                dimmed: false,
            },
            info: Style {
                fg_color: 33,   // Blue
                bg_color: 16,
                bold: false,
                dimmed: false,
            },
            background: Style {
                fg_color: 231,  // White text
                bg_color: 16,   // Dark background
                bold: false,
                dimmed: false,
            },
        }
    }

    /// Dark theme variant
    pub fn dark() -> Self {
        Self::warp()
    }

    /// Light theme variant
    pub fn light() -> Self {
        Self {
            primary: Style {
                fg_color: 33,   // Blue on light background
                bg_color: 231,  // White background
                bold: true,
                dimmed: false,
            },
            secondary: Style {
                fg_color: 240,  // Dark gray
                bg_color: 231,
                bold: false,
                dimmed: false,
            },
            accent: Style {
                fg_color: 33,   // Blue
                bg_color: 231,
                bold: true,
                dimmed: false,
            },
            success: Style {
                fg_color: 22,   // Dark green
                bg_color: 231,
                bold: true,
                dimmed: false,
            },
            error: Style {
                fg_color: 160,  // Dark red
                bg_color: 231,
                bold: true,
                dimmed: false,
            },
            warning: Style {
                fg_color: 172,  // Dark orange
                bg_color: 231,
                bold: true,
                dimmed: false,
            },
            info: Style {
                fg_color: 33,   // Blue
                bg_color: 231,
                bold: false,
                dimmed: false,
            },
            background: Style {
                fg_color: 0,    // Black text
                bg_color: 231,  // White background
                bold: false,
                dimmed: false,
            },
        }
    }

    pub fn apply_style(&self, text: &str, style: &Style) -> String {
        let mut result = String::new();
        result.push_str(&format!("\x1b[38;5;{}m", style.fg_color));
        result.push_str(&format!("\x1b[48;5;{}m", style.bg_color));
        if style.bold {
            result.push_str("\x1b[1m");
        }
        if style.dimmed {
            result.push_str("\x1b[2m");
        }
        result.push_str(text);
        result.push_str("\x1b[0m");
        result
    }
}
