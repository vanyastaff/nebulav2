use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterKind {
    Text,
    Textarea,
    Select,
    MultiSelect,
    Checkbox,
    Number,
    DateTime,
    Date,
    Time,
    Secret,
    Hidden,
    Notice,
    Button,
    Mode,
    Group,
    File,
    Color,
    Expirable,
    Radio,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterCapability {
    HasValue,
    Editable,
    Validatable,
    Displayable,
}

impl ParameterKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ParameterKind::Text => "text",
            ParameterKind::Textarea => "textarea",
            ParameterKind::Select => "select",
            ParameterKind::MultiSelect => "multiselect",
            ParameterKind::Checkbox => "checkbox",
            ParameterKind::Number => "number",
            ParameterKind::DateTime => "datetime",
            ParameterKind::Date => "date",
            ParameterKind::Time => "time",
            ParameterKind::Secret => "secret",
            ParameterKind::Hidden => "hidden",
            ParameterKind::Notice => "notice",
            ParameterKind::Button => "button",
            ParameterKind::Mode => "mode",
            ParameterKind::Group => "group",
            ParameterKind::File => "file",
            ParameterKind::Color => "color",
            ParameterKind::Expirable => "expirable",
            ParameterKind::Radio => "radio",
        }
    }

    pub fn capabilities(&self) -> &'static [ParameterCapability] {
        use ParameterCapability::*;

        match self {
            ParameterKind::Notice => &[Displayable],
            ParameterKind::Button => &[Displayable],
            ParameterKind::Group => &[Displayable],

            ParameterKind::Hidden => &[HasValue],

            ParameterKind::Checkbox => &[HasValue, Editable, Displayable],

            ParameterKind::Text |
            ParameterKind::Textarea |
            ParameterKind::Select |
            ParameterKind::MultiSelect |
            ParameterKind::Number |
            ParameterKind::DateTime |
            ParameterKind::Date |
            ParameterKind::Time |
            ParameterKind::Secret |
            ParameterKind::File |
            ParameterKind::Color |
            ParameterKind::Radio => &[HasValue, Editable, Validatable, Displayable],

            ParameterKind::Mode => &[HasValue, Editable, Validatable, Displayable],
            ParameterKind::Expirable => &[HasValue, Editable, Validatable, Displayable],
        }
    }

    pub fn has_capability(&self, capability: ParameterCapability) -> bool {
        self.capabilities().contains(&capability)
    }

    pub fn has_value(&self) -> bool {
        self.has_capability(ParameterCapability::HasValue)
    }

    pub fn is_editable(&self) -> bool {
        self.has_capability(ParameterCapability::Editable)
    }

    pub fn is_validatable(&self) -> bool {
        self.has_capability(ParameterCapability::Validatable)
    }

    pub fn is_displayable(&self) -> bool {
        self.has_capability(ParameterCapability::Displayable)
    }
}