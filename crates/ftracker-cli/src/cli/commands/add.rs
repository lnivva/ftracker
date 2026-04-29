use clap::Args;

/// Raw CLI arguments for `ft add`. Primitives only — no domain types.
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Operation type
    #[arg(short = 'o', long, value_enum)]
    pub operation: OperationTypeArg,

    /// Trade date in YYYY-MM-DD format (defaults to today)
    #[arg(short, long, value_name = "DATE", default_value = "today")]
    pub date: String,

    /// Stock ticker symbol (e.g. AAPL, MSFT)
    #[arg(short, long, value_name = "TICKER")]
    pub ticker: String,

    /// Number of units traded
    #[arg(short, long, value_name = "QUANTITY")]
    pub quantity: u64,

    /// Price per unit (e.g. 182.50)
    #[arg(short = 'p', long, value_name = "PRICE")]
    pub unit_price: String,
}

/// Clap-level mirror of `OperationType`. Decoupled so CLI flags can change
/// without touching the core domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OperationTypeArg {
    Buy,
    Sell,
}
