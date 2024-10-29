use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel, EMA};

use super::interface::Observer;

pub struct KlineManager {
    pub klines: Vec<binance::model::KlineSummary>,
    observers: Vec<Box<dyn Observer>>, // Liste d'observateurs dynamiques
}

impl KlineManager {
    pub fn new(initial_klines: Vec<binance::model::KlineSummary>, observers: Vec<Box<dyn Observer>>) -> Self {
        let mut manager = Self {
            klines: initial_klines.clone(),
            observers,
        };

        // Calculer les valeurs initiales pour les observateurs avec les klines fournies
        for observer in manager.observers.iter_mut() {
            observer.on_new_kline(&initial_klines[0], &manager.klines); // Passer les klines au complet
        }

        manager
    }

    pub fn add_kline(&mut self, kline: binance::model::KlineSummary) {
        self.klines.push(kline.clone());
        self.notify_observers(&kline);
    }

    fn notify_observers(&mut self, kline: &binance::model::KlineSummary) {
        for observer in self.observers.iter_mut() {
            observer.on_new_kline(kline, &self.klines); // Passer toutes les klines
        }
    }

    pub fn display_klines_one_by_one(&self) {
        for kline in &self.klines {
            println!("Kline: {:?}", kline);
            println!("Kline Open {}", kline.open_time);
            println!("Kline Close {}", kline.close_time);
        }
    }

    // Récupérer une référence à un `DonchianChannel` si c'est l'un des observateurs
    pub fn get_donchian_channel(&self) -> Option<&DonchianChannel> {
        for observer in &self.observers {
            if let Some(donchian) = observer.as_any().downcast_ref::<DonchianChannel>() {
                return Some(donchian);
            }
        }
        None
    }
    pub fn get_choppiness_index(&self) -> Option<&ChoppinessIndex> {
        for observer in &self.observers {
            if let Some(donchian) = observer.as_any().downcast_ref::<ChoppinessIndex>() {
                return Some(donchian);
            }
        }
        None
    }
    pub fn get_atr_stop_loss(&self) -> Option<&ATRStopLoss> {
        for observer in &self.observers {
            if let Some(donchian) = observer.as_any().downcast_ref::<ATRStopLoss>() {
                return Some(donchian);
            }
        }
        None
    }

    pub fn get_ema(&self) -> Option<&EMA> {
        for observer in &self.observers {
            if let Some(ema) = observer.as_any().downcast_ref::<EMA>() {
                return Some(ema)
            }
        }
        None
    }
    // Get the last kline
    pub fn get_last_kline(&self) -> Option<&binance::model::KlineSummary> {
        self.klines.last()
    }

    // Get the second to last kline
    pub fn get_prev_last_kline(&self) -> Option<&binance::model::KlineSummary> {
        if self.klines.len() > 1 {
            self.klines.get(self.klines.len() - 2)
        } else {
            None
        }
    }
    pub fn get_last_close(&self) -> f64 {
        self.get_last_kline()
            .and_then(|k| k.close.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    // Récupère la valeur close de l'avant-dernière kline en tant que f64
    pub fn get_prev_last_close(&self) -> f64 {
        self.get_prev_last_kline()
            .and_then(|k| k.close.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    pub fn get_last_donchian_upper_band(&self) -> f64 {
        self.get_donchian_channel()
            .and_then(|dc| dc.get_last_upper_band())
            .unwrap_or(0.0)
    }

    /// Récupère la dernière valeur de la bande inférieure du Donchian Channel ou une valeur par défaut
    pub fn get_last_donchian_lower_band(&self) -> f64 {
        self.get_donchian_channel()
            .and_then(|dc| dc.get_last_lower_band())
            .unwrap_or(0.0)
    }

    /// Récupère la dernière valeur de la ligne de base du Donchian Channel ou une valeur par défaut
    pub fn get_last_donchian_basis(&self) -> f64 {
        self.get_donchian_channel()
            .and_then(|dc| dc.get_last_basis())
            .unwrap_or(0.0)
    }
    pub fn get_prev_donchian_upper_band(&self) -> f64 {
        self.get_donchian_channel()
            .and_then(|dc| dc.get_prev_upper_band())
            .unwrap_or(0.0)
    }
    /// Récupère la dernière valeur de l'index de choppiness ou une valeur par défaut
    pub fn get_last_choppiness_index(&self) -> f64 {
        self.get_choppiness_index()
            .and_then(|ci| ci.get_last_value())
            .unwrap_or(0.0)
    }
    pub fn get_last_atr_stop_loss(&self) -> f64 {
        self.get_atr_stop_loss()
            .and_then(|atr| atr.get_last_stop_loss())
            .unwrap_or(0.0)
    }
    pub fn get_last_ema(&self) -> f64 {
        self.get_ema()
            .and_then(|ema| ema.get_last_value())
            .unwrap_or(0.0)
    }
}
