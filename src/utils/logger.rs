use anyhow::Result;
use log::warn;

/// Set up the application logger (legacy function, now redirects to data::logger)
pub fn setup_logger() -> Result<()> {
    warn!("utils::logger::setup_logger is deprecated, use data::logger::init_logger instead");
    // This function is kept for backward compatibility
    // It will be removed in a future version
    Ok(())
}
