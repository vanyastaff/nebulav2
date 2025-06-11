
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod backtrace;
mod category;
mod context;
mod info;
mod macros;
mod severity;
mod traits;

pub use backtrace::BacktraceExt;
pub use category::ErrorCategory;
pub use context::ErrorContext;
pub use info::ErrorInfo;
pub use severity::ErrorSeverity;
pub use traits::{ErrorChain, HasSeverity, AnyError, ToErrorInfo};

#[doc(inline)]
pub use crate::{error, error_context, impl_has_severity};

/// Main result type with backtrace support
pub type AnyResult<T, E = Box<dyn AnyError>> = Result<T, E>;

/// Commonly used error constants
pub mod consts {
    use super::*;

    pub const INTERNAL_ERROR: ErrorInfo = ErrorInfo {
        severity: ErrorSeverity::Critical,
        category: ErrorCategory::Internal,
        message: std::borrow::Cow::Borrowed("Internal server error"),
        context: ErrorContext::default(),
    };
}