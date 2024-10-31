use std::any::Any;

use super::KlineManager;

pub trait TradingStrategy {
    fn prepare(&self, kline: binance::model::KlineSummary) -> (KlineManager, KlineManager);
    fn execute_daily(&mut self, klines: binance::model::KlineSummary, manager1d: &mut KlineManager);
    fn execute(&mut self, klines: binance::model::KlineSummary, manager1d: &KlineManager, manager1h: &mut KlineManager);
}

pub trait Observer {
    fn on_new_kline(&mut self, kline: &binance::model::KlineSummary, all_klines: &[binance::model::KlineSummary]);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any; // New mutable method
    fn id(&self) -> &str;
}
