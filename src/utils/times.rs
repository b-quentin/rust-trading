use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use thiserror::Error;
use log::{error, info};

#[derive(Debug)]
pub enum Month {
	January,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Time {
    timestamp: u64, // en millisecondes
}

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("Erreur de parsing de date: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("Timestamp invalide : {0}")]
    InvalidTimestamp(i64),
}

impl Time {
    // Crée un nouvel objet Time à partir d'un timestamp UNIX en millisecondes
    pub fn from_unix(timestamp: u64) -> Self {
        info!("Création de l'objet Time à partir du timestamp (ms) : {}", timestamp);
        Self { timestamp }
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    // Crée un nouvel objet Time à partir d'une date en string (ex: "2024-10-29 15:30:00" ou en RFC 3339)
    pub fn from_str(date_str: &str) -> Result<Self, TimeError> {
        info!("Parsing de la date à partir de la chaîne : {}", date_str);

        // Tente de parser la date en RFC 3339
        if let Ok(datetime) = date_str.parse::<DateTime<Utc>>() {
            let timestamp = datetime.timestamp_millis() as u64;
            info!("Timestamp généré avec succès (ms) : {}", timestamp);
            return Ok(Self { timestamp });
        }

        // Si cela échoue, tente le format personnalisé
        let naive_datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
        let timestamp = naive_datetime.and_utc().timestamp_millis() as u64;

        info!("Timestamp généré avec succès (ms) : {}", timestamp);
        Ok(Self { timestamp })
    }

    // Convertit le timestamp UNIX en millisecondes en une date formatée
    pub fn to_string(&self) -> String {
        let timestamp_in_seconds = self.timestamp / 1000; // Convertir en secondes pour Chrono

        match Utc.timestamp_opt(timestamp_in_seconds as i64, 0).single() {
            Some(datetime) => {
                let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                info!("Conversion réussie du timestamp {} en date : {}", self.timestamp, formatted_date);
                formatted_date
            },
            None => {
                error!("Échec de la conversion pour le timestamp : {}", self.timestamp);
                "Invalid timestamp".to_string()
            }
        }
    }

    pub fn get_month(&self) -> Month {
        let datetime = DateTime::from_timestamp(self.timestamp as i64, 0).unwrap_or(DateTime::default());

        let month = datetime.month0() + 1;
        match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Unknown month"),
        }
    }
}


