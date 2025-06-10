use eframe::egui;
use nebula_ui::{RenderContext, TextWidget, Theme};

struct ParameterApp {
    ctx: RenderContext,
    text_widgets: Vec<TextWidget>,
    is_dark_theme: bool,
}

impl Default for ParameterApp {
    fn default() -> Self {
        let mut ctx = RenderContext::new(Theme::light());

        // Добавляем тестовые ошибки валидации
        ctx.add_validation_error(
            "email".to_string(),
            "Please enter a valid email address".to_string(),
        );

        Self {
            ctx,
            text_widgets: vec![
                TextWidget::new("name", "Full Name")
                    .with_value("John Doe")
                    .required(true),

                TextWidget::new("email", "Email Address")
                    .with_placeholder("example@domain.com")
                    .with_hint("We'll never share your email")
                    .required(true),

                TextWidget::new("phone", "Phone Number")
                    .with_placeholder("+1 (555) 123-4567")
                    .with_hint("Optional field"),

                TextWidget::new("bio", "Biography")
                    .with_value("Software developer")
                    .with_hint("Tell us about yourself"),

                TextWidget::new("readonly", "Read Only Field")
                    .with_value("This cannot be edited")
                    .readonly(true),
            ],
            is_dark_theme: false,
        }
    }
}

impl eframe::App for ParameterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Parameter Widgets Demo");

            ui.separator();

            // Theme toggle
            ui.horizontal(|ui| {
                if ui.button("Toggle Theme").clicked() {
                    self.is_dark_theme = !self.is_dark_theme;
                    self.ctx.theme = if self.is_dark_theme {
                        Theme::dark()
                    } else {
                        Theme::light()
                    };
                }

                ui.label(format!(
                    "Current theme: {}",
                    if self.is_dark_theme { "Dark" } else { "Light" }
                ));
            });

            ui.separator();

            // Parameters
            egui::ScrollArea::vertical().show(ui, |ui| {
                for widget in &mut self.text_widgets {
                    if widget.render(ui, &self.ctx) {
                        println!("Widget '{}' changed to: '{}'", widget.key, widget.value);

                        // Простая валидация для демо
                        if widget.key == "email" {
                            self.ctx.clear_validation_errors(&widget.key);
                            if !widget.value.is_empty() && !widget.value.contains('@') {
                                self.ctx.add_validation_error(
                                    widget.key.clone(),
                                    "Please enter a valid email address".to_string(),
                                );
                            }
                        }
                    }
                }
            });

            ui.separator();

            // Current values display
            ui.collapsing("Current Values", |ui| {
                for widget in &self.text_widgets {
                    ui.horizontal(|ui| {
                        ui.strong(&widget.key);
                        ui.label(":");
                        ui.label(&widget.value);
                    });
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Parameter Widgets Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Parameter Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ParameterApp::default()))),
    )
}