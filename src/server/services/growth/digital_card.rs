use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDigitalCardRequest {
    pub name: String,
    pub title: String,
    pub company: String,
    pub email: String,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DigitalCard {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub title: String,
    pub company: String,
    pub email: String,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub theme: Option<String>,
    pub vcard_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct DigitalCardService {
    pool: PgPool,
}

impl DigitalCardService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_card(&self, tenant_id: String, req: CreateDigitalCardRequest) -> Result<DigitalCard, sqlx::Error> {
        let card = sqlx::query_as::<_, DigitalCard>(
            r#"
            INSERT INTO growth_digital_cards (tenant_id, name, title, company, email, phone, bio, website, theme)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, name, title, company, email, phone, bio, website, theme, vcard_url, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(req.name)
        .bind(req.title)
        .bind(req.company)
        .bind(req.email)
        .bind(req.phone)
        .bind(req.bio)
        .bind(req.website)
        .bind(req.theme)
        .fetch_one(&self.pool)
        .await?;

        Ok(card)
    }

    pub async fn get_card(&self, id: Uuid) -> Result<DigitalCard, sqlx::Error> {
        let card = sqlx::query_as::<_, DigitalCard>(
            r#"
            SELECT id, tenant_id, name, title, company, email, phone, bio, website, theme, vcard_url, created_at, updated_at
            FROM growth_digital_cards
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(card)
    }
}
