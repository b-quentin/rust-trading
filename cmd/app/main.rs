use binance::api::*;
use binance::market::Market;
use chrono::Utc;
use root::{strategy::{Backtester, ChoppinessDonchianAtrStrategy, Mode}, utils::{klines, times::Time}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let symbols = klines::get_symbols_ending_with_btc();
    //println!("{:?}", symbols);

    let market = Market::new(None, None);

    let now = Utc::now().to_rfc3339();
    let start_time = Time::from_str("2024-01-01T00:00:00Z")?;
    let end_time = Time::from_str(&now)?;
    //let start_time = Time::from_str("2024-09-01T00:00:00Z")?;
    //let end_time = Time::from_str("2024-10-01T00:00:00Z")?;

    match klines::get_klines_summary_in_long_range(&market, "SOLBTC", "1d", &start_time, &end_time) {
        Ok(klines1d) => {
            let strategy = ChoppinessDonchianAtrStrategy::new(Mode::Backtest, "ETHBTC");
            let mut backtester = Backtester::new(Box::new(strategy));
            backtester.run(&klines1d);
        }
        Err(e) => eprintln!("Erreur: {:?}", e),
    }

    Ok(())
}
