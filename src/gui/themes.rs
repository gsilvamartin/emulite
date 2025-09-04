//! GUI themes for Emulite
//! 
//! This module provides different visual themes for the emulator interface.

use crate::gui::*;
use eframe::egui;

/// Available themes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeType {
    Default,
    Dark,
    Light,
    Retro,
    HighContrast,
}

impl ThemeType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Default,
            Self::Dark,
            Self::Light,
            Self::Retro,
            Self::HighContrast,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::Retro => "Retro",
            Self::HighContrast => "High Contrast",
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    current_theme: ThemeType,
    themes: std::collections::HashMap<ThemeType, GuiTheme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = std::collections::HashMap::new();
        
        // Default theme
        themes.insert(ThemeType::Default, Self::create_default_theme());
        themes.insert(ThemeType::Dark, Self::create_dark_theme());
        themes.insert(ThemeType::Light, Self::create_light_theme());
        themes.insert(ThemeType::Retro, Self::create_retro_theme());
        themes.insert(ThemeType::HighContrast, Self::create_high_contrast_theme());

        Self {
            current_theme: ThemeType::Default,
            themes,
        }
    }

    pub fn get_current_theme(&self) -> &GuiTheme {
        self.themes.get(&self.current_theme).unwrap()
    }

    pub fn set_theme(&mut self, theme_type: ThemeType) {
        self.current_theme = theme_type;
    }

    pub fn get_current_theme_type(&self) -> &ThemeType {
        &self.current_theme
    }

    /// Create default theme
    fn create_default_theme() -> GuiTheme {
        GuiTheme {
            name: "Default".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(30, 30, 30),
                foreground: egui::Color32::from_rgb(200, 200, 200),
                accent: egui::Color32::from_rgb(100, 150, 255),
                error: egui::Color32::from_rgb(255, 100, 100),
                warning: egui::Color32::from_rgb(255, 200, 100),
                success: egui::Color32::from_rgb(100, 255, 100),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }

    /// Create dark theme
    fn create_dark_theme() -> GuiTheme {
        GuiTheme {
            name: "Dark".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(20, 20, 20),
                foreground: egui::Color32::from_rgb(220, 220, 220),
                accent: egui::Color32::from_rgb(80, 130, 235),
                error: egui::Color32::from_rgb(255, 80, 80),
                warning: egui::Color32::from_rgb(255, 180, 80),
                success: egui::Color32::from_rgb(80, 255, 80),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }

    /// Create light theme
    fn create_light_theme() -> GuiTheme {
        GuiTheme {
            name: "Light".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(250, 250, 250),
                foreground: egui::Color32::from_rgb(30, 30, 30),
                accent: egui::Color32::from_rgb(0, 100, 200),
                error: egui::Color32::from_rgb(200, 0, 0),
                warning: egui::Color32::from_rgb(200, 100, 0),
                success: egui::Color32::from_rgb(0, 150, 0),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }

    /// Create retro theme
    fn create_retro_theme() -> GuiTheme {
        GuiTheme {
            name: "Retro".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(0, 0, 0),
                foreground: egui::Color32::from_rgb(0, 255, 0),
                accent: egui::Color32::from_rgb(255, 255, 0),
                error: egui::Color32::from_rgb(255, 0, 0),
                warning: egui::Color32::from_rgb(255, 165, 0),
                success: egui::Color32::from_rgb(0, 255, 0),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::monospace(18.0),
                body: egui::FontId::monospace(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 2.0,
                medium: 4.0,
                large: 8.0,
            },
        }
    }

    /// Create high contrast theme
    fn create_high_contrast_theme() -> GuiTheme {
        GuiTheme {
            name: "High Contrast".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(0, 0, 0),
                foreground: egui::Color32::from_rgb(255, 255, 255),
                accent: egui::Color32::from_rgb(255, 255, 0),
                error: egui::Color32::from_rgb(255, 0, 0),
                warning: egui::Color32::from_rgb(255, 255, 0),
                success: egui::Color32::from_rgb(0, 255, 0),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(20.0),
                body: egui::FontId::proportional(16.0),
                monospace: egui::FontId::monospace(14.0),
            },
            spacing: ThemeSpacing {
                small: 6.0,
                medium: 12.0,
                large: 24.0,
            },
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme selector widget
pub struct ThemeSelector {
    theme_manager: ThemeManager,
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            theme_manager: ThemeManager::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<ThemeType> {
        let mut changed = false;
        let current_theme = self.theme_manager.get_current_theme_type().clone();

        ui.horizontal(|ui| {
            ui.label("Theme:");
            
            egui::ComboBox::from_id_source("theme_selector")
                .selected_text(current_theme.name())
                .show_ui(ui, |ui| {
                    for theme_type in ThemeType::all() {
                        if ui.selectable_value(
                            &mut self.theme_manager.current_theme,
                            theme_type.clone(),
                            theme_type.name()
                        ).clicked() {
                            changed = true;
                        }
                    }
                });
        });

        if changed {
            Some(self.theme_manager.get_current_theme_type().clone())
        } else {
            None
        }
    }

    pub fn get_current_theme(&self) -> &GuiTheme {
        self.theme_manager.get_current_theme()
    }

    pub fn set_theme(&mut self, theme_type: ThemeType) {
        self.theme_manager.set_theme(theme_type);
    }
}

/// Apply a specific theme to the egui context
pub fn apply_theme_to_context(ctx: &egui::Context, theme: &GuiTheme) {
    let mut style = (*ctx.style()).clone();
    
    // Apply colors
    style.visuals.window_fill = theme.colors.background;
    style.visuals.panel_fill = theme.colors.background;
    style.visuals.window_stroke = egui::Stroke::new(1.0, theme.colors.foreground);
    style.visuals.override_text_color = Some(theme.colors.foreground);
    // style.visuals.accent_color = theme.colors.accent; // Not available in this egui version
    style.visuals.error_fg_color = theme.colors.error;
    style.visuals.warn_fg_color = theme.colors.warning;
    
    // Apply button colors
    style.visuals.button_frame = true;
    // style.visuals.inactive_fg_color = theme.colors.foreground; // Not available in this egui version
    // style.visuals.hovered_fg_color = theme.colors.accent; // Not available in this egui version
    // style.visuals.active_fg_color = theme.colors.accent; // Not available in this egui version
    
    // Apply selection colors
    style.visuals.selection.bg_fill = theme.colors.accent;
    style.visuals.selection.stroke = egui::Stroke::new(1.0, theme.colors.foreground);
    
    // Apply hyperlink colors
    style.visuals.hyperlink_color = theme.colors.accent;
    
    // Apply spacing
    style.spacing.item_spacing = egui::vec2(theme.spacing.medium, theme.spacing.medium);
    style.spacing.window_margin = egui::Margin::same(theme.spacing.medium);
    style.spacing.button_padding = egui::vec2(theme.spacing.medium, theme.spacing.small);
    style.spacing.menu_margin = egui::Margin::same(theme.spacing.small);
    
    // Apply window styling
    style.visuals.window_rounding = egui::Rounding::same(4.0);
    style.visuals.panel_fill = theme.colors.background;
    style.visuals.window_fill = theme.colors.background;
    
    // Apply button styling
    style.visuals.button_frame = true;
    // style.visuals.weak_text_color = theme.colors.foreground.gamma_multiply(0.7); // Not available in this egui version
    
    // Apply scrollbar styling
    // style.visuals.scroll_bg_fill = theme.colors.background; // Not available in this egui version
    // style.visuals.scroll_stroke = egui::Stroke::new(1.0, theme.colors.foreground.gamma_multiply(0.5)); // Not available in this egui version
    
    // Apply resize handle styling
    style.visuals.resize_corner_size = 8.0;
    
    // Apply text selection styling
    // style.visuals.text_selection.bg_fill = theme.colors.accent; // Not available in this egui version
    // style.visuals.text_selection.stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    // Apply cursor styling
    // style.visuals.cursor_stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    // Apply handle styling
    // style.visuals.handle_stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    // Apply collapsing header styling
    style.visuals.collapsing_header_frame = true;
    
    // Apply checkbox styling
    // style.visuals.checkbox_stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    // Apply radio button styling
    // style.visuals.radio_mark_stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    // Apply slider styling
    // style.visuals.slider_trailing_fill = theme.colors.accent; // Type mismatch in this egui version
    
    // Apply progress bar styling
    // style.visuals.progress_bar_fill = theme.colors.accent; // Not available in this egui version
    
    // Apply tooltip styling
    // style.visuals.tooltip_bg_fill = theme.colors.background; // Not available in this egui version
    // style.visuals.tooltip_stroke = egui::Stroke::new(1.0, theme.colors.foreground); // Not available in this egui version
    
    ctx.set_style(style);
}

/// Create a custom theme from user preferences
pub fn create_custom_theme(
    name: String,
    background: egui::Color32,
    foreground: egui::Color32,
    accent: egui::Color32,
) -> GuiTheme {
    GuiTheme {
        name,
        colors: ThemeColors {
            background,
            foreground,
            accent,
            error: egui::Color32::from_rgb(255, 100, 100),
            warning: egui::Color32::from_rgb(255, 200, 100),
            success: egui::Color32::from_rgb(100, 255, 100),
        },
        fonts: ThemeFonts {
            heading: egui::FontId::proportional(18.0),
            body: egui::FontId::proportional(14.0),
            monospace: egui::FontId::monospace(12.0),
        },
        spacing: ThemeSpacing {
            small: 4.0,
            medium: 8.0,
            large: 16.0,
        },
    }
}

/// Theme presets for common use cases
pub mod presets {
    use super::*;

    /// Gaming theme with dark colors and accent
    pub fn gaming_theme() -> GuiTheme {
        GuiTheme {
            name: "Gaming".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(15, 15, 25),
                foreground: egui::Color32::from_rgb(200, 200, 220),
                accent: egui::Color32::from_rgb(100, 200, 255),
                error: egui::Color32::from_rgb(255, 100, 100),
                warning: egui::Color32::from_rgb(255, 200, 100),
                success: egui::Color32::from_rgb(100, 255, 100),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }

    /// Professional theme for development
    pub fn professional_theme() -> GuiTheme {
        GuiTheme {
            name: "Professional".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(40, 40, 40),
                foreground: egui::Color32::from_rgb(220, 220, 220),
                accent: egui::Color32::from_rgb(100, 150, 255),
                error: egui::Color32::from_rgb(255, 100, 100),
                warning: egui::Color32::from_rgb(255, 200, 100),
                success: egui::Color32::from_rgb(100, 255, 100),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }

    /// Minimal theme with clean design
    pub fn minimal_theme() -> GuiTheme {
        GuiTheme {
            name: "Minimal".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(250, 250, 250),
                foreground: egui::Color32::from_rgb(50, 50, 50),
                accent: egui::Color32::from_rgb(0, 150, 255),
                error: egui::Color32::from_rgb(255, 100, 100),
                warning: egui::Color32::from_rgb(255, 200, 100),
                success: egui::Color32::from_rgb(100, 255, 100),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 6.0,
                medium: 12.0,
                large: 24.0,
            },
        }
    }
}
