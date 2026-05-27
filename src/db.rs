use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentMethod {
    #[serde(rename = "Mock Auto")]
    MockAuto,
    
    #[serde(rename = "Mock Manual")]
    MockManual,
    
    #[serde(rename = "Credit Card")]
    CreditCard,
    
    #[serde(rename = "Crypto")]
    Crypto,
    
    #[serde(rename = "PayPal")]
    PayPal,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MockAuto => write!(f, "Mock Auto"),
            Self::MockManual => write!(f, "Mock Manual"),
            Self::CreditCard => write!(f, "Credit Card"),
            Self::Crypto => write!(f, "Crypto"),
            Self::PayPal => write!(f, "PayPal"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Initialize,
    ReadyForDisplay,
    Displayed,
    Rejected,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initialize => write!(f, "INITIALIZE"),
            Self::ReadyForDisplay => write!(f, "READY_FOR_DISPLAY"),
            Self::Displayed => write!(f, "DISPLAYED"),
            Self::Rejected => write!(f, "REJECTED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DbStreamer {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub bio: String,
    pub is_live: bool,
    pub user_id: Option<String>,
    pub overlay_token: String,
    pub active_overlay_session: Option<String>,
    pub payment_methods: Vec<PaymentMethod>,
    pub overlay_paused: bool,
    pub overlay_sound_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DbTransaction {
    pub id: i32,
    pub streamer_id: i32,
    pub donor_name: String,
    pub amount: f64,
    pub message: Option<String>,
    pub payment_method: PaymentMethod,
    pub status: TransactionStatus,
    pub created_at: String,
}

#[cfg(feature = "ssr")]
pub mod db_ops {
    use sqlx::PgPool;

    /// Seed sample data (NeonViper streamer) if the table is empty.
    /// Table creation is handled by SQLx migrations.
    pub async fn seed_data(pool: &PgPool) -> Result<(), sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM streamers WHERE username = 'neonviper')"
        )
        .fetch_one(pool)
        .await?;

        if !exists {
            sqlx::query(
                "INSERT INTO streamers (username, display_name, avatar_url, bio, is_live, user_id, overlay_token)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind("neonviper")
            .bind("NeonViper")
            .bind("https://lh3.googleusercontent.com/aida-public/AB6AXuBLjEWKtvutw2bXJ6cXjX35VKSvndfcZFjksgkktDcFmWKH5w3JqiRsBENEnrWm0JHREPHPBQRwTGM2krlAjj-4IyFB_LtaFrMOvwlpVF-S4Wn-Qpc0Of9KKyyIayT9k7z69aL3NoVoXBzHPX-ZTbmlTm1ZFFq2kN49w8irdbwsj0edERW-AXu_cuLLa2XaiDOHQM4f5mbEU5MqTwigjzU5okvpS1kdr5WuV-yhcWwXphzBaqQ11rEVUtD0TpCxcHePnYEOrnYJOdc")
            .bind("Pushing the boundaries of competitive play. Today we're smashing the charity goals for the Digital Oceans Fund!")
            .bind(true)
            .bind(Some("seed_user_neonviper"))
            .bind("seed_overlay_token_neonviper")
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}


#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::Postgres> for TransactionStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for TransactionStatus {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "INITIALIZE" => Ok(TransactionStatus::Initialize),
            "READY_FOR_DISPLAY" => Ok(TransactionStatus::ReadyForDisplay),
            "DISPLAYED" => Ok(TransactionStatus::Displayed),
            "REJECTED" => Ok(TransactionStatus::Rejected),
            _ => Ok(TransactionStatus::Unknown),
        }
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::Postgres> for PaymentMethod {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PaymentMethod {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "Mock Auto" => Ok(PaymentMethod::MockAuto),
            "Mock Manual" => Ok(PaymentMethod::MockManual),
            "Credit Card" => Ok(PaymentMethod::CreditCard),
            "Crypto" => Ok(PaymentMethod::Crypto),
            "PayPal" => Ok(PaymentMethod::PayPal),
            _ => Ok(PaymentMethod::Unknown),
        }
    }
}

#[cfg(feature = "ssr")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for TransactionStatus {
    fn encode_by_ref(&self, buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let s = match self {
            Self::Initialize => "INITIALIZE",
            Self::ReadyForDisplay => "READY_FOR_DISPLAY",
            Self::Displayed => "DISPLAYED",
            Self::Rejected => "REJECTED",
            Self::Unknown => "UNKNOWN",
        };
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s.to_string(), buf)
    }
}

#[cfg(feature = "ssr")]
impl sqlx::postgres::PgHasArrayType for TransactionStatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}

#[cfg(feature = "ssr")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for PaymentMethod {
    fn encode_by_ref(&self, buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let s = match self {
            Self::MockAuto => "Mock Auto",
            Self::MockManual => "Mock Manual",
            Self::CreditCard => "Credit Card",
            Self::Crypto => "Crypto",
            Self::PayPal => "PayPal",
            Self::Unknown => "Unknown",
        };
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s.to_string(), buf)
    }
}

#[cfg(feature = "ssr")]
impl sqlx::postgres::PgHasArrayType for PaymentMethod {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
