/// Creates a new error with severity, category and optional context
#[macro_export]
macro_rules! error {
    // Basic form: error!(message)
    ($msg:expr) => {
        $crate::error!($crate::error::ErrorSeverity::Error, $msg)
    };

    // With severity: error!(severity, message)
    ($severity:expr, $msg:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
    }};

    // With severity and category: error!(severity, category, message)
    ($severity:expr, $category:expr, $msg:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
            .with_category($category)
    }};

    // With context: error!(severity, message; context)
    ($severity:expr, $msg:expr; $ctx:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
            .with_context($ctx)
    }};

    // Full form: error!(severity, category, message; context)
    ($severity:expr, $category:expr, $msg:expr; $ctx:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
            .with_category($category)
            .with_context($ctx)
    }};
}

/// Creates error context with location and optional data
#[macro_export]
macro_rules! error_context {
    // Empty context with just location
    () => {
        $crate::error::ErrorContext::at_caller()
    };

    // With data: error_context!(key => value, ...)
    ($($key:expr => $value:expr),+ $(,)?) => {
        $crate::error::ErrorContext::at_caller()
            $(.with_data($key, $value.to_string()))+
    };

    // With backtrace: error_context!(backtrace)
    (backtrace) => {
        $crate::error::ErrorContext::with_forced_backtrace()
    };

    // With backtrace and data: error_context!(backtrace, key => value, ...)
    (backtrace, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::error::ErrorContext::with_forced_backtrace()
            $(.with_data($key, $value.to_string()))+
    };

    // With custom location: error_context!(location: file, line, column)
    (location: $file:expr, $line:expr, $col:expr) => {
        $crate::error::ErrorContext {
            source_location: Some(format!("{}:{}:{}", $file, $line, $col).into_boxed_str()),
            ..Default::default()
        }
    };
}

/// Creates critical error with full context
#[macro_export]
macro_rules! critical_error {
    // Basic form: critical_error!(message)
    ($msg:expr) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $msg
        ).with_context($crate::error_context!(backtrace))
    };

    // With data: critical_error!(message; key => value, ...)
    ($msg:expr; $($key:expr => $value:expr),+ $(,)?) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $msg
        ).with_context(
            $crate::error_context!(backtrace, $($key => $value),+)
        )
    };

    // With category: critical_error!(category, message)
    ($category:expr, $msg:expr) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $category,
            $msg
        ).with_context($crate::error_context!(backtrace))
    };

    // Full form: critical_error!(category, message; key => value, ...)
    ($category:expr, $msg:expr; $($key:expr => $value:expr),+ $(,)?) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $category,
            $msg
        ).with_context(
            $crate::error_context!(backtrace, $($key => $value),+)
        )
    };
}

/// Implements HasSeverity trait for enums with severity mapping
#[macro_export]
macro_rules! impl_has_severity {
    // Simple variant mapping to severity
    ($enum:ident {
        $($variant:ident => $severity:expr),+ $(,)?
    }) => {
        impl $crate::error::HasSeverity for $enum {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant => $severity),+,
                    _ => $crate::error::ErrorSeverity::Error,
                }
            }
        }
    };
    
    // With custom category
    ($enum:ident, $category:expr) => {
        impl $crate::error::HasSeverity for $enum {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                $crate::error::ErrorSeverity::Error
            }
            
            fn category(&self) -> $crate::error::ErrorCategory {
                $category
            }
        }
    };

    // With variant-specific category
    ($enum:ident {
        $($variant:ident => ($severity:expr, $category:expr)),+ $(,)?
    }) => {
        impl $crate::error::HasSeverity for $enum {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant => $severity),+,
                    _ => $crate::error::ErrorSeverity::Error,
                }
            }
            
            fn category(&self) -> $crate::error::ErrorCategory {
                match self {
                    $(Self::$variant => $category),+,
                    _ => $crate::error::ErrorCategory::General,
                }
            }
        }
    };
}

/// Creates a new error enum with automatic implementations
#[macro_export]
macro_rules! define_error {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident {
                    $($field:ident: $field_type:ty),* $(,)?
                } => ($severity:expr, $message:literal)
            ),+ $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant {
                    $($field: $field_type),*
                },
            )+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant {..} => write!(f, $message),
                    )+
                }
            }
        }

        impl std::error::Error for $name {}

        impl $crate::error::HasSeverity for $name {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant {..} => $severity),+
                }
            }
        }

        $crate::impl_error_helpers!($name {
            $(
                $variant => $variant {
                    $($field: $field_type),*
                }
            ),+
        });
    };
}

/// Implements error creation helpers
#[macro_export]
macro_rules! impl_error_helpers {
    ($error_type:ident {
        $(
            $helper_name:ident => $variant:ident {
                $($param:ident: $param_type:ty),* $(,)?
            }
        ),+ $(,)?
    }) => {
        impl $error_type {
            $(
                #[track_caller]
                pub fn $helper_name($($param: $param_type),*) -> Self {
                    Self::$variant {
                        $($param),*
                    }
                }
            )+
        }
    };
}

/// Early return with error
#[macro_export]
macro_rules! bail {
    // Simple error
    ($error:expr) => {
        return Err($crate::error::ToErrorInfo::to_error_info(&$error).into());
    };
    
    // Construct error variant
    ($error_type:ident :: $variant:ident { $($field:ident: $value:expr),* $(,)? }) => {
        return Err($error_type::$variant { $($field: $value),* }.into());
    };

    // With context
    ($error:expr; $ctx:expr) => {
        return Err($crate::error::ToErrorInfo::with_error_info(&$error, $ctx).into());
    };
}

/// Ensure condition or return error
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !($condition) {
            $crate::bail!($error);
        }
    };

    ($condition:expr, $error:expr; $ctx:expr) => {
        if !($condition) {
            $crate::bail!($error; $ctx);
        }
    };
}

/// Add context to Result error
#[macro_export]
macro_rules! context {
    ($result:expr, $ctx:expr) => {
        $result.map_err(|e| {
            $crate::error::ToErrorInfo::with_error_info(&e, $ctx)
        })
    };

    ($result:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $result.map_err(|e| {
            $crate::error::ToErrorInfo::with_error_info(
                &e,
                $crate::error_context!($($key => $value),+)
            )
        })
    };
}