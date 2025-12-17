use tauri::{ AppHandle, command, Runtime };

use crate::models::*;
use crate::Result;
use crate::LiquidGlassExt;

#[command]
pub(crate) async fn set_window_chrome<R: Runtime>(
    app: AppHandle<R>,
    payload: SetWindowChromeRequest
) -> Result<()> {
    app.liquid_glass().set_window_chrome(payload)
}

#[command]
pub(crate) async fn apply_liquid_glass<R: Runtime>(
    app: AppHandle<R>,
    payload: ApplyLiquidGlassRequest
) -> Result<()> {
    app.liquid_glass().apply_liquid_glass(payload)
}

#[command]
pub(crate) async fn set_liquid_glass_style<R: Runtime>(
    app: AppHandle<R>,
    payload: SetLiquidGlassStyleRequest
) -> Result<()> {
    app.liquid_glass().set_liquid_glass_style(payload)
}
