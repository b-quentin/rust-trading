use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel};

use super::TradingStrategy;

pub struct ChoppinessDonchianAtrStrategy;

impl TradingStrategy for ChoppinessDonchianAtrStrategy {
    fn execute(&self, klines: &[binance::model::KlineSummary]) {
        println!("Running ChoppinessDonchianAtrStrategy...");

        let choppiness_index = ChoppinessIndex::new(klines, 100);
        let donchian_channel = DonchianChannel::new(klines, 20, 20);
        let donchian_channel = donchian_channel;
        let atr_indicator = ATRStopLoss::new(klines, 12, 1.5);
        
        println!("Choppiness Index: {:?}", choppiness_index);
        println!("Donchian Channel: {:?}", donchian_channel);
        println!("ATR Stop Loss Indicator: {:?}", atr_indicator);



        // Logique de décision de trading
    }
}
