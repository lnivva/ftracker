use ftracker_core::services::TradeService;
use ftracker_infra::in_memory::InMemoryTradeStore;

use crate::config::AppConfig;
use crate::errors::AppResult;

pub struct AppContext {
    pub config: AppConfig,
    pub trade_service: TradeService<InMemoryTradeStore>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> AppResult<Self> {
        let store = InMemoryTradeStore::new();
        let trade_service = TradeService::new(store);
        Ok(Self {
            config,
            trade_service,
        })
    }
}
