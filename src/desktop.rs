use serde::de::DeserializeOwned;
use tauri::{ plugin::PluginApi, AppHandle, Runtime };

use crate::models::*;

/// Access to the liquid-glass APIs.
pub struct LiquidGlass<R: Runtime>(AppHandle<R>);

impl<R: Runtime> LiquidGlass<R> {
    pub fn set_window_chrome(&self, payload: SetWindowChromeRequest) -> crate::Result<()> {
        use objc2::{ rc::Retained, MainThreadOnly };
        use objc2_app_kit::{ NSToolbar, NSWindow, NSWindowToolbarStyle };
        use objc2_foundation::{ ns_string, MainThreadMarker };

        let label = payload.window_label.unwrap_or_else(|| "main".to_string());
        let window = self.0
            .get_webview_window(&label)
            .ok_or_else(|| crate::Error::WindowNotFound(label))?;

        unsafe {
            let mtm = MainThreadMarker::new().expect("must be on main thread");
            let ns_window = window
                .ns_window()
                .map_err(|_| crate::Error::NsWindowFailed)? as *mut NSWindow;

            let toolbar: Retained<NSToolbar> = NSToolbar::initWithIdentifier(
                NSToolbar::alloc(mtm),
                ns_string!("main_toolbar")
            );

            (*ns_window).setToolbar(Some(&toolbar));
            (*ns_window).setToolbarStyle(NSWindowToolbarStyle::Unified);
        }

        Ok(())
    }

    pub fn apply_liquid_glass(&self, payload: ApplyLiquidGlassRequest) -> crate::Result<()> {
        Ok(())
    }

    pub fn set_liquid_glass_style(&self, payload: SetLiquidGlassStyleRequest) -> crate::Result<()> {
        Ok(())
    }
}
