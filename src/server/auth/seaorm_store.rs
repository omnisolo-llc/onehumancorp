use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
};

use crate::{User, user_repository::UserRepository};

pub mod entities {
    use sea_orm::entity::prelude::*;

    pub mod user {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "users")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub username: String,
            pub email: String,
            pub password_hash: String,
            pub roles: Json,
            pub active: bool,
            pub tenant_id: String,
            pub oidc_subject: Option<String>,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod application_setting {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "application_settings")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub key: String,
            pub value: String,
            pub updated_at: DateTimeUtc,
            pub updated_by: Option<String>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod email_challenge {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "email_verification_challenges")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub email: String,
            pub code_hash: String,
            pub source_hash: String,
            pub created_at: DateTimeUtc,
            pub expires_at: DateTimeUtc,
            pub resend_after: DateTimeUtc,
            pub attempts: i32,
            pub send_count: i32,
            pub verified_at: Option<DateTimeUtc>,
            pub invitation_id: Option<String>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod registration_ticket {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "registration_tickets")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub email: String,
            pub token_hash: String,
            pub issued_at: DateTimeUtc,
            pub expires_at: DateTimeUtc,
            pub consumed_at: Option<DateTimeUtc>,
            pub invitation_id: Option<String>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod invitation {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "registration_invitations")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub email: String,
            pub token_hash: String,
            pub created_at: DateTimeUtc,
            pub expires_at: DateTimeUtc,
            pub consumed_at: Option<DateTimeUtc>,
            pub created_by: String,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod oidc_provider {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "oidc_providers")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub key: String,
            pub display_name: String,
            pub provider_kind: String,
            pub issuer: String,
            pub client_id: String,
            pub scopes: Json,
            pub secret_ref: String,
            pub enabled: bool,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod external_identity {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "external_identities")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub user_id: String,
            pub provider_key: String,
            pub issuer: String,
            pub subject: String,
            pub email: String,
            pub created_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod revoked_token {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "auth_revoked_tokens")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub jti: String,
            pub tenant_id: String,
            pub expires_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistrationMode {
    Closed,
    Open,
    InviteOnly,
}

impl RegistrationMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::InviteOnly => "invite_only",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            "invite_only" => Ok(Self::InviteOnly),
            _ => Err("invalid registration mode".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewEmailChallenge {
    pub id: String,
    pub email: String,
    pub code_hash: String,
    pub source_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub resend_after: DateTime<Utc>,
    pub invitation_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NewRegistrationTicket {
    pub id: String,
    pub email: String,
    pub token_hash: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub invitation_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PublicOidcProvider {
    pub key: String,
    pub display_name: String,
    pub provider_kind: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct AdminOidcProvider {
    pub key: String,
    pub display_name: String,
    pub provider_kind: String,
    pub issuer: String,
    pub configured: bool,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct OidcProviderConfig {
    pub key: String,
    pub display_name: String,
    pub issuer: String,
    pub client_id: String,
    pub secret_ref: String,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct SeaOrmAuthRepository {
    connection: DatabaseConnection,
}

impl SeaOrmAuthRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn registration_mode(&self) -> Result<RegistrationMode, String> {
        let value = entities::application_setting::Entity::find_by_id("registration_mode")
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .map(|setting| setting.value)
            .unwrap_or_else(|| "closed".to_string());
        RegistrationMode::parse(&value)
    }

    pub async fn set_registration_mode(
        &self,
        mode: RegistrationMode,
        actor: &str,
        now: DateTime<Utc>,
    ) -> Result<(), String> {
        use entities::application_setting;
        if let Some(setting) = application_setting::Entity::find_by_id("registration_mode")
            .one(&self.connection)
            .await
            .map_err(db_error)?
        {
            let mut active: application_setting::ActiveModel = setting.into();
            active.value = Set(mode.as_str().to_string());
            active.updated_at = Set(now);
            active.updated_by = Set(Some(actor.to_string()));
            active.update(&self.connection).await.map_err(db_error)?;
        } else {
            application_setting::ActiveModel {
                key: Set("registration_mode".to_string()),
                value: Set(mode.as_str().to_string()),
                updated_at: Set(now),
                updated_by: Set(Some(actor.to_string())),
            }
            .insert(&self.connection)
            .await
            .map_err(db_error)?;
        }
        Ok(())
    }

    pub async fn create_email_challenge(&self, challenge: NewEmailChallenge) -> Result<(), String> {
        entities::email_challenge::ActiveModel {
            id: Set(challenge.id),
            email: Set(challenge.email),
            code_hash: Set(challenge.code_hash),
            source_hash: Set(challenge.source_hash),
            created_at: Set(challenge.created_at),
            expires_at: Set(challenge.expires_at),
            resend_after: Set(challenge.resend_after),
            attempts: Set(0),
            send_count: Set(1),
            verified_at: Set(None),
            invitation_id: Set(challenge.invitation_id),
        }
        .insert(&self.connection)
        .await
        .map_err(db_error)?;
        Ok(())
    }

    pub async fn email_challenge(
        &self,
        id: &str,
    ) -> Result<Option<entities::email_challenge::Model>, String> {
        entities::email_challenge::Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(db_error)
    }

    pub async fn latest_email_challenge(
        &self,
        email: &str,
    ) -> Result<Option<entities::email_challenge::Model>, String> {
        entities::email_challenge::Entity::find()
            .filter(entities::email_challenge::Column::Email.eq(email))
            .order_by_desc(entities::email_challenge::Column::CreatedAt)
            .one(&self.connection)
            .await
            .map_err(db_error)
    }

    pub async fn delete_email_challenge(&self, id: &str) -> Result<(), String> {
        entities::email_challenge::Entity::delete_by_id(id)
            .exec(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(())
    }

    pub async fn registration_ticket_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<entities::registration_ticket::Model>, String> {
        let tickets = entities::registration_ticket::Entity::find()
            .filter(entities::registration_ticket::Column::TokenHash.eq(token_hash))
            .limit(2)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok((tickets.len() == 1).then(|| tickets[0].clone()))
    }

    pub async fn record_challenge_failure(&self, id: &str, attempts: i32) -> Result<bool, String> {
        use entities::email_challenge;
        let result = email_challenge::Entity::update_many()
            .col_expr(
                email_challenge::Column::Attempts,
                sea_orm::sea_query::Expr::value(attempts + 1),
            )
            .filter(email_challenge::Column::Id.eq(id))
            .filter(email_challenge::Column::Attempts.eq(attempts))
            .filter(email_challenge::Column::VerifiedAt.is_null())
            .exec(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(result.rows_affected == 1)
    }

    pub async fn verify_challenge_and_issue_ticket(
        &self,
        challenge_id: &str,
        expected_code_hash: &str,
        ticket: NewRegistrationTicket,
    ) -> Result<(), String> {
        use entities::{email_challenge, registration_ticket};
        let transaction = self.connection.begin().await.map_err(db_error)?;
        let challenge = email_challenge::Entity::find_by_id(challenge_id)
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "verification challenge unavailable".to_string())?;
        if challenge.code_hash != expected_code_hash
            || challenge.verified_at.is_some()
            || challenge.expires_at <= ticket.issued_at
            || challenge.attempts >= 5
            || challenge.email != ticket.email
            || challenge.invitation_id != ticket.invitation_id
        {
            return Err("verification challenge unavailable".to_string());
        }

        registration_ticket::ActiveModel {
            id: Set(ticket.id),
            email: Set(ticket.email),
            token_hash: Set(ticket.token_hash),
            issued_at: Set(ticket.issued_at),
            expires_at: Set(ticket.expires_at),
            consumed_at: Set(None),
            invitation_id: Set(ticket.invitation_id),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;

        let mut active: email_challenge::ActiveModel = challenge.into();
        active.verified_at = Set(Some(ticket.issued_at));
        active.update(&transaction).await.map_err(db_error)?;
        transaction.commit().await.map_err(db_error)
    }

    pub async fn consume_ticket_and_create_user(
        &self,
        token_hash: &str,
        now: DateTime<Utc>,
        user: User,
    ) -> Result<(), String> {
        use entities::{invitation, registration_ticket, user as user_entity};
        let transaction = self.connection.begin().await.map_err(db_error)?;
        let tickets = registration_ticket::Entity::find()
            .filter(registration_ticket::Column::TokenHash.eq(token_hash))
            .limit(2)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        if tickets.len() != 1 {
            return Err("registration ticket unavailable".to_string());
        }
        let ticket = tickets.into_iter().next().expect("one ticket");
        if ticket.consumed_at.is_some() || ticket.expires_at <= now || ticket.email != user.email {
            return Err("registration ticket unavailable".to_string());
        }

        let invitation = if let Some(invitation_id) = ticket.invitation_id.as_deref() {
            let invitation = invitation::Entity::find_by_id(invitation_id)
                .one(&transaction)
                .await
                .map_err(db_error)?
                .ok_or_else(|| "invitation unavailable".to_string())?;
            if invitation.consumed_at.is_some()
                || invitation.expires_at <= now
                || invitation.email != user.email
            {
                return Err("invitation unavailable".to_string());
            }
            Some(invitation)
        } else {
            None
        };

        let tenant_id = user.organization_id.clone().unwrap_or_default();
        user_entity::ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            roles: Set(serde_json::json!(user.roles)),
            active: Set(user.active),
            tenant_id: Set(tenant_id),
            oidc_subject: Set(user.oidc_subject.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;

        let mut active: registration_ticket::ActiveModel = ticket.into();
        active.consumed_at = Set(Some(now));
        active.update(&transaction).await.map_err(db_error)?;
        if let Some(invitation) = invitation {
            let mut active: invitation::ActiveModel = invitation.into();
            active.consumed_at = Set(Some(now));
            active.update(&transaction).await.map_err(db_error)?;
        }
        transaction.commit().await.map_err(db_error)
    }

    pub async fn public_oidc_providers(&self) -> Result<Vec<PublicOidcProvider>, String> {
        let providers = entities::oidc_provider::Entity::find()
            .filter(entities::oidc_provider::Column::Enabled.eq(true))
            .order_by_asc(entities::oidc_provider::Column::DisplayName)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(providers
            .into_iter()
            .map(|provider| PublicOidcProvider {
                key: provider.key,
                display_name: provider.display_name,
                provider_kind: provider.provider_kind,
            })
            .collect())
    }

    pub async fn admin_oidc_providers(&self) -> Result<Vec<AdminOidcProvider>, String> {
        let providers = entities::oidc_provider::Entity::find()
            .order_by_asc(entities::oidc_provider::Column::DisplayName)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(providers
            .into_iter()
            .map(|provider| AdminOidcProvider {
                key: provider.key,
                display_name: provider.display_name,
                provider_kind: provider.provider_kind,
                issuer: provider.issuer,
                configured: !provider.client_id.trim().is_empty()
                    && !provider.secret_ref.trim().is_empty(),
                enabled: provider.enabled,
            })
            .collect())
    }

    pub async fn oidc_provider(&self, key: &str) -> Result<Option<OidcProviderConfig>, String> {
        entities::oidc_provider::Entity::find_by_id(key)
            .one(&self.connection)
            .await
            .map_err(db_error)
            .map(|provider| {
                provider.map(|provider| OidcProviderConfig {
                    key: provider.key,
                    display_name: provider.display_name,
                    issuer: provider.issuer,
                    client_id: provider.client_id,
                    secret_ref: provider.secret_ref,
                    enabled: provider.enabled,
                })
            })
    }

    pub async fn set_oidc_provider_enabled(
        &self,
        key: &str,
        enabled: bool,
        now: DateTime<Utc>,
    ) -> Result<bool, String> {
        let Some(provider) = entities::oidc_provider::Entity::find_by_id(key)
            .one(&self.connection)
            .await
            .map_err(db_error)?
        else {
            return Ok(false);
        };
        let mut active: entities::oidc_provider::ActiveModel = provider.into();
        active.enabled = Set(enabled);
        active.updated_at = Set(now);
        active.update(&self.connection).await.map_err(db_error)?;
        Ok(true)
    }

    pub async fn sync_configured_oidc_providers_from_environment(&self) -> Result<(), String> {
        let mut configured = Vec::new();
        if let Ok(client_id) = std::env::var("OHC_OIDC_GOOGLE_CLIENT_ID") {
            if !client_id.trim().is_empty() {
                configured.push((
                    "google".to_string(),
                    "Google".to_string(),
                    "google".to_string(),
                    "https://accounts.google.com".to_string(),
                    client_id,
                    "OHC_OIDC_GOOGLE_CLIENT_SECRET".to_string(),
                ));
            }
        }
        if let (Ok(issuer), Ok(client_id)) = (
            std::env::var("OHC_OIDC_KEYCLOAK_ISSUER"),
            std::env::var("OHC_OIDC_KEYCLOAK_CLIENT_ID"),
        ) {
            if !issuer.trim().is_empty() && !client_id.trim().is_empty() {
                configured.push((
                    "keycloak".to_string(),
                    "Keycloak".to_string(),
                    "oidc".to_string(),
                    issuer,
                    client_id,
                    "OHC_OIDC_KEYCLOAK_CLIENT_SECRET".to_string(),
                ));
            }
        }
        for known_key in ["google", "keycloak"] {
            if !configured.iter().any(|provider| provider.0 == known_key) {
                if let Some(existing) = entities::oidc_provider::Entity::find_by_id(known_key)
                    .one(&self.connection)
                    .await
                    .map_err(db_error)?
                {
                    let mut active: entities::oidc_provider::ActiveModel = existing.into();
                    active.client_id = Set(String::new());
                    active.secret_ref = Set(String::new());
                    active.enabled = Set(false);
                    active.updated_at = Set(Utc::now());
                    active.update(&self.connection).await.map_err(db_error)?;
                }
            }
        }
        for (key, display_name, provider_kind, issuer, client_id, secret_ref) in configured {
            if let Some(existing) = entities::oidc_provider::Entity::find_by_id(&key)
                .one(&self.connection)
                .await
                .map_err(db_error)?
            {
                let mut active: entities::oidc_provider::ActiveModel = existing.into();
                active.display_name = Set(display_name);
                active.provider_kind = Set(provider_kind);
                active.issuer = Set(issuer.trim_end_matches('/').to_string());
                active.client_id = Set(client_id);
                active.secret_ref = Set(secret_ref);
                active.scopes = Set(serde_json::json!(["openid", "email", "profile"]));
                active.updated_at = Set(Utc::now());
                active.update(&self.connection).await.map_err(db_error)?;
            } else {
                let now = Utc::now();
                entities::oidc_provider::ActiveModel {
                    key: Set(key),
                    display_name: Set(display_name),
                    provider_kind: Set(provider_kind),
                    issuer: Set(issuer.trim_end_matches('/').to_string()),
                    client_id: Set(client_id),
                    scopes: Set(serde_json::json!(["openid", "email", "profile"])),
                    secret_ref: Set(secret_ref),
                    enabled: Set(false),
                    created_at: Set(now),
                    updated_at: Set(now),
                }
                .insert(&self.connection)
                .await
                .map_err(db_error)?;
            }
        }
        Ok(())
    }

    pub async fn user_for_external_identity(
        &self,
        provider_key: &str,
        issuer: &str,
        subject: &str,
    ) -> Result<Option<User>, String> {
        let identities = entities::external_identity::Entity::find()
            .filter(entities::external_identity::Column::ProviderKey.eq(provider_key))
            .filter(entities::external_identity::Column::Issuer.eq(issuer))
            .filter(entities::external_identity::Column::Subject.eq(subject))
            .limit(2)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        if identities.len() != 1 {
            return Ok(None);
        }
        let user = entities::user::Entity::find_by_id(&identities[0].user_id)
            .one(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(user.map(model_to_user))
    }

    pub async fn any_user_with_email(&self, email: &str) -> Result<bool, String> {
        Ok(entities::user::Entity::find()
            .filter(entities::user::Column::Email.eq(email))
            .limit(1)
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .is_some())
    }

    pub async fn active_invitation_id(
        &self,
        email: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<String>, String> {
        let invitations = entities::invitation::Entity::find()
            .filter(entities::invitation::Column::Email.eq(email))
            .filter(entities::invitation::Column::ConsumedAt.is_null())
            .filter(entities::invitation::Column::ExpiresAt.gt(now))
            .order_by_asc(entities::invitation::Column::CreatedAt)
            .limit(2)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok((invitations.len() == 1).then(|| invitations[0].id.clone()))
    }

    pub async fn active_invitation_id_by_token(
        &self,
        email: &str,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<String>, String> {
        let invitations = entities::invitation::Entity::find()
            .filter(entities::invitation::Column::Email.eq(email))
            .filter(entities::invitation::Column::TokenHash.eq(token_hash))
            .filter(entities::invitation::Column::ConsumedAt.is_null())
            .filter(entities::invitation::Column::ExpiresAt.gt(now))
            .limit(2)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok((invitations.len() == 1).then(|| invitations[0].id.clone()))
    }

    pub async fn create_oidc_user(
        &self,
        user: User,
        provider_key: &str,
        issuer: &str,
        subject: &str,
        invitation_id: Option<&str>,
    ) -> Result<(), String> {
        use entities::{external_identity, user as user_entity};
        let transaction = self.connection.begin().await.map_err(db_error)?;
        user_entity::ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            roles: Set(serde_json::json!(user.roles)),
            active: Set(true),
            tenant_id: Set(user.organization_id.clone().unwrap_or_default()),
            oidc_subject: Set(Some(format!("{issuer}|{subject}"))),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        external_identity::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            user_id: Set(user.id),
            provider_key: Set(provider_key.to_string()),
            issuer: Set(issuer.to_string()),
            subject: Set(subject.to_string()),
            email: Set(user.email.clone()),
            created_at: Set(Utc::now()),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        if let Some(invitation_id) = invitation_id {
            let invitation = entities::invitation::Entity::find_by_id(invitation_id)
                .one(&transaction)
                .await
                .map_err(db_error)?
                .ok_or_else(|| "invitation unavailable".to_string())?;
            if invitation.consumed_at.is_some()
                || invitation.expires_at <= Utc::now()
                || invitation.email != user.email
            {
                return Err("invitation unavailable".to_string());
            }
            let mut active: entities::invitation::ActiveModel = invitation.into();
            active.consumed_at = Set(Some(Utc::now()));
            active.update(&transaction).await.map_err(db_error)?;
        }
        transaction.commit().await.map_err(db_error)
    }
}

fn model_to_user(model: entities::user::Model) -> User {
    User {
        id: model.id,
        username: model.username,
        email: model.email,
        password_hash: model.password_hash,
        roles: serde_json::from_value(model.roles).unwrap_or_default(),
        active: model.active,
        organization_id: Some(model.tenant_id),
        created_at: model.created_at,
        updated_at: model.updated_at,
        oidc_subject: model.oidc_subject,
    }
}

fn db_error(error: sea_orm::DbErr) -> String {
    tracing::error!(event = "auth.persistence.unavailable", error_kind = ?error.sql_err());
    "authentication persistence unavailable".to_string()
}

#[async_trait]
impl UserRepository for SeaOrmAuthRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let mut user = user;
        user.organization_id = Some(org_id.to_string());
        entities::user::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            roles: Set(serde_json::json!(user.roles)),
            active: Set(user.active),
            tenant_id: Set(org_id.to_string()),
            oidc_subject: Set(user.oidc_subject),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&self.connection)
        .await
        .map_err(db_error)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let model = entities::user::Entity::find_by_id(id)
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        Ok(model_to_user(model))
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        Ok(model_to_user(model))
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::Email.eq(email))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        Ok(model_to_user(model))
    }

    async fn get_by_login_identifier(
        &self,
        identifier: &str,
        org_id: &str,
    ) -> Result<Option<User>, String> {
        let models = entities::user::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(entities::user::Column::Username.eq(identifier))
                    .add(entities::user::Column::Email.eq(identifier)),
            )
            .filter(entities::user::Column::TenantId.eq(org_id))
            .filter(entities::user::Column::Active.eq(true))
            .limit(2)
            .all(&self.connection)
            .await
            .map_err(db_error)?;
        Ok((models.len() == 1).then(|| model_to_user(models[0].clone())))
    }

    async fn get_by_oidc_subject(&self, subject: &str, org_id: &str) -> Result<User, String> {
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::OidcSubject.eq(subject))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        Ok(model_to_user(model))
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        entities::user::Entity::find()
            .filter(entities::user::Column::TenantId.eq(org_id))
            .order_by_asc(entities::user::Column::CreatedAt)
            .all(&self.connection)
            .await
            .map_err(db_error)
            .map(|models| models.into_iter().map(model_to_user).collect())
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let model = entities::user::Entity::find_by_id(&user.id)
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let mut active: entities::user::ActiveModel = model.into();
        active.username = Set(user.username);
        active.email = Set(user.email);
        active.password_hash = Set(user.password_hash);
        active.roles = Set(serde_json::json!(user.roles));
        active.active = Set(user.active);
        active.oidc_subject = Set(user.oidc_subject);
        active.updated_at = Set(user.updated_at);
        active.update(&self.connection).await.map_err(db_error)?;
        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        entities::user::Entity::delete_many()
            .filter(entities::user::Column::Id.eq(id))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .exec(&self.connection)
            .await
            .map_err(db_error)?;
        Ok(())
    }

    async fn revoke_token(
        &self,
        jti: String,
        exp: DateTime<Utc>,
        org_id: &str,
    ) -> Result<(), String> {
        if entities::revoked_token::Entity::find_by_id(&jti)
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .is_none()
        {
            entities::revoked_token::ActiveModel {
                jti: Set(jti),
                tenant_id: Set(org_id.to_string()),
                expires_at: Set(exp),
            }
            .insert(&self.connection)
            .await
            .map_err(db_error)?;
        }
        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        Ok(entities::revoked_token::Entity::find_by_id(jti)
            .filter(entities::revoked_token::Column::TenantId.eq(org_id))
            .filter(entities::revoked_token::Column::ExpiresAt.gt(Utc::now()))
            .one(&self.connection)
            .await
            .map_err(db_error)?
            .is_some())
    }
}
