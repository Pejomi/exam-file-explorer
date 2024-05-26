use eframe::epaint;
use egui::{Color32, style};
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub base: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub tertiary: Color32,
    pub stroke: Color32,
    pub text: Color32,
}

const LIGHT_THEME: Theme = Theme {
    base: Color32::WHITE,
    primary: Color32::LIGHT_GRAY,
    secondary: Color32::from_rgb(222, 222, 222),
    tertiary: Color32::from_rgb(235, 235, 235),
    stroke: Color32::GRAY,
    text: Color32::from_rgb(110, 110, 110)
};

const DARK_THEME: Theme = Theme {
    base: Color32::from_rgb(15, 16, 18),
    primary: Color32::from_rgb(36, 37, 41),
    secondary: Color32::from_rgb(32, 33, 36),
    tertiary: Color32::from_rgb(52, 58, 64),
    stroke: Color32::from_rgb(52, 58, 64),
    text: Color32::from_rgb(173, 181, 189)
};

const OCEAN_THEME: Theme = Theme {
    base: Color32::from_rgb(202, 240, 248),
    primary: Color32::from_rgb(173, 232, 244),
    secondary: Color32::from_rgb(144, 224, 239),
    tertiary: Color32::from_rgb(227, 242, 253),
    stroke: Color32::from_rgb(0, 180, 216),
    text: Color32::from_rgb(0, 119, 182)
};

const PRO_HACKER_THEME: Theme = Theme {
    base: Color32::BLACK,
    primary: Color32::BLACK,
    secondary: Color32::BLACK,
    tertiary: Color32::from_rgb(0, 9, 173),
    stroke: Color32::from_rgb(0, 196, 7),
    text: Color32::from_rgb(0, 196, 7)
};


#[derive(PartialEq, Default, Debug, Clone, Sequence, Copy)]
pub enum Mode {
    #[default]
    Light,
    Dark,
    Ocean,
    ProHacker
}

impl Mode {
    pub fn get_theme(&self) -> (Theme, String) {
        match self {
            Mode::Light => (LIGHT_THEME, "💡 Light".parse().unwrap()),
            Mode::Dark => (DARK_THEME, "🌙 Dark".parse().unwrap()),
            Mode::Ocean => (OCEAN_THEME, "🌊 Ocean".parse().unwrap()),
            Mode::ProHacker => (PRO_HACKER_THEME, "🔥 Pro Hacker".parse().unwrap())
        }
    }
}


pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    ctx.set_visuals(theme.visuals(ctx.style().visuals.clone()));
}

fn make_widget(default: style::WidgetVisuals, theme: Theme) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill: theme.primary,
        weak_bg_fill: theme.primary,
        bg_stroke: egui::Stroke {
            color: theme.stroke,
            ..default.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.stroke,
            ..default.fg_stroke
        },
        ..default
    }
}

impl Theme {
    fn visuals(self, default: egui::Visuals) -> egui::Visuals {
        egui::Visuals {
            override_text_color: Some(self.text),
            widgets: style::Widgets {
                noninteractive: make_widget(default.widgets.noninteractive, self),
                inactive: make_widget(default.widgets.inactive, self),
                hovered: make_widget(default.widgets.hovered, self),
                active: make_widget(default.widgets.active, self),
                open: make_widget(default.widgets.open, self),
            },
            selection: style::Selection {
                bg_fill: self.tertiary,
                ..default.selection
            },
            extreme_bg_color: self.secondary,
            window_fill: self.primary,
            window_stroke: egui::Stroke {
                color: self.stroke,
                ..default.window_stroke
            },
            panel_fill: self.base,
            popup_shadow: epaint::Shadow {
                offset: Default::default(),
                blur: 0.0,
                spread: 0.0,
                color: self.primary,
            },
            ..default
        }
    }
}