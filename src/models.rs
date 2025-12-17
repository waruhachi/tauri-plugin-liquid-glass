use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWindowChromeRequest {
    pub window_label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyLiquidGlassRequest {
    pub ns_window: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLiquidGlassStyleRequest {
    pub style: Option<i64>,
}
