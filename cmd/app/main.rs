use binance::api::*;
use binance::market::Market;
use root::{indicator::{ChoppinessIndex, EMA}, utils::klines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = klines::get_symbols_ending_with_btc();
    let trading_pairs = [
        "ETHBTC",
        "BNBBTC",
        "LTCBTC",
        "XRPBTC",
        "ADABTC",
        "DOTBTC",
        "AVAXBTC",
        "LINKBTC",
        "SOLBTC",
        "MATICBTC",
        "DOGEBTC",
        "NEARBTC",
        "FILBTC",
        "AAVEBTC",
        "ATOMBTC",
        "FTMBTC",
        "KSMBTC",
        "EGLDBTC",
        "HBARBTC",
        "CHZBTC",
        "SUSHIBTC",
        "UNIBTC",
        "CRVBTC",
        "ZECBTC",
        "COMPBTC",
        "SNXBTC",
        "YFIBTC",
        "RUNEBTC",
        "ALPHABTC",
    ];
    println!("Symbols: {:?}", symbols);

    let market = Market::new(None, None);

    for symbol in trading_pairs {
        // Retrieve 1-hour klines for the Choppiness Index
        let choppiness_index = match market.get_klines(symbol, "1h", 100, None, None) {
            Ok(klines) => match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines1h) => {
                    ChoppinessIndex::new(&klines1h, 100).get_last_value().unwrap_or_default()
                },
            },
            Err(e) => {
                eprintln!("Error fetching 1h klines for {}: {:?}", symbol, e);
                continue;
            },
        };

        // Only proceed if Choppiness Index is below 50
        if choppiness_index >= 50.0 {
            continue;
        }

        // Retrieve 15-minute klines for EMA calculations
        let (ema_50, ema_200) = match market.get_klines(symbol, "15m", 200, None, None) {
            Ok(klines) => match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines15m) => {
                    let ema_50 = EMA::new(&klines15m, 50, 0).get_last_value().unwrap_or_default();
                    let ema_200 = EMA::new(&klines15m, 200, 0).get_last_value().unwrap_or_default();
                    (ema_50, ema_200)
                },
            },
            Err(e) => {
                eprintln!("Error fetching 15m klines for {}: {:?}", symbol, e);
                continue;
            },
        };

        // Print symbol if ema_50 is greater than ema_200
        if ema_50 > ema_200 {
            println!("True");
            println!(
                "Symbol: {}, choppiness_index: {:.5}, ema_50: {:.5}, ema_200: {:.2}",
                symbol, choppiness_index, ema_50, ema_200
            );
        } else {
            println!("False");
            println!(
                "Symbol: {}, choppiness_index: {:.5}, ema_50: {:.5}, ema_200: {:.2}",
                symbol, choppiness_index, ema_50, ema_200
            );
        }
    }

    Ok(())
}

