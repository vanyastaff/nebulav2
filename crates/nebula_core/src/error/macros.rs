#[macro_export]
macro_rules! error_context {
    () => {
        $crate::error::ErrorContext::new().with_location(format!("{}:{}", file!(), line!()))
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut context = $crate::error::ErrorContext::new().with_location(format!("{}:{}", file!(), line!()));
            $(
                context = context.with_data($key, $value);
            )+
            context
        }
    };
}

/// Helper for creating error info quickly
#[macro_export]
macro_rules! error_info {
    ($severity:expr, $code:expr, $message:expr) => {
        $crate::error::ErrorInfo::new($severity, $code, $message)
    };
    ($severity:expr, $code:expr, $message:expr, $($method:ident($value:expr)),+ $(,)?) => {
        {
            let mut info = $crate::error::ErrorInfo::new($severity, $code, $message);
            $(
                info = info.$method($value);
            )+
            info
        }
    };
}

/// Macro to implement HasSeverity for error enums
#[macro_export]
macro_rules! impl_has_severity {
    (
        $error_type:ident {
            $(
                $variant:ident => {
                    severity: $severity:expr,
                    code: $code:expr,
                    category: $category:expr
                    $(, user_message: $user_msg:expr)?
                    $(, suggestions: [$($suggestion:expr),*])?
                    $(, help_links: [$($help_link:expr),*])?
                    $(, tags: [$($tag:expr),*])?
                    $(, recoverable: $recoverable:expr)?
                }
            ),* $(,)?
        }
    ) => {
        impl $crate::error::HasSeverity for $error_type {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant { .. } => $severity),*
                }
            }

            fn error_code(&self) -> &'static str {
                match self {
                    $(Self::$variant { .. } => $code),*
                }
            }

            fn category(&self) -> $crate::error::ErrorCategory {
                match self {
                    $(Self::$variant { .. } => $category),*
                }
            }

            fn user_message(&self) -> Option<String> {
                match self {
                    $(
                        Self::$variant { .. } => {
                            #[allow(unreachable_code)]
                            {
                                $(return Some($user_msg.to_string());)?
                                None
                            }
                        }
                    )*
                }
            }

            fn suggestions(&self) -> Vec<String> {
                match self {
                    $(
                        Self::$variant { .. } => {
                            #[allow(unreachable_code)]
                            {
                                $(return vec![$($suggestion.to_string()),*];)?
                                Vec::new()
                            }
                        }
                    )*
                }
            }

            fn help_links(&self) -> Vec<String> {
                match self {
                    $(
                        Self::$variant { .. } => {
                            #[allow(unreachable_code)]
                            {
                                $(return vec![$($help_link.to_string()),*];)?
                                Vec::new()
                            }
                        }
                    )*
                }
            }

            fn tags(&self) -> Vec<String> {
                match self {
                    $(
                        Self::$variant { .. } => {
                            #[allow(unreachable_code)]
                            {
                                $(return vec![$($tag.to_string()),*];)?
                                Vec::new()
                            }
                        }
                    )*
                }
            }

            fn is_recoverable(&self) -> bool {
                match self {
                    $(
                        Self::$variant { .. } => {
                            #[allow(unreachable_code)]
                            {
                                $(return $recoverable;)?
                                false
                            }
                        }
                    )*
                }
            }
        }
    };
}

/// Macro to generate error constructors
#[macro_export]
macro_rules! error_constructor {
    ($name:ident, $field:ident: $type:ty) => {
        pub fn $name($field: $type) -> Self {
            Self::$name { $field }
        }
    };
    ($name:ident, $($field:ident: $type:ty),+) => {
        pub fn $name($($field: $type),+) -> Self {
            Self::$name { $($field),+ }
        }
    };
}