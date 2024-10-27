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
        let mut kline_manager = self.strategy.prepare(klines);

        for i in 101..klines.len() {
            let current_kline = &klines[i-1];
            self.strategy.execute(current_kline.clone(), &mut kline_manager);
        }
    }
}
