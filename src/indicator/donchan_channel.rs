use std::any::Any;

use binance::model::KlineSummary;

use crate::strategy::interface::Observer;

#[derive(Debug)]
pub struct DonchianChannel {
    pub upper_band: Vec<f64>,  // Vecteur des bandes supérieures
    pub lower_band: Vec<f64>,  // Vecteur des bandes inférieures
    pub basis: Vec<f64>,       // Vecteur des lignes de base
    length: usize,             // Longueur de la période pour le calcul
    offset: usize,             // Décalage pour le calcul
}

impl DonchianChannel {
    pub fn new(klines: &[KlineSummary], length: usize, offset: usize) -> Self {
        let mut upper_band = Vec::new();
        let mut lower_band = Vec::new();
        let mut basis = Vec::new();

        // Calculer les valeurs pour chaque période en tenant compte du décalage
        for i in offset..=klines.len() {
            if i < length + offset {
                continue; // Ignorer si on n'a pas encore assez de données
            }

            let current_upper = Self::get_highest_for_dc(&klines[0..i], length, offset);
            let current_lower = Self::get_lowest_for_dc(&klines[0..i], length, offset);
            let current_basis = Self::calculate_basis(current_upper, current_lower);

            upper_band.push(current_upper);
            lower_band.push(current_lower);
            basis.push(current_basis);
        }

        Self {
            upper_band,
            lower_band,
            basis,
            length,
            offset,
        }
    }

    pub fn add(&mut self, all_klines: &[KlineSummary]) {
        // Recalculer avec toutes les klines disponibles
        if all_klines.len() >= self.length + self.offset {
            let current_upper = Self::get_highest_for_dc(all_klines, self.length, self.offset);
            let current_lower = Self::get_lowest_for_dc(all_klines, self.length, self.offset);
            let current_basis = Self::calculate_basis(current_upper, current_lower);

            self.upper_band.push(current_upper);
            self.lower_band.push(current_lower);
            self.basis.push(current_basis);
        }
    }

    fn get_lowest_for_dc(klines: &[KlineSummary], length: usize, offset: usize) -> f64 {
        if klines.len() < length + offset {
            return f64::MAX; // Retourner une valeur par défaut si les données sont insuffisantes
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
            return f64::MIN; // Retourner une valeur par défaut si les données sont insuffisantes
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

    /// Récupère la dernière valeur de la bande supérieure (upper_band)
    pub fn get_last_upper_band(&self) -> Option<f64> {
        self.upper_band.last().cloned()
    }

    /// Récupère la dernière valeur de la bande inférieure (lower_band)
    pub fn get_last_lower_band(&self) -> Option<f64> {
        self.lower_band.last().cloned()
    }

    /// Récupère la dernière valeur de la ligne de base (basis)
    pub fn get_last_basis(&self) -> Option<f64> {
        self.basis.last().cloned()
    }

    pub fn get_prev_upper_band(&self) -> Option<f64> {
        if self.upper_band.len() > 1 {
            self.upper_band.get(self.upper_band.len() - 2).cloned()
        } else {
            None
        }
    }
}

impl Observer for DonchianChannel {
    fn on_new_kline(&mut self, _kline: &binance::model::KlineSummary, all_klines: &[binance::model::KlineSummary]) {
        self.add(all_klines); // Recalculer et mettre à jour avec toutes les klines
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

