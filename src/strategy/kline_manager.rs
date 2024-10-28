use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel};

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
}
