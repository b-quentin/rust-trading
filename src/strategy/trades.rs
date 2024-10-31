use binance::model::KlineSummary;

use crate::utils::times::{Month, Time};

#[derive(Debug)]
pub enum WinOrLoose {
    Win,
    Loose,
    Undefined,
}

#[derive(Debug)]
pub struct ResultTrade {
    amount: f64,
    percentage: f64,
    win_or_loose: WinOrLoose,
}

pub struct TradesManager {
    capital: f64,
    risk: f64,
    trades: Vec<TradeManager>,
}

impl TradesManager {
    pub fn new(capital: f64, risk: f64) -> Self {
        TradesManager { 
            risk,
            capital, 
            trades: Vec::new() 
        }
    }

    pub fn add_trade(&mut self, trade: TradeManager) {
        self.trades.push(trade);
    }

    pub fn get_last_trade(&self) -> &TradeManager {
        &self.trades[self.trades.len() - 1]
    }
}

#[derive(Debug)]
pub struct TradeManager {
    pub entry_price: f64,
    pub take_profit: f64,
    pub stop_loss: f64,
    pub quantity: f64,
    pub month: Month,
    pub timestamp: Time,
    pub result: ResultTrade,
}

impl TradeManager {
    pub fn buy(kline: KlineSummary, entry_price: f64, stop_loss: f64, risk: f64) -> Self {
        let risk_amount = entry_price - stop_loss;
        let take_profit = entry_price + ( risk_amount * 3.0 );
        let timestamp = &Time::from_unix(kline.close_time as u64);
        let month = timestamp.get_month();

        Self {
            entry_price,
            stop_loss,
            take_profit,
            timestamp: timestamp.clone(),
            quantity: risk / entry_price,
            result: ResultTrade { amount: 0.0, percentage: 0.0, win_or_loose: WinOrLoose::Undefined },
            month,
        }
    }
}
