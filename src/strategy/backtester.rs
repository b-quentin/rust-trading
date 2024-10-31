use binance::{api::Binance, market::Market, model::KlineSummary};

use crate::utils::{klines, times::Time};

use super::TradingStrategy;

#[allow(dead_code)]
pub struct Backtester {
    strategy: Box<dyn TradingStrategy>,
}

impl Backtester {
    pub fn new(strategy: Box<dyn TradingStrategy>) -> Self {
        Self { strategy }
    }

    pub fn run(&mut self, klines1d: &[KlineSummary]) {
        println!("Running backtester...");
        let (mut kline_manager_1d, mut kline_manager_1h) = self.strategy.prepare(klines1d[0].clone());

        let market: Market = Binance::new(None, None);

        for daily_kline in klines1d {
            //println!("----------------------------------------------------------------");
            //println!("{:?}", &Time::from_unix(daily_kline.open_time as u64).to_string());
            //println!("{:?}", &Time::from_unix(daily_kline.close_time as u64).to_string());

            match klines::get_klines_summary_in_range(
                &market,
                "SOLBTC",
                "1h",
                &Time::from_unix(daily_kline.open_time as u64),
                &Time::from_unix(daily_kline.close_time as u64),
            ) {
                Ok(klines_1h) => {
                    for kline_1h in klines_1h {
                        //println!("{:?}", &Time::from_unix(kline_1h.open_time as u64).to_string());
                        //println!("{:?}", &Time::from_unix(kline_1h.close_time as u64).to_string());
                        // Exécute la logique de stratégie avec kline1h
                        self.strategy.execute(kline_1h.clone(), &kline_manager_1d, &mut kline_manager_1h);
                    }
                }
                Err(e) => eprintln!("Erreur: {:?}", e),
            }
            self.strategy.execute_daily(daily_kline.clone(), &mut kline_manager_1d);

            //println!("{:?}", &Time::from_unix(daily_kline.close_time as u64).to_string());
        }
    }
}
