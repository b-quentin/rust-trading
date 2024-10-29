use binance::model::KlineSummary;
use crate::strategy::interface::Observer;
use std::any::Any;

#[derive(Debug)]
pub struct ATRStopLoss {
    pub stop_losses: Vec<f64>, // Vecteur pour stocker plusieurs valeurs de Stop Loss
    length: usize,             // Longueur pour le calcul de l'ATR
    multiplier: f64,           // Multiplicateur pour le calcul du Stop Loss
}

impl ATRStopLoss {
    pub fn new(klines: &[KlineSummary], length: usize, multiplier: f64) -> Self {
        let mut stop_losses = Vec::new();
        
        // Calculer les valeurs initiales pour chaque période possible
        for i in length..=klines.len() {
            let atr = Self::calculate_atr(&klines[0..i], length);
            let stop_loss = Self::calculate_stop_loss(&klines[0..i], atr, multiplier);
            stop_losses.push(stop_loss);
        }

        Self { stop_losses, length, multiplier }
    }

    // Ajouter une nouvelle valeur basée sur les données actuelles de klines
    pub fn add(&mut self, klines: &[KlineSummary]) {
        if klines.len() < self.length {
            return; // Ne pas calculer si les données sont insuffisantes
        }

        let atr = Self::calculate_atr(klines, self.length);
        let stop_loss = Self::calculate_stop_loss(klines, atr, self.multiplier);
        self.stop_losses.push(stop_loss); // Ajouter la nouvelle valeur de Stop Loss calculée au vecteur
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

    pub fn get_last_stop_loss(&self) -> Option<f64> {
        self.stop_losses.last().cloned()
    }
}

impl Observer for ATRStopLoss {
    fn on_new_kline(&mut self, _kline: &KlineSummary, all_klines: &[KlineSummary]) {
        self.add(all_klines); // Met à jour avec toutes les klines disponibles
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


