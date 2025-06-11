mod category;
mod context;
mod info;
mod macros;
mod severity;
mod traits;

pub use category::ErrorCategory;
pub use context::ErrorContext;
pub use info::ErrorInfo;
pub use severity::ErrorSeverity;
pub use traits::{HasSeverity, ToErrorInfo};

// Re-export macros for convenience
pub use macros::{error_context, error_info, impl_has_severity};

pub type NebulaResult<T> = Result<T, Box<dyn HasSeverity + Send + Sync>>;