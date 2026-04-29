//! The seam between raw CLI args and validated core domain types.
//! A desktop frontend would have its own equivalent of this module.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

use ftracker_core::domain::{OperationType, Quantity, Ticker, TradeRecord, UnitPrice};

use crate::cli::commands::add::{AddArgs, OperationTypeArg};
use crate::errors::{AppError, AppResult};

impl From<OperationTypeArg> for OperationType {
    fn from(arg: OperationTypeArg) -> Self {
        match arg {
            OperationTypeArg::Buy => OperationType::Buy,
            OperationTypeArg::Sell => OperationType::Sell,
        }
    }
}

pub fn parse_add_args(args: &AddArgs) -> AppResult<TradeRecord> {
    let operation_type = OperationType::from(args.operation);
    let date = parse_date(&args.date)?;
    let ticker = Ticker::new(&args.ticker).map_err(|e| AppError::Core(e.into()))?;
    let quantity = Quantity::new(args.quantity).map_err(|e| AppError::Core(e.into()))?;
    let unit_price = parse_unit_price(&args.unit_price)?;

    Ok(TradeRecord::new(
        operation_type,
        date,
        ticker,
        quantity,
        unit_price,
    ))
}

fn parse_date(raw: &str) -> AppResult<NaiveDate> {
    if raw == "today" {
        return Ok(chrono::Local::now().date_naive());
    }
    NaiveDate::parse_from_str(raw, "%Y-%m-%d").map_err(|_| AppError::InvalidDate(raw.to_string()))
}

fn parse_unit_price(raw: &str) -> AppResult<UnitPrice> {
    let decimal = Decimal::from_str(raw).map_err(|_| AppError::InvalidPrice(raw.to_string()))?;
    UnitPrice::new(decimal).map_err(|e| AppError::Core(e.into()))
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::add::OperationTypeArg;

    fn make_args(op: OperationTypeArg, date: &str, ticker: &str, qty: u64, price: &str) -> AddArgs {
        AddArgs {
            operation: op,
            date: date.to_string(),
            ticker: ticker.to_string(),
            quantity: qty,
            unit_price: price.to_string(),
        }
    }

    #[test]
    fn valid_buy_parses_correctly() {
        let record = parse_add_args(&make_args(
            OperationTypeArg::Buy,
            "2026-04-29",
            "AAPL",
            10,
            "182.50",
        ))
        .unwrap();
        assert_eq!(record.operation_type, OperationType::Buy);
        assert_eq!(record.ticker.as_str(), "AAPL");
        assert_eq!(record.quantity.value(), 10);
    }

    #[test]
    fn today_is_accepted() {
        assert!(parse_add_args(&make_args(
            OperationTypeArg::Sell,
            "today",
            "TSLA",
            5,
            "250.00"
        ))
        .is_ok());
    }

    #[test]
    fn invalid_date_returns_error() {
        assert!(matches!(
            parse_add_args(&make_args(
                OperationTypeArg::Buy,
                "29/04/2026",
                "AAPL",
                1,
                "100.00"
            )),
            Err(AppError::InvalidDate(_))
        ));
    }

    #[test]
    fn invalid_price_returns_error() {
        assert!(matches!(
            parse_add_args(&make_args(OperationTypeArg::Buy, "today", "AAPL", 1, "abc")),
            Err(AppError::InvalidPrice(_))
        ));
    }

    #[test]
    fn invalid_ticker_returns_core_error() {
        assert!(matches!(
            parse_add_args(&make_args(
                OperationTypeArg::Buy,
                "today",
                "!!!!",
                1,
                "10.00"
            )),
            Err(AppError::Core(_))
        ));
    }
}
