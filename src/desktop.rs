use serde::de::DeserializeOwned;
use tauri::{ plugin::PluginApi, AppHandle, Runtime };

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>
) -> crate::Result<LiquidGlass<R>> {
    Ok(LiquidGlass(app.clone()))
}

/// Access to the liquid-glass APIs.
pub struct LiquidGlass<R: Runtime>(AppHandle<R>);

impl<R: Runtime> LiquidGlass<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
