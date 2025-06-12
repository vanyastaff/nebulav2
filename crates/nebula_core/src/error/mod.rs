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
pub use crate::{error, error_context, impl_has_severity, critical_error};

/// Main result type with backtrace support
pub type AnyResult<T> = anyhow::Result<T>;