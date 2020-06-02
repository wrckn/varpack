/// App module, containing GTK-related GUI code
pub mod app;

/// Ptr module
pub mod ptr;

use std::{
    error::Error
};



fn main() -> Result<(), Box<dyn Error>> {
    app::run_app()?;
    Ok(())
}