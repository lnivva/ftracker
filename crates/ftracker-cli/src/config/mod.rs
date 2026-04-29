mod schema;

pub use schema::AppConfig;

use crate::errors::AppResult;
use std::path::Path;

pub fn load(config_path: Option<&Path>) -> AppResult<AppConfig> {
    let _ = config_path;
    Ok(AppConfig::default())
}
