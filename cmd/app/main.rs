use binance::api::*;
use binance::general::General;
use binance::market::Market;
use root::strategy::{Backtester, ChoppinessDonchianAtrStrategy, Mode};

fn get_symbols_ending_with_btc() -> Vec<String> {
    let general: General = Binance::new(None, None);
    
    match general.exchange_info() {
        Ok(result) => {
            result.symbols
                .into_iter()
                .filter(|symbol| symbol.symbol.ends_with("BTC"))
                .map(|symbol| symbol.symbol)
                .collect()
        }
        Err(err) => {
            println!("Erreur : {:?}", err);
            Vec::new() // Retourne un vecteur vide en cas d'erreur
        }
    }
}

fn main() {
    let _symbols = get_symbols_ending_with_btc();
    //println!("{:?}", symbols);
    let market: Market = Binance::new(None, None);

    match market.get_klines("ETHBTC", "1h", 999, None, None) {
        Ok(klines) => {   
            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    let strategy = ChoppinessDonchianAtrStrategy::new(Mode::Backtest, "ETHBTC");
                    let mut backtester = Backtester::new(Box::new(strategy));

                    // Exécute le backtesting avec la stratégie spécifiée
                    backtester.run(&klines);

                    // Optionnel : Afficher les Klines pour vérifier
                    //for kline in klines {
                    //    println!(
                    //        "Open: {}, High: {}, Low: {}",
                    //        kline.open, kline.high, kline.low
                    //    );
                    //}
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
