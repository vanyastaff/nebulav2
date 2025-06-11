/// Creates new error with severity and message
#[macro_export]
macro_rules! error {
    ($severity:expr, $msg:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
    }};
    
    ($msg:expr) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Error, 
            $msg
        )
    };

    ($severity:expr, $category:expr, $msg:expr) => {{
        $crate::error::ErrorInfo::new($severity, $msg)
            .with_category($category)
    }};
}

/// Creates error context with location and optional data
#[macro_export]
macro_rules! error_context {
    () => {
        $crate::error::ErrorContext::at_caller()
    };
    
    ($($key:expr => $value:expr),+ $(,)?) => {
        $crate::error::ErrorContext::at_caller()
            $(.with_data($key, $value.to_string()))+
    };

    (backtrace) => {
        $crate::error::ErrorContext::with_debug_backtrace()
    };

    (backtrace, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::error::ErrorContext::with_debug_backtrace()
            $(.with_data($key, $value.to_string()))+
    };
}

/// Creates critical error with full context
#[macro_export]
macro_rules! critical_error {
    ($msg:expr) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $msg
        ).with_context($crate::error_context!(backtrace))
    };

    ($msg:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::error!(
            $crate::error::ErrorSeverity::Critical,
            $msg
        ).with_context(
            $crate::error_context!(backtrace, $($key => $value),+)
        )
    };
}

/// Implements HasSeverity for enum with better error messages
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

        impl $crate::error::AnyError for $enum {}
    };

    ($enum:ident {
        $($variant:ident => ($severity:expr, $category:expr)),+ $(,)?
    }) => {
        impl $crate::error::HasSeverity for $enum {
            fn severity(&self) -> $crate::error::ErrorSeverity {
                match self {
                    $(Self::$variant { .. } => $severity),+
                }
            }

            fn category(&self) -> $crate::error::ErrorCategory {
                match self {
                    $(Self::$variant { .. } => $category),+
                }
            }
        }

        impl $crate::error::AnyError for $enum {}
    };
}