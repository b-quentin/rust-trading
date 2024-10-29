
use std::any::Any;

use binance::model::KlineSummary;
use crate::strategy::interface::Observer;

#[derive(Debug)]
pub struct EMA {
    pub values: Vec<f64>, // Stocke directement les valeurs de l'EMA sans `Option`
    period: usize,
    offset: usize,
}

impl EMA {
    pub fn new(klines: &[KlineSummary], period: usize, offset: usize) -> Self {
        let mut values = vec![0.0; offset]; // Initialise avec des zéros pour le décalage initial

        if klines.len() >= period + offset {
            // Calculer la première valeur d'EMA avec la moyenne simple des `period` premières valeurs
            let initial_sum: f64 = klines
                .iter()
                .take(period)
                .map(|kline| kline.close.parse::<f64>().unwrap_or(0.0))
                .sum();
            let initial_ema = initial_sum / period as f64;
            values.push(initial_ema);

            // Calcul de l'EMA pour chaque kline après la période initiale
            for kline in &klines[period + offset..] {
                let close_price = kline.close.parse::<f64>().unwrap_or(0.0);
                let last_ema = *values.last().unwrap();
                let new_ema = (close_price - last_ema) * (2.0 / (period as f64 + 1.0)) + last_ema;
                values.push(new_ema);
            }
        }

        Self {
            values,
            period,
            offset,
        }
    }

    // Recalculer l'EMA pour chaque nouvelle kline
    pub fn add(&mut self, all_klines: &[KlineSummary]) {
        if all_klines.len() >= self.period + self.offset {
            let close_price = all_klines.last().unwrap().close.parse::<f64>().unwrap_or(0.0);
            let last_ema = *self.values.last().unwrap();
            let new_ema = (close_price - last_ema) * (2.0 / (self.period as f64 + 1.0)) + last_ema;
            self.values.push(new_ema);
        }
    }
    pub fn get_last_value(&self) -> Option<f64> {
        self.values.last().cloned()
    }
}

// Exemple d'utilisation dans votre calcul de stratégie
impl Observer for EMA {
    fn on_new_kline(&mut self, _kline: &KlineSummary, all_klines: &[KlineSummary]) {
        self.add(all_klines); // Recalculer et mettre à jour avec toutes les klines disponibles
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

