use binance::model::KlineSummary;

#[derive(Debug)]
pub struct ChoppinessIndex {
    pub value: f64,
}

impl ChoppinessIndex {
    pub fn new(klines: &[KlineSummary], length: usize) -> Self {
        let atr_sum = Self::calculate_atr_sum(klines, length);
        let highest = Self::get_highest(klines, length);
        let lowest = Self::get_lowest(klines, length);
        let value = Self::calculate_choppiness_index(atr_sum, highest, lowest, length);

        Self { value } 
    }

    fn calculate_true_range(current: &KlineSummary, previous_close: f64) -> f64 {
        let high = current.high.parse::<f64>().unwrap_or(0.0);
        let low = current.low.parse::<f64>().unwrap_or(0.0);
        (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs())
    }

    fn calculate_atr_sum(klines: &[KlineSummary], length: usize) -> f64 {
        let mut true_ranges = Vec::new();
        for i in 1..=length.min(klines.len() - 1) {
            let current = &klines[i];
            let previous_close = klines[i - 1].close.parse::<f64>().unwrap_or(0.0);
            let tr = Self::calculate_true_range(current, previous_close);
            true_ranges.push(tr);
        }
        true_ranges.iter().sum()
    }

    fn get_highest(klines: &[KlineSummary], length: usize) -> f64 {
        klines.iter()
            .take(length)
            .map(|kline| kline.high.parse::<f64>().unwrap_or(0.0))
            .fold(f64::MIN, |a, b| a.max(b))
    }

    fn get_lowest(klines: &[KlineSummary], length: usize) -> f64 {
        klines.iter()
            .take(length)
            .map(|kline| kline.low.parse::<f64>().unwrap_or(f64::MAX))
            .fold(f64::MAX, |a, b| a.min(b))
    }

    fn calculate_choppiness_index(atr_sum: f64, highest: f64, lowest: f64, length: usize) -> f64 {
        if highest - lowest == 0.0 {
            return 0.0; // Avoid division by zero
        }

        let ratio = atr_sum / (highest - lowest);
        let log10_ratio = ratio.log10();
        let log10_length = (length as f64).log10();

        100.0 * log10_ratio / log10_length
    }
}
