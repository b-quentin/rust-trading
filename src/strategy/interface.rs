pub trait TradingStrategy {
    fn execute(&self, klines: &[binance::model::KlineSummary]);
}
