use binance::model::KlineSummary;
use crate::strategy::interface::Observer;
use std::any::Any;

#[derive(Debug)]
pub struct ChoppinessIndex {
    pub id: String, // Ajout d'un champ pour l'identifiant
    pub values: Vec<f64>,  // Vecteur pour stocker plusieurs valeurs de Choppiness Index
    length: usize,         // Longueur pour le calcul
}

impl ChoppinessIndex {
    pub fn new(id: String, klines: &[KlineSummary], length: usize) -> Self {
        let mut values = Vec::new();
        
        // Ajouter des zéros pour les premières valeurs jusqu'à ce que nous ayons suffisamment de données
        for _ in 0..(length - 1).min(klines.len()) {
            values.push(0.0);
        }

        // Commencer le calcul seulement après avoir assez de données
        for i in (length - 1)..klines.len() {
            let atr_sum = Self::calculate_atr_sum(&klines[(i + 1 - length)..=i], length);
            let highest = Self::get_highest(&klines[(i + 1 - length)..=i], length);
            let lowest = Self::get_lowest(&klines[(i + 1 - length)..=i], length);
            let value = Self::calculate_choppiness_index(atr_sum, highest, lowest, length);
            values.push(value);
        }

        Self { 
            id, 
            values, 
            length 
        }
    }

    // Ajouter une nouvelle valeur basée sur les données actuelles de klines
    pub fn add(&mut self, klines: &[KlineSummary]) {
        if klines.len() < self.length {
            self.values.push(0.0); // Ajouter 0 si les données sont insuffisantes
            return;
        }

        let atr_sum = Self::calculate_atr_sum(&klines[(klines.len() - self.length)..], self.length);
        let highest = Self::get_highest(&klines[(klines.len() - self.length)..], self.length);
        let lowest = Self::get_lowest(&klines[(klines.len() - self.length)..], self.length);
        let value = Self::calculate_choppiness_index(atr_sum, highest, lowest, self.length);

        self.values.push(value); // Ajouter la nouvelle valeur calculée au vecteur
    }

    fn calculate_true_range(current: &KlineSummary, previous_close: f64) -> f64 {
        let high = current.high.parse::<f64>().unwrap_or(0.0);
        let low = current.low.parse::<f64>().unwrap_or(0.0);
        (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs())
    }

    fn calculate_atr_sum(klines: &[KlineSummary], _length: usize) -> f64 {
        let mut true_ranges = Vec::new();
        for i in 1..klines.len() {
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
            return 0.0; // Éviter la division par zéro
        }

        let ratio = atr_sum / (highest - lowest);
        let log10_ratio = ratio.log10();
        let log10_length = (length as f64).log10();

        100.0 * log10_ratio / log10_length
    }
    pub fn get_last_value(&self) -> Option<f64> {
        self.values.last().cloned()
    }
}

impl Observer for ChoppinessIndex {
    fn on_new_kline(&mut self, _kline: &KlineSummary, all_klines: &[KlineSummary]) {
        self.add(all_klines); // Met à jour avec toutes les klines disponibles
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn id(&self) -> &str {
        &self.id
    }
}

