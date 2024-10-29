use crate::indicator::{ATRStopLoss, ChoppinessIndex, DonchianChannel};
use binance::{api::Binance, market::Market, model::KlineSummaries};
use chrono::DateTime;
use super::{KlineManager, TradingStrategy};

pub enum Mode {
    Backtest,
    Live,
}

pub struct ChoppinessDonchianAtrStrategy {
    mode: Mode,
    symbol: String,
    capital: f64,
    quantity: f64,
    on_trade: bool,
    take_profit: f64,
    stop_loss: f64,
}

impl ChoppinessDonchianAtrStrategy {
    pub fn new(mode: Mode, symbol: &str) -> Self {
        Self {
            mode,
            symbol: symbol.to_string(),
            capital: 0.01,
            quantity: 0.0,
            on_trade: false,
            take_profit: 0.0,
            stop_loss: 0.0,
        }
    }

    fn place_order_buy(&mut self, price: f64, stop_loss: f64) {
        self.on_trade = true;
        self.quantity = self.capital / price;

        match self.mode {
            Mode::Backtest => {
                // Simuler un achat pour le backtesting
                self.stop_loss = stop_loss;
                let risk_amount = price - stop_loss;
                println!("risk_amount: {}", risk_amount);
                self.take_profit = price + ( risk_amount * 3.0 );

                self.capital -= self.quantity * price;
                println!("Mocking order: Buy {} of price {}, placing stop loss at {} and take profit at {}", self.symbol, price, self.stop_loss, self.take_profit);
            }
            Mode::Live => {
                // Appel réel à l'API Binance pour exécuter un achat
                println!("Placing real order on Binance: Buy {} at price {}", self.symbol, price);
                // Ici, vous devriez intégrer la logique d'achat réelle via l'API Binance
                // let market: Market = Binance::new(Some(api_key), Some(secret_key));
                // market.buy_limit(symbol, quantity, price).unwrap();
            }
        }
    }

    fn place_order_sell(&mut self, price: f64) {
        self.on_trade = false;

        match self.mode {
            Mode::Backtest => {
                // Simuler un achat pour le backtesting
                println!("Mocking order: Sell {} of price {}", self.symbol, price);
                if price >= self.take_profit {
                    println!("Stop loss triggered: Sell {} of price {}", self.symbol, price);
                    self.capital += self.quantity * price;
                }
                if price <= self.stop_loss {
                    println!("Take profit triggered: Sell {} of price {}", self.symbol, price);
                    self.capital += self.quantity * price;
                }

            // Calculer le taux d'évolution du capital en pourcentage
            let evolution_percentage = ((self.capital - 0.01) / 0.01) * 100.0;
            println!("Capital after sell: {}", self.capital);
            println!("Capital evolution: {:.2}%", evolution_percentage);
            }
            Mode::Live => {
                // Appel réel à l'API Binance pour exécuter un achat
                println!("Placing real order on Binance: Buy {} at price {}", self.symbol, price);
                // Ici, vous devriez intégrer la logique d'achat réelle via l'API Binance
                // let market: Market = Binance::new(Some(api_key), Some(secret_key));
                // market.buy_limit(symbol, quantity, price).unwrap();
            }
        }
    }
}

impl TradingStrategy for ChoppinessDonchianAtrStrategy {
    fn prepare(&self, kline1d: binance::model::KlineSummary, kline1h: binance::model::KlineSummary) -> (KlineManager, KlineManager) {
        println!("Preparing ChoppinessDonchianAtrStrategy...");
        let market: Market = Binance::new(None, None);

        // Définir les intervalles pour les données horaires et journalières
        let interval_1h = "1h";
        let interval_1d = "1d";
        let end_time_1h = Some(kline1h.open_time as u64);
        let end_time_1d = Some(kline1d.open_time as u64);

        // Récupérer les 100 dernières klines horaires
        let initial_klines_1h: Vec<binance::model::KlineSummary> = match market.get_klines(self.symbol.clone(), interval_1h, Some(100), None, end_time_1h) {
            Ok(KlineSummaries::AllKlineSummaries(klines)) => {
                println!("Fetched 100 previous 1h klines successfully.");
                klines
            },
            Err(e) => {
                println!("Failed to fetch 100 previous 1h klines: {:?}", e);
                Vec::new()
            }
        };

        // Récupérer les 100 dernières klines journalières
        let initial_klines_1d: Vec<binance::model::KlineSummary> = match market.get_klines(self.symbol.clone(), interval_1d, Some(100), None, end_time_1d) {
            Ok(KlineSummaries::AllKlineSummaries(klines)) => {
                println!("Fetched 100 previous 1d klines successfully.");
                klines
            },
            Err(e) => {
                println!("Failed to fetch 100 previous 1d klines: {:?}", e);
                Vec::new()
            }
        };

        // Initialiser les indicateurs pour les données horaires
        let donchian_channel_1h = Box::new(DonchianChannel::new(&initial_klines_1h, 20, 20));
        let choppiness_index_1h = Box::new(ChoppinessIndex::new(&initial_klines_1h, 100));
        let atr_stop_loss_1h = Box::new(ATRStopLoss::new(&initial_klines_1h, 14, 1.5));

        // Créer le KlineManager pour les données horaires
        let kline_manager_1h = KlineManager::new(initial_klines_1h, vec![donchian_channel_1h, choppiness_index_1h, atr_stop_loss_1h]);

        // Initialiser les indicateurs pour les données journalières (exemple, vous pouvez ajuster selon vos besoins)
        let donchian_channel_1d = Box::new(DonchianChannel::new(&initial_klines_1d, 20, 20));
        let choppiness_index_1d = Box::new(ChoppinessIndex::new(&initial_klines_1d, 100));
        let atr_stop_loss_1d = Box::new(ATRStopLoss::new(&initial_klines_1d, 14, 1.5));

        // Créer le KlineManager pour les données journalières
        let kline_manager_1d = KlineManager::new(initial_klines_1d, vec![donchian_channel_1d, choppiness_index_1d, atr_stop_loss_1d]);

        // Retourner les deux KlineManager
        (kline_manager_1d, kline_manager_1h)
    }


    fn execute(&mut self, kline: binance::model::KlineSummary, manager: &mut KlineManager) {
        //println!("Running ChoppinessDonchianAtrStrategy...");
        manager.add_kline(kline.clone());

        let last_kline = manager.klines[manager.klines.len() - 1].clone();
        let prev_kline = manager.klines[manager.klines.len() - 2].clone();
        let prev_close = prev_kline.close.parse::<f64>().unwrap_or(0.0);        
        let close = last_kline.close.parse::<f64>().unwrap_or(0.0);
        let obj_donchian_channel = manager.get_donchian_channel().unwrap();
        let donchian_channel = obj_donchian_channel.upper_band[obj_donchian_channel.upper_band.len() - 1].clone();
        let prev_donchian_channel = obj_donchian_channel.upper_band[obj_donchian_channel.upper_band.len() - 2].clone();
        let obj_choppiness_index = manager.get_choppiness_index().unwrap();
        let choppiness_index = obj_choppiness_index.values[obj_choppiness_index.values.len() - 1].clone();
        let obj_atr_stop_loss = manager.get_atr_stop_loss().unwrap();
        let atr_stop_loss = obj_atr_stop_loss.stop_losses[obj_atr_stop_loss.stop_losses.len() - 1].clone();

        //println!("closed time: {}", convert_timestamp_to_datetime(kline.close_time));
        //println!("Prev Close: {}", prev_close);
        //println!("Close: {}", close);
        //println!("Donchian Channel: {}", donchian_channel);
        //println!("Prev Donchian Channel: {}", prev_donchian_channel);
        //println!("Choppiness Index: {}", choppiness_index);

        if self.on_trade == false 
            && prev_close < prev_donchian_channel
            && close > donchian_channel 
            && choppiness_index <= 50.0 {
            println!("Placing buy order...");
            self.place_order_buy(close, atr_stop_loss);
            println!("last kline: {:?}", kline);
            println!("closed time: {}", convert_timestamp_to_datetime(kline.close_time));
            // Récupérer le DonchianChannel et afficher `upper_band`
            if let Some(donchian_channel) = manager.get_donchian_channel() {
                println!("Upper Band: {:?}", donchian_channel.upper_band[donchian_channel.upper_band.len() - 1]);
            } else {
                println!("DonchianChannel non trouvé parmi les observateurs.");
            }
            if let Some(choppiness_index) = manager.get_choppiness_index() {
                println!("Choppiness Index Values: {:?}", choppiness_index.values[choppiness_index.values.len() - 1]);
            }
            if let Some(atr_stop_loss) = manager.get_atr_stop_loss() {
                println!("ATR Stop Loss: {:?}", atr_stop_loss.stop_losses[atr_stop_loss.stop_losses.len() - 1]);
            }
        }

        if self.on_trade == true && (close > self.take_profit || close < self.stop_loss) {
            self.place_order_sell(close);
        }
        
        //println!("last kline: {:?}", kline);
        //println!("closed time: {}", convert_timestamp_to_datetime(kline.close_time));
        //// Récupérer le DonchianChannel et afficher `upper_band`
        //if let Some(donchian_channel) = manager.get_donchian_channel() {
        //    println!("Upper Band: {:?}", donchian_channel.upper_band[donchian_channel.upper_band.len() - 1]);
        //} else {
        //    println!("DonchianChannel non trouvé parmi les observateurs.");
        //}
        //if let Some(choppiness_index) = manager.get_choppiness_index() {
        //    println!("Choppiness Index Values: {:?}", choppiness_index.values[choppiness_index.values.len() - 1]);
        //}
        //if let Some(atr_stop_loss) = manager.get_atr_stop_loss() {
        //    println!("ATR Stop Loss: {:?}", atr_stop_loss.stop_losses[atr_stop_loss.stop_losses.len() - 1]);
        //}
    }
}


fn convert_timestamp_to_datetime(timestamp: i64) -> String {
    // Crée un NaiveDateTime à partir du timestamp (secondes depuis l'époque UNIX)
    let naive = DateTime::from_timestamp_millis(timestamp).unwrap();

    naive.format("%Y-%m-%d %H:%M:%S").to_string()
}
