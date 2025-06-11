// crates/nebula_core/src/error/macros.rs

/// Creates new error with severity and message
#[macro_export]
macro_rules! error {
    ($severity:expr, $msg:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
    }};
    
    ($msg:expr) => {
        error!($crate::error::ErrorSeverity::Error, $msg)
    };
}

/// Creates error context with location
#[macro_export]
macro_rules! error_context {
    () => {
        $crate::error::ErrorContext::at_caller()
    };
    
    ($($key:expr => $value:expr),+ $(,)?) => {
        $crate::error::ErrorContext::at_caller()
            $(.with_data($key, $value.to_string()))+
    };
}

/// Implements HasSeverity for enum
#[macro_export]
macro_rules! impl_has_severity {
    ($enum:ident {
        $($variant:ident => $severity:expr),+ $(,)?
    }) => {
        impl $crate::error::HasSeverity for $enum {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant { .. } => $severity),+
                }
            }
        }
    };
}