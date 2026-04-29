use crate::cli::commands::add::AddArgs;
use crate::cli::commands::parse::parse_add_args;
use crate::context::AppContext;
use crate::errors::AppResult;

pub fn run(args: &AddArgs, ctx: &mut AppContext) -> AppResult<()> {
    let record = parse_add_args(args)?;
    ctx.trade_service.add_trade(record.clone())?;
    println!("{record}");
    Ok(())
}
