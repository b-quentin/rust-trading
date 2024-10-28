use std::error::Error;

use binance::model::KlineSummary;
use binance::api::*;
use binance::general::General;
use binance::market::Market;
use chrono::{DateTime, Utc};
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

fn parse_datetime_to_unix(date: &str) -> Result<u64, &'static str> {
    let parsed = DateTime::parse_from_rfc3339(date)
        .map_err(|_| "Invalid date format. Please use RFC3339 format (e.g., 2023-01-01T00:00:00Z)")?
        .timestamp();
    Ok(parsed as u64 * 1000) // Convertir en millisecondes
}

pub fn get_klines_summary_in_range(
    market: &Market,
    symbol: &str,
    interval: &str,
    start_time: &str,
    end_time: &str,
) -> Result<Vec<KlineSummary>, Box<dyn Error>> {
    let mut all_klines = Vec::new();
    
    // Convert the start and end time from string to UNIX timestamps
    let start_timestamp = parse_datetime_to_unix(start_time)?;
    let end_timestamp = parse_datetime_to_unix(end_time)?;
    
    let mut current_start = start_timestamp;
    
    while current_start < end_timestamp {
        // Utilisez match pour gérer KlineSummaries
        let klines = match market.get_klines(
            symbol,
            interval,
            999,
            Some(current_start),
            None,
        )? {
            binance::model::KlineSummaries::AllKlineSummaries(k) => k,
        };
        
        // Si aucun kline n'est retourné, arrêter la boucle
        if klines.is_empty() {
            break;
        }
        
        // Ajoute les klines récupérés à la liste complète
        all_klines.extend(klines.clone());
        
        // Met à jour la valeur de current_start pour la prochaine itération
        let last_kline_time = klines.last().unwrap().close_time as u64;
        current_start = last_kline_time + 1;
    }
    
    Ok(all_klines)
}

fn main() {
    let _symbols = get_symbols_ending_with_btc();
    //println!("{:?}", symbols);
    let market: Market = Binance::new(None, None);

    let now = Utc::now().to_rfc3339();

    match get_klines_summary_in_range(&market, "ETHBTC", "1h", "2024-01-01T00:00:00Z", &now) {
        Ok(klines) => {
            let strategy = ChoppinessDonchianAtrStrategy::new(Mode::Backtest, "ETHBTC");
            let mut backtester = Backtester::new(Box::new(strategy));
            backtester.run(&klines);
        }
        Err(e) => eprintln!("Erreur: {:?}", e),
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
}
