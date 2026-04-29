use ftracker_core::domain::TradeRecord;
use ftracker_core::errors::CoreResult;
use ftracker_core::ports::TradeStore;

#[derive(Debug, Default)]
pub struct InMemoryTradeStore {
    records: Vec<TradeRecord>,
}

impl InMemoryTradeStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TradeStore for InMemoryTradeStore {
    fn save(&mut self, record: &TradeRecord) -> CoreResult<()> {
        self.records.push(record.clone());
        Ok(())
    }

    fn list_all(&self) -> CoreResult<Vec<TradeRecord>> {
        Ok(self.records.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use ftracker_core::domain::{OperationType, Quantity, Ticker, UnitPrice};
    use rust_decimal::Decimal;

    #[test]
    fn in_memory_store_is_empty_by_default() {
        let store = InMemoryTradeStore::new();
        assert!(store.list_all().unwrap().is_empty());
    }

    #[test]
    fn save_trade() {
        let mut trade1 = default_trade();
        trade1.quantity = Quantity::new(1000).unwrap();

        let mut trade2 = default_trade();
        trade2.operation_type = OperationType::Sell;
        trade2.ticker = Ticker::new("MSFT").unwrap();

        let mut store = InMemoryTradeStore::new();
        store.save(&trade1).unwrap();
        store.save(&trade2).unwrap();

        assert_eq!(store.list_all().unwrap(), vec![trade1, trade2]);
    }

    #[test]
    fn list_all_trades() {
        let trade = default_trade();

        let mut store = InMemoryTradeStore::new();
        store.save(&trade).unwrap();
        assert_eq!(store.list_all().unwrap(), vec![trade]);
    }

    fn default_trade() -> TradeRecord {
        TradeRecord {
            operation_type: OperationType::Buy,
            date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            ticker: Ticker::new("AAPL").unwrap(),
            quantity: Quantity::new(100).unwrap(),
            unit_price: UnitPrice::new(Decimal::new(10000, 2)).unwrap(),
        }
    }
}
