use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel};

use super::TradingStrategy;

pub struct Backtester {
    strategy: Box<dyn TradingStrategy>,
}

impl Backtester {
    pub fn new(strategy: Box<dyn TradingStrategy>) -> Self {
        Self { strategy }
    }

    pub fn run(&self, klines: &[binance::model::KlineSummary]) {
        println!("Running backtester...");
        let choppiness_index = ChoppinessIndex::new(klines, 100);
        let donchian_channel = DonchianChannel::new(klines, 20, 20);
        let atr_indicator = ATRStopLoss::new(klines, 12, 1.5);

        for i in 100..klines.len() {
            let current_klines = &klines[0..=i];
            self.strategy.execute(current_klines);
        }
    }
}
