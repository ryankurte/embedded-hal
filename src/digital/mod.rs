//! Digital I/O
//!
//! 
//!

// Deprecated / infallible traits
#[allow(deprecated)]
#[deprecated(since = "0.2.2", note = "Deprecated because the methods cannot return errors. \
                                      Users should use the traits in digital::v2.")]
pub use embedded_hal_v3::digital::v1;

// New / fallible traits
pub use embedded_hal_v3::digital::v2;

// v2 -> v1 compatibility wrappers
// These require explicit casts from v2 -> v1
// and implemented in the v3 crate due to E0210
pub use embedded_hal_v3::digital::v1_compat;

// v1 -> v2 compatibility shims
// These are implicit over v1 implementations
// and implemented in the v3 crate due to E0210
pub use embedded_hal_v3::digital::v2_compat;

// Re-export old traits so this isn't a breaking change
#[allow(deprecated)]
pub use self::v1::*;

