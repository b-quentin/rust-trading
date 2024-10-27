use binance::model::KlineSummary;

#[derive(Debug)]
pub struct DonchianChannel {
    pub upper_band: f64,
    pub lower_band: f64,
    pub basis: f64,
}

impl DonchianChannel {
    pub fn new(klines: &[KlineSummary], length: usize, offset: usize) -> Self {
        let upper_band = Self::get_highest_for_dc(klines, length, offset);
        let lower_band = Self::get_lowest_for_dc(klines, length, offset);
        let basis = Self::calculate_basis(upper_band, lower_band);

        Self {
            upper_band,
            lower_band,
            basis,
        }
    }

    fn get_lowest_for_dc(klines: &[KlineSummary], length: usize, offset: usize) -> f64 {
        if klines.len() < length + offset {
            return f64::MAX; // Si les données ne sont pas suffisantes, retourner une valeur par défaut
        }

        let start = klines.len() - length - offset;
        let end = klines.len() - offset;

        klines[start..end]
            .iter()
            .map(|kline| kline.low.parse::<f64>().unwrap_or(f64::MAX))
            .fold(f64::MAX, |a, b| a.min(b))
    }

    fn get_highest_for_dc(klines: &[KlineSummary], length: usize, offset: usize) -> f64 {
        if klines.len() < length + offset {
            return f64::MIN; // Si les données ne sont pas suffisantes, retourner une valeur par défaut
        }

        let start = klines.len() - length - offset;
        let end = klines.len() - offset;

        klines[start..end]
            .iter()
            .map(|kline| kline.high.parse::<f64>().unwrap_or(f64::MIN))
            .fold(f64::MIN, |a, b| a.max(b))
    }

    fn calculate_basis(upper: f64, lower: f64) -> f64 {
        (upper + lower) / 2.0
    }
}
