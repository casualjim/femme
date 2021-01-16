//! Not just a pretty (inter)face.
//!
//! A pretty-printer and [ndjson](http://ndjson.org/) logger for the [log](https://docs.rs/log) crate.
//!
//! ## Examples
//! ```
//! femme::start();
//! log::warn!("Unauthorized access attempt on /login");
//! log::info!("Listening on port 8080");
//! ```

pub use log::LevelFilter;
use log::SetLoggerError;

#[cfg(not(target_arch = "wasm32"))]
mod ndjson;

#[cfg(not(target_arch = "wasm32"))]
mod pretty;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Starts logging depending on current environment.
///
/// # Log output
///
/// - when compiling with `--release` uses ndjson.
/// - pretty-prints otherwise.
/// - works in WASM out of the box.
///
/// # Examples
///
/// ```
/// femme::start();
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
pub fn start() {
    with_level(LevelFilter::Info);
}

/// Starts logging depending on current environment.
///
/// # Log output
///
/// - when compiling with `--release` uses ndjson.
/// - pretty-prints otherwise.
/// - works in WASM out of the box.
///
/// # Examples
///
/// ```
/// femme::try_start()?;
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
pub fn try_start() -> Result<(), SetLoggerError> {
    try_with_level(LevelFilter::Info)
}

/// Start logging with a log level.
///
/// All messages under the specified log level will statically be filtered out.
///
/// # Examples
/// ```
/// femme::with_level(log::LevelFilter::Trace);
/// ```
pub fn with_level(level: log::LevelFilter) {
    try_with_level(level).expect("Could not start logging")
}

/// Start logging with a log level.
///
/// All messages under the specified log level will statically be filtered out.
///
/// # Examples
/// ```
/// femme::try_with_level(log::LevelFilter::Trace)?;
/// ```
pub fn try_with_level(level: log::LevelFilter) -> Result<(), SetLoggerError> {
    #[cfg(target_arch = "wasm32")]
    return wasm::try_start(level);

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use ndjson in release mode, pretty logging while debugging.
        if cfg!(debug_assertions) {
            return pretty::try_start(level)
        } else {
            return ndjson::try_start(level)
        }
    }
}
