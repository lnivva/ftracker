use crate::domain::TradeRecord;
use crate::errors::CoreResult;

pub trait TradeStore {
    fn save(&mut self, record: &TradeRecord) -> CoreResult<()>;
    fn list_all(&self) -> CoreResult<Vec<TradeRecord>>;
}
