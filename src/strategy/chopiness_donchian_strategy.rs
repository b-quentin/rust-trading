use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel};

use super::{KlineManager, TradingStrategy};

pub struct ChoppinessDonchianAtrStrategy;

impl TradingStrategy for ChoppinessDonchianAtrStrategy {
    fn prepare(&self, klines: &[binance::model::KlineSummary]) -> KlineManager {
        println!("Preparing ChoppinessDonchianAtrStrategy...");

        let initial_klines: Vec<binance::model::KlineSummary> = klines[0..100].to_vec();
        let donchian_channel = Box::new(DonchianChannel::new(&initial_klines, 20, 20));
        let choppiness_index = Box::new(ChoppinessIndex::new(&initial_klines, 100));
        let atr_stop_loss = Box::new(ATRStopLoss::new(&initial_klines, 14, 1.5));

        KlineManager::new(initial_klines, vec![donchian_channel, choppiness_index, atr_stop_loss])
    }

    fn execute(&self, kline: binance::model::KlineSummary, manager: &mut KlineManager) {
        println!("Running ChoppinessDonchianAtrStrategy...");

        manager.add_kline(kline.clone());
        
        println!("last kline: {:?}", kline);
        // Récupérer le DonchianChannel et afficher `upper_band`
        if let Some(donchian_channel) = manager.get_donchian_channel() {
            println!("Upper Band: {:?}", donchian_channel.upper_band[donchian_channel.upper_band.len() - 1]);
        } else {
            println!("DonchianChannel non trouvé parmi les observateurs.");
        }
        if let Some(choppiness_index) = manager.get_choppiness_index() {
            println!("Choppiness Index Values: {:?}", choppiness_index.values[choppiness_index.values.len() - 1]);
        }
        if let Some(atr_stop_loss) = manager.get_atr_stop_loss() {
            println!("ATR Stop Loss: {:?}", atr_stop_loss.stop_losses[atr_stop_loss.stop_losses.len() - 1]);
        }
    }
}
