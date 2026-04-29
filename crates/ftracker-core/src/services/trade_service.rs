use crate::domain::TradeRecord;
use crate::errors::CoreResult;
use crate::ports::TradeStore;

pub struct TradeService<S: TradeStore> {
    store: S,
}

impl<S: TradeStore> TradeService<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn add_trade(&mut self, record: TradeRecord) -> CoreResult<()> {
        // Business rules (duplicate checks, validation beyond types) go here.
        self.store.save(&record)
    }

    pub fn list_trades(&self) -> CoreResult<Vec<TradeRecord>> {
        self.store.list_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{OperationType, TradeRecord};
    use crate::errors::CoreResult;

    struct InMemoryStore(Vec<TradeRecord>);

    impl TradeStore for InMemoryStore {
        fn save(&mut self, record: &TradeRecord) -> CoreResult<()> {
            self.0.push(record.clone());
            Ok(())
        }
        fn list_all(&self) -> CoreResult<Vec<TradeRecord>> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn add_trade_persists_record() {
        let mut svc = TradeService::new(InMemoryStore(vec![]));
        let trade = TradeRecord::trade(OperationType::Buy, (2026, 4, 29), "AAPL", 10, 18250);
        svc.add_trade(trade).unwrap();
        assert_eq!(svc.list_trades().unwrap().len(), 1);
    }
}
