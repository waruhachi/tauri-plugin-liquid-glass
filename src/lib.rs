use tauri::{ plugin::{ Builder, TauriPlugin }, Manager, Runtime };

pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;

pub use error::{ Error, Result };

#[cfg(desktop)]
use desktop::LiquidGlass;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the liquid-glass APIs.
pub trait LiquidGlassExt<R: Runtime> {
    fn liquid_glass(&self) -> &LiquidGlass<R>;
}

impl<R: Runtime, T: Manager<R>> crate::LiquidGlassExt<R> for T {
    fn liquid_glass(&self) -> &LiquidGlass<R> {
        self.state::<LiquidGlass<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("liquid-glass")
        .invoke_handler(
            tauri::generate_handler![
                commands::set_window_chrome,
                commands::apply_liquid_glass,
                commands::set_liquid_glass_style
            ]
        )
        .build()
}
