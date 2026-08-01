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

    pub mod identity_email_claim {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "identity_email_claims")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub normalized_email: String,
            pub user_id: String,
            pub claimed_at: DateTimeUtc,
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

async fn registration_mode_in_transaction(
    transaction: &sea_orm::DatabaseTransaction,
) -> Result<RegistrationMode, String> {
    let value = entities::application_setting::Entity::find_by_id("registration_mode")
        .one(transaction)
        .await
        .map_err(db_error)?
        .map(|setting| setting.value)
        .unwrap_or_else(|| "closed".to_string());
    RegistrationMode::parse(&value)
}

async fn invitation_creator_tenant(
    transaction: &sea_orm::DatabaseTransaction,
    invitation: &entities::invitation::Model,
) -> Result<String, String> {
    let creator = entities::user::Entity::find_by_id(&invitation.created_by)
        .one(transaction)
        .await
        .map_err(db_error)?
        .filter(|creator| creator.active)
        .ok_or_else(|| "invitation unavailable".to_string())?;
    let creator_roles: Vec<String> = serde_json::from_value(creator.roles)
        .map_err(|_| "invitation unavailable".to_string())?;
    if !creator_roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case(crate::ROLE_ADMIN))
        || creator.tenant_id.trim().is_empty()
        || creator.tenant_id.eq_ignore_ascii_case("system")
    {
        return Err("invitation unavailable".to_string());
    }
    Ok(creator.tenant_id)
}

async fn claim_identity_email(
    transaction: &sea_orm::DatabaseTransaction,
    normalized_email: &str,
    user_id: &str,
    claimed_at: DateTime<Utc>,
    conflict_message: &str,
) -> Result<(), String> {
    let result = entities::identity_email_claim::ActiveModel {
        normalized_email: Set(normalized_email.to_string()),
        user_id: Set(user_id.to_string()),
        claimed_at: Set(claimed_at),
    }
    .insert(transaction)
    .await;
    match result {
        Ok(_) => Ok(()),
        Err(error)
            if matches!(
                error.sql_err(),
                Some(sea_orm::SqlErr::UniqueConstraintViolation(_))
            ) =>
        {
            Err(conflict_message.to_string())
        }
        Err(error) => Err(db_error(error)),
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
        let mut claim = email_challenge::Entity::update_many()
            .col_expr(
                email_challenge::Column::VerifiedAt,
                sea_orm::sea_query::Expr::value(Some(ticket.issued_at)),
            )
            .filter(email_challenge::Column::Id.eq(challenge_id))
            .filter(email_challenge::Column::CodeHash.eq(expected_code_hash))
            .filter(email_challenge::Column::Email.eq(&ticket.email))
            .filter(email_challenge::Column::VerifiedAt.is_null())
            .filter(email_challenge::Column::ExpiresAt.gt(ticket.issued_at))
            .filter(email_challenge::Column::Attempts.lt(5));
        claim = match ticket.invitation_id.as_deref() {
            Some(invitation_id) => claim.filter(
                email_challenge::Column::InvitationId.eq(invitation_id),
            ),
            None => claim.filter(email_challenge::Column::InvitationId.is_null()),
        };
        let claimed = claim.exec(&transaction).await.map_err(db_error)?;
        if claimed.rows_affected != 1 {
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
        transaction.commit().await.map_err(db_error)
    }

    pub async fn consume_ticket_and_create_user(
        &self,
        token_hash: &str,
        now: DateTime<Utc>,
        mut user: User,
    ) -> Result<User, String> {
        use entities::{invitation, registration_ticket, user as user_entity};
        let transaction = self.connection.begin().await.map_err(db_error)?;
        let normalized_email = crate::validation::normalize_email(&user.email)
            .map_err(|_| "account already exists".to_string())?;
        user.email = normalized_email.clone();
        let mode = registration_mode_in_transaction(&transaction).await?;
        if mode == RegistrationMode::Closed {
            return Err("registration closed".to_string());
        }
        let claimed = registration_ticket::Entity::update_many()
            .col_expr(
                registration_ticket::Column::ConsumedAt,
                sea_orm::sea_query::Expr::value(Some(now)),
            )
            .filter(registration_ticket::Column::TokenHash.eq(token_hash))
            .filter(registration_ticket::Column::Email.eq(&user.email))
            .filter(registration_ticket::Column::ConsumedAt.is_null())
            .filter(registration_ticket::Column::ExpiresAt.gt(now))
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        if claimed.rows_affected != 1 {
            return Err("registration ticket unavailable".to_string());
        }
        let ticket = registration_ticket::Entity::find()
            .filter(registration_ticket::Column::TokenHash.eq(token_hash))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "registration ticket unavailable".to_string())?;

        let invitation = if mode == RegistrationMode::InviteOnly {
            let invitation_id = ticket
                .invitation_id
                .as_deref()
                .ok_or_else(|| "invitation unavailable".to_string())?;
            let invitation = invitation::Entity::find_by_id(invitation_id)
                .one(&transaction)
                .await
                .map_err(db_error)?
                .ok_or_else(|| "invitation unavailable".to_string())?;
            let tenant_id = invitation_creator_tenant(&transaction, &invitation).await?;
            let claimed = invitation::Entity::update_many()
                .col_expr(
                    invitation::Column::ConsumedAt,
                    sea_orm::sea_query::Expr::value(Some(now)),
                )
                .filter(invitation::Column::Id.eq(invitation_id))
                .filter(invitation::Column::Email.eq(&user.email))
                .filter(invitation::Column::ConsumedAt.is_null())
                .filter(invitation::Column::ExpiresAt.gt(now))
                .exec(&transaction)
                .await
                .map_err(db_error)?;
            if claimed.rows_affected != 1 {
                return Err("invitation unavailable".to_string());
            }
            Some(tenant_id)
        } else {
            None
        };

        let tenant_id = match mode {
            RegistrationMode::Closed => return Err("registration closed".to_string()),
            RegistrationMode::Open => user.organization_id.clone().unwrap_or_default(),
            RegistrationMode::InviteOnly => invitation
                .ok_or_else(|| "invitation unavailable".to_string())?,
        };
        user.organization_id = Some(tenant_id.clone());
        claim_identity_email(
            &transaction,
            &normalized_email,
            &user.id,
            now,
            "account already exists",
        )
        .await?;
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

        transaction.commit().await.map_err(db_error)?;
        Ok(user)
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
        mut user: User,
        provider_key: &str,
        issuer: &str,
        subject: &str,
    ) -> Result<User, String> {
        use entities::{external_identity, invitation, user as user_entity};
        let transaction = self.connection.begin().await.map_err(db_error)?;
        let normalized_email = crate::validation::normalize_email(&user.email)
            .map_err(|_| "OIDC login denied".to_string())?;
        user.email = normalized_email.clone();
        let now = Utc::now();
        let mode = registration_mode_in_transaction(&transaction).await?;
        if mode == RegistrationMode::Closed {
            return Err("registration closed".to_string());
        }
        if user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(&normalized_email))
            .limit(1)
            .one(&transaction)
            .await
            .map_err(db_error)?
            .is_some()
        {
            return Err("existing account must explicitly link this provider".to_string());
        }
        let tenant_id = match mode {
            RegistrationMode::Closed => return Err("registration closed".to_string()),
            RegistrationMode::Open => user.organization_id.clone().unwrap_or_default(),
            RegistrationMode::InviteOnly => {
                let invitations = invitation::Entity::find()
                    .filter(invitation::Column::Email.eq(&normalized_email))
                    .filter(invitation::Column::ConsumedAt.is_null())
                    .filter(invitation::Column::ExpiresAt.gt(now))
                    .order_by_asc(invitation::Column::CreatedAt)
                    .limit(2)
                    .all(&transaction)
                    .await
                    .map_err(db_error)?;
                if invitations.len() != 1 {
                    return Err("registration closed".to_string());
                }
                let invitation = &invitations[0];
                let tenant_id = invitation_creator_tenant(&transaction, invitation).await?;
                let claimed = invitation::Entity::update_many()
                    .col_expr(
                        invitation::Column::ConsumedAt,
                        sea_orm::sea_query::Expr::value(Some(now)),
                    )
                    .filter(invitation::Column::Id.eq(&invitation.id))
                    .filter(invitation::Column::Email.eq(&normalized_email))
                    .filter(invitation::Column::ConsumedAt.is_null())
                    .filter(invitation::Column::ExpiresAt.gt(now))
                    .exec(&transaction)
                    .await
                    .map_err(db_error)?;
                if claimed.rows_affected != 1 {
                    return Err("invitation unavailable".to_string());
                }
                tenant_id
            }
        };
        user.organization_id = Some(tenant_id.clone());
        user.oidc_subject = Some(format!("{issuer}|{subject}"));
        claim_identity_email(
            &transaction,
            &normalized_email,
            &user.id,
            now,
            "existing account must explicitly link this provider",
        )
        .await?;
        user_entity::ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            roles: Set(serde_json::json!(user.roles)),
            active: Set(true),
            tenant_id: Set(tenant_id),
            oidc_subject: Set(user.oidc_subject.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        external_identity::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            user_id: Set(user.id.clone()),
            provider_key: Set(provider_key.to_string()),
            issuer: Set(issuer.to_string()),
            subject: Set(subject.to_string()),
            email: Set(user.email.clone()),
            created_at: Set(now),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        transaction.commit().await.map_err(db_error)?;
        Ok(user)
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
        let normalized_email = crate::validation::normalize_email(&user.email)
            .map_err(|_| "email already registered".to_string())?;
        user.email = normalized_email.clone();
        let transaction = self.connection.begin().await.map_err(db_error)?;
        claim_identity_email(
            &transaction,
            &normalized_email,
            &user.id,
            user.created_at,
            "email already registered",
        )
        .await?;
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
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        transaction.commit().await.map_err(db_error)
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
        let transaction = self.connection.begin().await.map_err(db_error)?;
        let model = entities::user::Entity::find_by_id(&user.id)
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let persisted_email = crate::validation::normalize_email(&model.email)
            .map_err(|_| "authentication persistence unavailable".to_string())?;
        let requested_email = crate::validation::normalize_email(&user.email)
            .map_err(|_| "email changes require verification".to_string())?;
        if persisted_email != requested_email {
            return Err("email changes require verification".to_string());
        }
        let mut active: entities::user::ActiveModel = model.into();
        active.username = Set(user.username);
        active.email = Set(requested_email);
        active.password_hash = Set(user.password_hash);
        active.roles = Set(serde_json::json!(user.roles));
        active.active = Set(user.active);
        active.oidc_subject = Set(user.oidc_subject);
        active.updated_at = Set(user.updated_at);
        active.update(&transaction).await.map_err(db_error)?;
        transaction.commit().await.map_err(db_error)
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

#[cfg(test)]
mod atomic_registration_tests {
    use super::*;
    use sea_orm::{
        ConnectOptions, ConnectionTrait, Database, EntityTrait, PaginatorTrait, Schema,
    };
    use std::time::Duration;

    async fn repositories() -> (
        tempfile::TempDir,
        SeaOrmAuthRepository,
        SeaOrmAuthRepository,
    ) {
        let directory = tempfile::tempdir().unwrap();
        let database_path = directory.path().join("atomic-auth.sqlite");
        let url = format!("sqlite://{}?mode=rwc", database_path.display());
        let connect = |url: String| async move {
            let mut options = ConnectOptions::new(url);
            options
                .max_connections(4)
                .connect_timeout(Duration::from_secs(5))
                .acquire_timeout(Duration::from_secs(5))
                .sqlx_logging(false);
            Database::connect(options).await.unwrap()
        };
        let first = connect(url.clone()).await;
        let schema = Schema::new(first.get_database_backend());
        for statement in [
            schema.create_table_from_entity(entities::user::Entity),
            schema.create_table_from_entity(entities::application_setting::Entity),
            schema.create_table_from_entity(entities::email_challenge::Entity),
            schema.create_table_from_entity(entities::registration_ticket::Entity),
            schema.create_table_from_entity(entities::invitation::Entity),
            schema.create_table_from_entity(entities::external_identity::Entity),
            schema.create_table_from_entity(entities::identity_email_claim::Entity),
        ] {
            first
                .execute(first.get_database_backend().build(&statement))
                .await
                .unwrap();
        }
        let second = connect(url).await;
        (
            directory,
            SeaOrmAuthRepository::new(first),
            SeaOrmAuthRepository::new(second),
        )
    }

    async fn set_mode(repository: &SeaOrmAuthRepository, mode: RegistrationMode) {
        entities::application_setting::ActiveModel {
            key: Set("registration_mode".to_string()),
            value: Set(mode.as_str().to_string()),
            updated_at: Set(Utc::now()),
            updated_by: Set(None),
        }
        .insert(repository.connection())
        .await
        .unwrap();
    }

    fn registration_user(id: &str, email: &str) -> User {
        let now = Utc::now();
        User {
            id: id.to_string(),
            username: id.to_string(),
            email: email.to_string(),
            password_hash: "unused".to_string(),
            roles: vec![crate::ROLE_ADMIN.to_string()],
            active: true,
            organization_id: Some(uuid::Uuid::new_v4().to_string()),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        }
    }

    #[tokio::test]
    async fn concurrent_challenge_verification_issues_exactly_one_ticket() {
        let (_directory, first, second) = repositories().await;
        let now = Utc::now();
        entities::email_challenge::ActiveModel {
            id: Set("challenge".to_string()),
            email: Set("verified@example.test".to_string()),
            code_hash: Set("expected-code".to_string()),
            source_hash: Set("source".to_string()),
            created_at: Set(now),
            expires_at: Set(now + chrono::Duration::minutes(10)),
            resend_after: Set(now),
            attempts: Set(0),
            send_count: Set(1),
            verified_at: Set(None),
            invitation_id: Set(None),
        }
        .insert(first.connection())
        .await
        .unwrap();
        let ticket = |id: &str| NewRegistrationTicket {
            id: id.to_string(),
            email: "verified@example.test".to_string(),
            token_hash: format!("hash-{id}"),
            issued_at: now,
            expires_at: now + chrono::Duration::minutes(20),
            invitation_id: None,
        };
        let (left, right) = tokio::join!(
            first.verify_challenge_and_issue_ticket("challenge", "expected-code", ticket("one")),
            second.verify_challenge_and_issue_ticket("challenge", "expected-code", ticket("two")),
        );
        assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
        assert_eq!(
            entities::registration_ticket::Entity::find()
                .count(first.connection())
                .await
                .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn concurrent_ticket_consumption_creates_exactly_one_admin_tenant() {
        let (_directory, first, second) = repositories().await;
        set_mode(&first, RegistrationMode::Open).await;
        let now = Utc::now();
        entities::registration_ticket::ActiveModel {
            id: Set("ticket".to_string()),
            email: Set("verified@example.test".to_string()),
            token_hash: Set("single-use".to_string()),
            issued_at: Set(now),
            expires_at: Set(now + chrono::Duration::minutes(20)),
            consumed_at: Set(None),
            invitation_id: Set(None),
        }
        .insert(first.connection())
        .await
        .unwrap();
        let (left, right) = tokio::join!(
            first.consume_ticket_and_create_user(
                "single-use",
                now,
                registration_user("first-user", "verified@example.test"),
            ),
            second.consume_ticket_and_create_user(
                "single-use",
                now,
                registration_user("second-user", "verified@example.test"),
            ),
        );
        assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
        let users = entities::user::Entity::find()
            .all(first.connection())
            .await
            .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(
            serde_json::from_value::<Vec<String>>(users[0].roles.clone()).unwrap(),
            vec![crate::ROLE_ADMIN.to_string()]
        );
        assert!(!users[0].tenant_id.is_empty());
    }

    #[tokio::test]
    async fn separate_password_tickets_can_claim_a_normalized_email_only_once() {
        let (_directory, first, second) = repositories().await;
        set_mode(&first, RegistrationMode::Open).await;
        let now = Utc::now();
        for (id, email) in [
            ("first-ticket", "same.person@example.test"),
            ("second-ticket", "same.person@example.test"),
        ] {
            entities::registration_ticket::ActiveModel {
                id: Set(id.to_string()),
                email: Set(email.to_string()),
                token_hash: Set(format!("hash-{id}")),
                issued_at: Set(now),
                expires_at: Set(now + chrono::Duration::minutes(20)),
                consumed_at: Set(None),
                invitation_id: Set(None),
            }
            .insert(first.connection())
            .await
            .unwrap();
        }

        let (left, right) = tokio::join!(
            first.consume_ticket_and_create_user(
                "hash-first-ticket",
                now,
                registration_user("first-user", "same.person@example.test"),
            ),
            second.consume_ticket_and_create_user(
                "hash-second-ticket",
                now,
                registration_user("second-user", "same.person@example.test"),
            ),
        );
        assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
        assert_eq!(
            entities::user::Entity::find()
                .count(first.connection())
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            entities::identity_email_claim::Entity::find()
                .count(first.connection())
                .await
                .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn concurrent_oidc_subjects_can_claim_a_normalized_email_only_once() {
        let (_directory, first, second) = repositories().await;
        set_mode(&first, RegistrationMode::Open).await;
        let email = "same.person@example.test";
        let first_user = registration_user("google-user", email);
        let second_user = registration_user("keycloak-user", email);
        let (left, right) = tokio::join!(
            first.create_oidc_user(
                first_user.clone(),
                "google",
                "https://accounts.google.com",
                "google-subject",
            ),
            second.create_oidc_user(
                second_user.clone(),
                "keycloak",
                "https://idp.example.test",
                "keycloak-subject",
            ),
        );
        let mut successes = usize::from(left.is_ok()) + usize::from(right.is_ok());
        if left.is_err() {
            successes += usize::from(first
                .create_oidc_user(
                    first_user,
                    "google",
                    "https://accounts.google.com",
                    "google-subject",
                )
                .await
                .is_ok());
        }
        if right.is_err() {
            successes += usize::from(second
                .create_oidc_user(
                    second_user,
                    "keycloak",
                    "https://idp.example.test",
                    "keycloak-subject",
                )
                .await
                .is_ok());
        }
        assert_eq!(successes, 1);
        assert_eq!(entities::user::Entity::find().count(first.connection()).await.unwrap(), 1);
        assert_eq!(
            entities::external_identity::Entity::find()
                .count(first.connection())
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            entities::identity_email_claim::Entity::find()
                .count(first.connection())
                .await
                .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn open_oidc_registration_does_not_consume_a_matching_invitation() {
        let (_directory, first, _second) = repositories().await;
        set_mode(&first, RegistrationMode::Open).await;
        let now = Utc::now();
        entities::invitation::ActiveModel {
            id: Set("unused-invitation".to_string()),
            email: Set("invited@example.test".to_string()),
            token_hash: Set("unused-token".to_string()),
            created_at: Set(now),
            expires_at: Set(now + chrono::Duration::hours(1)),
            consumed_at: Set(None),
            created_by: Set("missing-creator".to_string()),
        }
        .insert(first.connection())
        .await
        .unwrap();

        first
            .create_oidc_user(
                registration_user("open-user", "invited@example.test"),
                "google",
                "https://accounts.google.com",
                "open-subject",
            )
            .await
            .unwrap();

        let invitation = entities::invitation::Entity::find_by_id("unused-invitation")
            .one(first.connection())
            .await
            .unwrap()
            .unwrap();
        assert!(invitation.consumed_at.is_none());
    }

    #[tokio::test]
    async fn update_user_cannot_adopt_another_claimed_email() {
        let (_directory, first, _second) = repositories().await;
        let tenant_id = "tenant-a";
        first
            .create_user(
                registration_user("first-user", "first@example.test"),
                tenant_id,
            )
            .await
            .unwrap();
        first
            .create_user(
                registration_user("second-user", "second@example.test"),
                tenant_id,
            )
            .await
            .unwrap();
        let mut first_user = first.get_by_id("first-user", tenant_id).await.unwrap();
        first_user.email = "SECOND@EXAMPLE.TEST".to_string();

        let error = first
            .update_user(first_user, tenant_id)
            .await
            .unwrap_err();

        assert_eq!(error, "email changes require verification");
        assert_eq!(
            first
                .get_by_id("first-user", tenant_id)
                .await
                .unwrap()
                .email,
            "first@example.test"
        );
        for (email, owner) in [
            ("first@example.test", "first-user"),
            ("second@example.test", "second-user"),
        ] {
            let claim = entities::identity_email_claim::Entity::find_by_id(email)
                .one(first.connection())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(claim.user_id, owner);
        }
    }
}
