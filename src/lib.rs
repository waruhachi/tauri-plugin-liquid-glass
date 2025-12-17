use tauri::{ plugin::{ Builder, TauriPlugin }, Manager, Runtime };

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{ Error, Result };

#[cfg(desktop)]
use desktop::LiquidGlass;
#[cfg(mobile)]
use mobile::LiquidGlass;

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
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(mobile)]
            let liquid_glass = mobile::init(app, api)?;
            #[cfg(desktop)]
            let liquid_glass = desktop::init(app, api)?;
            app.manage(liquid_glass);
            Ok(())
        })
        .build()
}
