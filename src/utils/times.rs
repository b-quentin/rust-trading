use chrono::{NaiveDateTime, Utc, TimeZone};

#[derive(Debug, Clone)]
pub struct Time {
    timestamp: u64,
}

impl Time {
    // Crée un nouvel objet Time à partir d'un timestamp UNIX en secondes
    pub fn from_unix(timestamp: u64) -> Self {
        Self { timestamp }
    }

    // Crée un nouvel objet Time à partir d'une date en string (ex: "2024-10-29 15:30:00")
    pub fn from_str(date_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let naive_datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
        let timestamp = naive_datetime.and_utc().timestamp() as u64;
        Ok(Self { timestamp })
    }

    // Convertit le timestamp UNIX en une date formatée
    pub fn to_string(&self) -> String {
        if let Some(datetime) = Utc.timestamp_opt(self.timestamp as i64, 0).single() {
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "Invalid timestamp".to_string()
        }
    }    

    // Retourne le timestamp UNIX
    pub fn as_unix(&self) -> u64 {
        self.timestamp
    }
}
