pub mod prelude;

pub use ne::*;

// TODO REMOVE
// I want a file of cfgs
pub const CONF_UI: bool = false;

/// tracing::Level::INFO, tracing::Level::ERROR, tracing::Level::WARN
pub fn run_engine(log_level: tracing::Level, title:&str)
{
    warn!("UI disabled!");

    if CONF_UI {
        info!("UI enabled");
    }
    else {
        info!("UI disabled!");
    }
    //initialize renderer, NOTE: hasn't been tested for wasm32
    pollster::block_on(ne_render::init_renderer(title));
}