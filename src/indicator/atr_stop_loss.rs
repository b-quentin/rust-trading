use binance::model::KlineSummary;

#[derive(Debug)]
pub struct ATRStopLoss {
    pub stop_loss: f64,
}

impl ATRStopLoss {
    pub fn new(klines: &[KlineSummary], length: usize, multiplier: f64) -> Self {
        let atr = Self::calculate_atr(klines, length);
        let stop_loss = Self::calculate_stop_loss(klines, atr, multiplier);

        Self {
            stop_loss,
        }
    }

    fn calculate_true_range(current: &KlineSummary, previous_close: f64) -> f64 {
        let high = current.high.parse::<f64>().unwrap_or(0.0);
        let low = current.low.parse::<f64>().unwrap_or(0.0);
        (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs())
    }

    fn calculate_atr(klines: &[KlineSummary], length: usize) -> f64 {
        if klines.len() < length {
            return 0.0;
        }

        let mut true_ranges = Vec::new();

        // Calculate True Ranges
        for i in 1..klines.len() {
            let current = &klines[i];
            let previous_close = klines[i - 1].close.parse::<f64>().unwrap_or(0.0);
            let tr = Self::calculate_true_range(current, previous_close);
            true_ranges.push(tr);
        }

        // Calculate RMA
        Self::calculate_rma(&true_ranges, length)
    }

    fn calculate_rma(values: &[f64], length: usize) -> f64 {
        if values.is_empty() || length == 0 {
            return 0.0;
        }

        // Start with the Simple Moving Average for the first 'length' periods
        let mut rma = values.iter().take(length).sum::<f64>() / (length as f64);
        let alpha = 1.0 / (length as f64);

        // Apply RMA formula: RMA = (Previous RMA * (length - 1) + Current TR) / length
        for value in values.iter().skip(length) {
            rma = (value * alpha) + (rma * (1.0 - alpha));
        }

        rma
    }

    fn calculate_stop_loss(klines: &[KlineSummary], atr: f64, multiplier: f64) -> f64 {
        if let Some(last_kline) = klines.last() {
            let close = last_kline.close.parse::<f64>().unwrap_or(0.0);
            return close - (atr * multiplier); // Stop Loss based on ATR multiplier
        }
        0.0
    }
}

