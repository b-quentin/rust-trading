use binance::api::*;
use binance::market::Market;
use chrono::Utc;
use root::{strategy::{Backtester, ChoppinessDonchianAtrStrategy, Mode}, utils::{klines, times::Time}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let symbols = klines::get_symbols_ending_with_btc();
    //println!("{:?}", symbols);

    let market = Market::new(None, None);

    let now = Utc::now().to_rfc3339();
    let start_time = Time::from_str("2024-10-28T00:00:00Z")?;
    let end_time = Time::from_str(&now)?;

    match klines::get_klines_summary_in_range(&market, "ETHBTC", "1d", &start_time, &end_time) {
        Ok(klines1d) => {
            let strategy = ChoppinessDonchianAtrStrategy::new(Mode::Backtest, "ETHBTC");
            let mut backtester = Backtester::new(Box::new(strategy));
            backtester.run(&klines1d);
        }
        Err(e) => eprintln!("Erreur: {:?}", e),
    }

    Ok(())
}


    //match market.get_klines("ETHBTC", "1h", 999, None, None) {
    //    Ok(klines) => {   
    //        match klines {
    //            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
    //                let strategy = ChoppinessDonchianAtrStrategy::new(Mode::Backtest, "ETHBTC");
    //                let mut backtester = Backtester::new(Box::new(strategy));

    //                // Exécute le backtesting avec la stratégie spécifiée
    //                backtester.run(&klines);

    //                // Optionnel : Afficher les Klines pour vérifier
    //                //for kline in klines {
    //                //    println!(
    //                //        "Open: {}, High: {}, Low: {}",
    //                //        kline.open, kline.high, kline.low
    //                //    );
    //                //}
    //            }
    //        }
    //    },
    //    Err(e) => println!("Error: {}", e),
    //}
