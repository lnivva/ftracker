use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticker(String);

impl Ticker {
    pub fn new(raw: &str) -> Result<Self, TradeRecordError> {
        let t = raw.trim().to_uppercase();
        if t.is_empty() || t.len() > 10 {
            return Err(TradeRecordError::InvalidTicker(raw.to_string()));
        }
        if !t.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TradeRecordError::InvalidTicker(raw.to_string()));
        }
        Ok(Self(t))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantity(u64);

impl Quantity {
    pub fn new(value: u64) -> Result<Self, TradeRecordError> {
        if value == 0 {
            return Err(TradeRecordError::ZeroQuantity);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitPrice(Decimal);

impl UnitPrice {
    pub fn new(value: Decimal) -> Result<Self, TradeRecordError> {
        if value <= Decimal::ZERO {
            return Err(TradeRecordError::NonPositivePrice(value));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> Decimal {
        self.0
    }
}

impl fmt::Display for UnitPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Buy,
    Sell,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationType::Buy => write!(f, "buy"),
            OperationType::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeRecord {
    pub operation_type: OperationType,
    pub date: NaiveDate,
    pub ticker: Ticker,
    pub quantity: Quantity,
    pub unit_price: UnitPrice,
}

impl TradeRecord {
    pub fn new(
        operation_type: OperationType,
        date: NaiveDate,
        ticker: Ticker,
        quantity: Quantity,
        unit_price: UnitPrice,
    ) -> Self {
        Self {
            operation_type,
            date,
            ticker,
            quantity,
            unit_price,
        }
    }

    pub fn trade(
        op: OperationType,
        date: (i32, u32, u32),
        ticker: &str,
        qty: u64,
        price_cents: i64,
    ) -> TradeRecord {
        TradeRecord {
            operation_type: op,
            date: NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
            ticker: Ticker::new(ticker).unwrap(),
            quantity: Quantity::new(qty).unwrap(),
            unit_price: UnitPrice::new(Decimal::new(price_cents, 2)).unwrap(),
        }
    }

    pub fn total_value(&self) -> Decimal {
        Decimal::from(self.quantity.value()) * self.unit_price.value()
    }
}

impl fmt::Display for TradeRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} {} x {} @ {}",
            self.date, self.operation_type, self.ticker, self.quantity, self.unit_price,
        )
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TradeRecordError {
    #[error("invalid ticker '{0}': must be 1–10 uppercase ASCII letters")]
    InvalidTicker(String),

    #[error("quantity must be greater than zero")]
    ZeroQuantity,

    #[error("unit price must be positive, got {0}")]
    NonPositivePrice(Decimal),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn valid_record() -> TradeRecord {
        TradeRecord::trade(OperationType::Buy, (2026, 4, 29), "AAPL", 10, 18250)
    }

    #[test]
    fn total_value_is_quantity_times_price() {
        assert_eq!(valid_record().total_value(), dec!(1825.00));
    }

    #[test]
    fn ticker_normalises_to_uppercase() {
        assert_eq!(Ticker::new("aapl").unwrap().as_str(), "AAPL");
    }

    #[test]
    fn ticker_rejects_empty() {
        assert!(Ticker::new("").is_err());
    }

    #[test]
    fn ticker_rejects_too_long() {
        assert!(Ticker::new("TOOLONGTICKER").is_err());
    }

    #[test]
    fn quantity_rejects_zero() {
        assert!(Quantity::new(0).is_err());
    }

    #[test]
    fn unit_price_rejects_zero_and_negative() {
        assert!(UnitPrice::new(dec!(0)).is_err());
        assert!(UnitPrice::new(dec!(-1.00)).is_err());
    }
}
