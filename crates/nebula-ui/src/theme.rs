use egui::Color32;

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: Color32,
    pub secondary: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub success: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub border: Color32,
}

#[derive(Debug, Clone)]
pub struct ThemeSpacing {
    pub item_spacing: f32,
    pub indent: f32,
    pub input_height: f32,
    pub label_spacing: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    pub fn light() -> Self {
        Self {
            colors: ThemeColors {
                primary: Color32::from_rgb(0, 123, 255),
                secondary: Color32::from_rgb(108, 117, 125),
                error: Color32::from_rgb(220, 53, 69),
                warning: Color32::from_rgb(255, 193, 7),
                success: Color32::from_rgb(40, 167, 69),
                background: Color32::from_rgb(248, 249, 250),
                surface: Color32::WHITE,
                text: Color32::from_rgb(33, 37, 41),
                text_secondary: Color32::from_rgb(108, 117, 125),
                border: Color32::from_rgb(222, 226, 230),
            },
            spacing: ThemeSpacing {
                item_spacing: 8.0,
                indent: 16.0,
                input_height: 32.0,
                label_spacing: 4.0,
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            colors: ThemeColors {
                primary: Color32::from_rgb(13, 110, 253),
                secondary: Color32::from_rgb(108, 117, 125),
                error: Color32::from_rgb(248, 81, 73),
                warning: Color32::from_rgb(255, 205, 57),
                success: Color32::from_rgb(25, 135, 84),
                background: Color32::from_rgb(33, 37, 41),
                surface: Color32::from_rgb(52, 58, 64),
                text: Color32::from_rgb(248, 249, 250),
                text_secondary: Color32::from_rgb(173, 181, 189),
                border: Color32::from_rgb(73, 80, 87),
            },
            spacing: ThemeSpacing {
                item_spacing: 8.0,
                indent: 16.0,
                input_height: 32.0,
                label_spacing: 4.0,
            },
        }
    }
}
