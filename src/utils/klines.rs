use std::error::Error;

use binance::{api::Binance, general::General, market::Market, model::KlineSummary};

use super::times::Time;

pub fn get_symbols_ending_with_btc() -> Vec<String> {
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

pub fn get_klines_summary_in_long_range(
    market: &Market,
    symbol: &str,
    interval: &str,
    start_time: &Time,
    end_time: &Time,
) -> Result<Vec<KlineSummary>, Box<dyn Error>> {
    let mut all_klines = Vec::new();
    let mut current_start = start_time.clone();

    while &current_start < end_time {
        // Utilise l'API `?` pour extraire directement les klines
        let klines = match market.get_klines(
            symbol,
            interval,
            999,
            current_start.get_timestamp(),
            None,
        )? {
            binance::model::KlineSummaries::AllKlineSummaries(k) => k,
        };

        if klines.is_empty() {
            break; // Arrête la boucle si aucun kline n'est retourné
        }

        all_klines.extend_from_slice(&klines);

        // Met à jour `current_start` pour la prochaine itération
        let last_kline_time = klines.last().unwrap().close_time as u64;
        current_start = Time::from_unix(last_kline_time + 1);
    }

    Ok(all_klines)
}


pub fn get_klines_summary_in_range(
    market: &Market,
    symbol: &str,
    interval: &str,
    start_time: &Time,
    end_time: &Time,
) -> Result<Vec<KlineSummary>, Box<dyn Error>> {
    // Utilisation de la méthode `get_klines` avec les temps de début et de fin
    let klines = match market.get_klines(
        symbol,
        interval,
        None,  // Pas de limite spécifiée ici
        start_time.get_timestamp(),
        end_time.get_timestamp(),
    )? {
        binance::model::KlineSummaries::AllKlineSummaries(k) => k,
    };

    Ok(klines)
}

