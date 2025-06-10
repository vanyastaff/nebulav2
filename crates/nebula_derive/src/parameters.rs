// crates/nebula_macros/src/parameters.rs

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result, Field, Attribute, Meta, Lit, Path};
use std::collections::HashMap;
use crate::utils::field_name_to_key;

// Убираем Default из FieldConfig
#[derive(Debug)]
struct FieldConfig {
    field_name: String,
    field_type: syn::Type,
    common_attrs: CommonAttributes,
    param_type: ParameterTypeConfig,
    display_attrs: Option<DisplayAttributes>,
    validation_attrs: Option<ValidationAttributes>,
}

impl FieldConfig {
    fn new(field_name: &str, field_type: syn::Type) -> Self {
        Self {
            field_name: field_name.to_string(),
            field_type,
            common_attrs: CommonAttributes::default(),
            param_type: ParameterTypeConfig::default(),
            display_attrs: None,
            validation_attrs: None,
        }
    }
}

#[derive(Debug, Default)]
struct CommonAttributes {
    key: Option<String>,
    name: Option<String>,
    description: Option<String>,
    placeholder: Option<String>,
    hint: Option<String>,
    required: Option<bool>,
}

#[derive(Debug)]
enum ParameterTypeConfig {
    Auto, // Определяется автоматически по типу поля
    Text {
        min_length: Option<usize>,
        max_length: Option<usize>,
        pattern: Option<String>,
    },
    Textarea {
        rows: Option<usize>,
        min_length: Option<usize>,
        max_length: Option<usize>,
    },
    Select {
        options: Option<Vec<String>>,
        multiple: Option<bool>,
    },
    Checkbox {
        default: Option<bool>,
    },
    Secret {
        min_length: Option<usize>,
        max_length: Option<usize>,
    },
    // UI элементы
    Notice {
        notice_type: String,
        text: String,
    },
    Button {
        text: String,
        action: Option<String>,
        button_type: Option<String>,
    },
    Hidden,
}

impl Default for ParameterTypeConfig {
    fn default() -> Self {
        ParameterTypeConfig::Auto
    }
}

#[derive(Debug, Default)]
struct DisplayAttributes {
    show_when: Vec<FieldCondition>,
    hide_when: Vec<FieldCondition>,
}

#[derive(Debug)]
struct FieldCondition {
    field: String,
    operator: ConditionOperator,
    value: ConditionValue,
}

#[derive(Debug)]
enum ConditionOperator {
    Eq, NotEq, Gt, Gte, Lt, Lte,
    In, NotIn, Matches, Contains,
    StartsWith, EndsWith, Between,
    // Унарные
    NotEmpty, Empty, True, False,
}

#[derive(Debug)]
enum ConditionValue {
    Single(syn::Lit),
    Multiple(Vec<syn::Lit>),
    Range { from: syn::Lit, to: syn::Lit },
    None, // для унарных операторов
}

#[derive(Debug, Default)]
struct ValidationAttributes {
    rules: Vec<ValidationRule>,
}

#[derive(Debug)]
enum ValidationRule {
    NotEmpty,
    Empty,
    MinLength(usize),
    MaxLength(usize),
    Min(syn::Lit),
    Max(syn::Lit),
    Between { from: syn::Lit, to: syn::Lit },
    Regex(String),
    Email,
    Url,
    Custom(String),
}

pub fn derive_parameters_impl(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Parameters can only be derived for structs"
            ));
        }
    };

    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Parameters requires named fields"
            ));
        }
    };

    // Парсим все поля
    let mut field_configs = Vec::new();
    for field in fields {
        let config = parse_field(field)?;
        field_configs.push(config);
    }

    // Генерируем код
    let parameter_collection_impl = generate_parameter_collection(&field_configs)?;
    let from_values_impl = generate_from_values(struct_name, &field_configs)?;
    let to_values_impl = generate_to_values(&field_configs)?;

    let nebula_core = crate::nebula_core_path();

    Ok(quote! {
        impl #struct_name {
            pub fn parameter_collection() -> #nebula_core::ParameterCollection {
                #parameter_collection_impl
            }

            pub fn from_values(
                values: &std::collections::HashMap<#nebula_core::types::Key, #nebula_core::ParameterValue>
            ) -> Result<Self, #nebula_core::ParameterError> {
                #from_values_impl
            }

            pub fn to_values(&self) -> std::collections::HashMap<#nebula_core::types::Key, #nebula_core::ParameterValue> {
                #to_values_impl
            }
        }
    })
}

fn parse_field(field: &Field) -> Result<FieldConfig> {
    let field_name = field.ident.as_ref()
        .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
        .to_string();

    let field_type = field.ty.clone();

    // Создаем конфиг (передаем &str)
    let mut config = FieldConfig::new(&field_name, field_type);

    // Генерируем ключ
    config.common_attrs.key = Some(field_name_to_key(&field_name)?);

    // Парсим атрибуты
    for attr in &field.attrs {
        parse_attribute(attr, &mut config)?;
    }

    // Если тип параметра не указан, определяем автоматически
    if matches!(config.param_type, ParameterTypeConfig::Auto) {
        config.param_type = infer_parameter_type(&config.field_type)?;
    }

    Ok(config)
}

fn parse_attribute(attr: &Attribute, config: &mut FieldConfig) -> Result<()> {
    let path = attr.path();

    if path.is_ident("text") {
        config.param_type = parse_text_attribute(attr)?;
    } else if path.is_ident("textarea") {
        config.param_type = parse_textarea_attribute(attr)?;
    } else if path.is_ident("select") {
        config.param_type = parse_select_attribute(attr)?;
    } else if path.is_ident("checkbox") {
        config.param_type = parse_checkbox_attribute(attr)?;
    } else if path.is_ident("notice") {
        config.param_type = parse_notice_attribute(attr)?;
    } else if path.is_ident("button") {
        config.param_type = parse_button_attribute(attr)?;
    } else if path.is_ident("hidden") {
        config.param_type = ParameterTypeConfig::Hidden;
    } else if path.is_ident("display") {
        config.display_attrs = Some(parse_display_attribute(attr)?);
    } else if path.is_ident("validation") {
        config.validation_attrs = Some(parse_validation_attribute(attr)?);
    }
    // Игнорируем неизвестные атрибуты

    Ok(())
}

fn infer_parameter_type(field_type: &syn::Type) -> Result<ParameterTypeConfig> {
    match field_type {
        // Обработка Option<T>
        syn::Type::Path(type_path) if is_option_type(type_path) => {
            let inner_type = extract_option_inner_type(type_path)?;
            infer_parameter_type_for_inner(&inner_type)
        }

        // Обычные типы
        _ => infer_parameter_type_for_inner(field_type),
    }
}

fn is_option_type(type_path: &syn::TypePath) -> bool {
    // Проверяем что последний сегмент пути это "Option"
    type_path.path.segments.last()
        .map(|segment| segment.ident == "Option")
        .unwrap_or(false)
}

fn extract_option_inner_type(type_path: &syn::TypePath) -> Result<syn::Type> {
    let last_segment = type_path.path.segments.last()
        .ok_or_else(|| syn::Error::new_spanned(type_path, "Empty type path"))?;

    match &last_segment.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            if args.args.len() != 1 {
                return Err(syn::Error::new_spanned(args, "Option must have exactly one type argument"));
            }

            match &args.args[0] {
                syn::GenericArgument::Type(inner_type) => Ok(inner_type.clone()),
                _ => Err(syn::Error::new_spanned(&args.args[0], "Expected type argument")),
            }
        }
        _ => Err(syn::Error::new_spanned(last_segment, "Option must have type arguments")),
    }
}

fn infer_parameter_type_for_inner(field_type: &syn::Type) -> Result<ParameterTypeConfig> {
    match field_type {
        syn::Type::Path(type_path) => {
            let type_name = type_path.path.segments.last()
                .map(|seg| seg.ident.to_string())
                .unwrap_or_default();

            match type_name.as_str() {
                "bool" => Ok(ParameterTypeConfig::Checkbox { default: None }),
                "String" => Ok(ParameterTypeConfig::Text {
                    min_length: None,
                    max_length: None,
                    pattern: None,
                }),
                "i32" | "u32" | "f64" | "i64" | "u64" => Ok(ParameterTypeConfig::Text {
                    min_length: None,
                    max_length: None,
                    pattern: None,
                }),
                _ => Err(syn::Error::new_spanned(
                    field_type,
                    "Cannot infer parameter type, please add explicit attribute"
                )),
            }
        }
        syn::Type::Tuple(tuple) if tuple.elems.is_empty() => {
            Err(syn::Error::new_spanned(
                field_type,
                "Unit type fields must have explicit parameter type attribute"
            ))
        }
        _ => Err(syn::Error::new_spanned(
            field_type,
            "Cannot infer parameter type for this type"
        )),
    }
}

// Заглушки для парсинга конкретных атрибутов
fn parse_text_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    // TODO: Парсить text(min_length = 5, max_length = 100, ...)
    Ok(ParameterTypeConfig::Text {
        min_length: None,
        max_length: None,
        pattern: None,
    })
}

fn parse_textarea_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    Ok(ParameterTypeConfig::Textarea {
        rows: None,
        min_length: None,
        max_length: None,
    })
}

fn parse_select_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    Ok(ParameterTypeConfig::Select {
        options: None,
        multiple: None,
    })
}

fn parse_checkbox_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    Ok(ParameterTypeConfig::Checkbox { default: None })
}

fn parse_notice_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    Ok(ParameterTypeConfig::Notice {
        notice_type: "info".to_string(),
        text: "Notice".to_string(),
    })
}

fn parse_button_attribute(_attr: &Attribute) -> Result<ParameterTypeConfig> {
    Ok(ParameterTypeConfig::Button {
        text: "Button".to_string(),
        action: None,
        button_type: None,
    })
}

fn parse_display_attribute(_attr: &Attribute) -> Result<DisplayAttributes> {
    Ok(DisplayAttributes::default())
}

fn parse_validation_attribute(_attr: &Attribute) -> Result<ValidationAttributes> {
    Ok(ValidationAttributes::default())
}

// Заглушки для генерации
fn generate_parameter_collection(configs: &[FieldConfig]) -> Result<TokenStream> {
    let nebula_core = crate::nebula_core_path();

    Ok(quote! {
        let mut collection = #nebula_core::ParameterCollection::new();
        // TODO: добавить параметры из configs
        collection
    })
}

fn generate_from_values(
    struct_name: &syn::Ident,
    _configs: &[FieldConfig]
) -> Result<TokenStream> {
    Ok(quote! {
        Ok(#struct_name {
            // TODO: десериализация полей
        })
    })
}

fn generate_to_values(_configs: &[FieldConfig]) -> Result<TokenStream> {
    Ok(quote! {
        let mut values = std::collections::HashMap::new();
        // TODO: сериализация полей
        values
    })
}