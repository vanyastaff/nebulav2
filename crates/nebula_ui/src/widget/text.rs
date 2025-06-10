use crate::context::RenderContext;
use egui::{Response, Ui, Widget};

#[derive(Debug, Clone)]
pub struct TextWidget {
    pub key: String,
    pub label: String,
    pub value: String,
    pub placeholder: Option<String>,
    pub hint: Option<String>,
    pub required: bool,
    pub readonly: bool,
}

impl TextWidget {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            value: String::new(),
            placeholder: None,
            hint: None,
            required: false,
            readonly: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn render(&mut self, ui: &mut Ui, ctx: &RenderContext) -> bool {
        let mut changed = false;
        let has_errors = ctx.has_errors(&self.key);

        ui.vertical(|ui| {
            // Label с индикатором обязательности
            ui.horizontal(|ui| {
                let label_text = if self.required {
                    format!("{} *", self.label)
                } else {
                    self.label.clone()
                };

                let label_color = if has_errors {
                    ctx.theme.colors.error
                } else {
                    ctx.theme.colors.text
                };

                ui.colored_label(label_color, label_text);
            });

            ui.add_space(ctx.theme.spacing.label_spacing);

            // Input field с учетом ошибок
            let text_edit = egui::TextEdit::singleline(&mut self.value)
                .desired_width(f32::INFINITY)
                .hint_text(self.placeholder.as_deref().unwrap_or(""))
                .vertical_align(egui::Align::Center) // Центрируем по вертикали
                .margin(egui::Margin::symmetric(8, 4)); // Добавляем внутренние отступы

            let text_edit = if self.readonly {
                text_edit.interactive(false)
            } else {
                text_edit
            };

            // Применяем стиль ошибки к самому TextEdit
            let text_edit = if has_errors {
                text_edit.text_color(ctx.theme.colors.error)
            } else {
                text_edit
            };

            let mut response = ui.add_sized(
                [ui.available_width(), ctx.theme.spacing.input_height],
                text_edit,
            );

            // Добавляем hint как hover tooltip
            if let Some(hint) = &self.hint {
                response = response.on_hover_text(hint);
            }

            if response.changed() {
                changed = true;
            }

            // Validation errors
            if let Some(errors) = ctx.get_errors(&self.key) {
                ui.add_space(2.0);
                for error in errors {
                    ui.colored_label(ctx.theme.colors.error, format!("⚠ {}", error));
                }
            }

            ui.add_space(ctx.theme.spacing.item_spacing);
        });

        changed
    }
}

// Implement Widget trait for convenient usage
impl Widget for &mut TextWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let ctx = RenderContext::default();
        self.render(ui, &ctx);
        ui.allocate_response(ui.available_size(), egui::Sense::hover())
    }
}