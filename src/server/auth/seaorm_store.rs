use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, Statement,
    TransactionTrait,
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

    pub mod identity_user_role {
        use super::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "identity_user_roles")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub user_id: String,
            #[sea_orm(primary_key, auto_increment = false)]
            pub role_name: String,
            pub tenant_id: String,
            pub position: i32,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(
                belongs_to = "super::user::Entity",
                from = "Column::UserId",
                to = "super::user::Column::Id",
                on_update = "Cascade",
                on_delete = "Cascade"
            )]
            User,
        }
        impl Related<super::user::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::User.def()
            }
        }
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
    let creator_roles = roles_for_user(transaction, &creator.id).await?;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmailChallengeCreation {
    Created,
    Throttled { retry_after_seconds: u64 },
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

pub async fn begin_global_transaction(
    connection: &DatabaseConnection,
) -> Result<DatabaseTransaction, String> {
    let transaction = connection.begin().await.map_err(db_error)?;
    if transaction.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
        transaction
            .execute(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                "SET LOCAL ROLE ohc_bypassrls".to_string(),
            ))
            .await
            .map_err(db_error)?;
    }
    Ok(transaction)
}

pub async fn begin_tenant_transaction(
    connection: &DatabaseConnection,
    tenant_id: &str,
) -> Result<DatabaseTransaction, String> {
    let backend = connection.get_database_backend();
    if backend == sea_orm::DatabaseBackend::Postgres && ::server_config::get().multitenant {
        if tenant_id.trim().eq_ignore_ascii_case("system") {
            return Err("tenant_id 'system' cannot be queried in multi-tenant mode".to_string());
        }
        if tenant_id.trim().is_empty() {
            return Err("empty tenant_id is not allowed in multi-tenant mode".to_string());
        }
    }
    if backend == sea_orm::DatabaseBackend::Postgres
        && tenant_id.trim().eq_ignore_ascii_case("system")
    {
        return begin_global_transaction(connection).await;
    }
    let transaction = connection.begin().await.map_err(db_error)?;
    if backend == sea_orm::DatabaseBackend::Postgres {
        transaction
            .execute(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                "SELECT set_config('role', 'none', true), set_config('app.current_tenant', $1, true)",
                [tenant_id.into()],
            ))
            .await
            .map_err(db_error)?;
    }
    Ok(transaction)
}

async fn roles_for_user<C>(connection: &C, user_id: &str) -> Result<Vec<String>, String>
where
    C: ConnectionTrait,
{
    entities::identity_user_role::Entity::find()
        .filter(entities::identity_user_role::Column::UserId.eq(user_id))
        .order_by_asc(entities::identity_user_role::Column::Position)
        .order_by_asc(entities::identity_user_role::Column::RoleName)
        .all(connection)
        .await
        .map_err(db_error)
        .map(|roles| roles.into_iter().map(|role| role.role_name).collect())
}

pub async fn replace_user_roles(
    transaction: &DatabaseTransaction,
    user_id: &str,
    tenant_id: &str,
    roles: &[String],
) -> Result<(), String> {
    use entities::identity_user_role;

    identity_user_role::Entity::delete_many()
        .filter(identity_user_role::Column::UserId.eq(user_id))
        .filter(identity_user_role::Column::TenantId.eq(tenant_id))
        .exec(transaction)
        .await
        .map_err(db_error)?;

    let mut seen = std::collections::HashSet::new();
    for (position, role_name) in roles.iter().enumerate() {
        if !seen.insert(role_name) {
            continue;
        }
        identity_user_role::ActiveModel {
            user_id: Set(user_id.to_string()),
            role_name: Set(role_name.clone()),
            tenant_id: Set(tenant_id.to_string()),
            position: Set(position as i32),
        }
        .insert(transaction)
        .await
        .map_err(db_error)?;
    }

    let (sql, values) = match transaction.get_database_backend() {
        sea_orm::DatabaseBackend::Postgres => (
            "UPDATE users SET roles = COALESCE((SELECT array_agg(role_name ORDER BY position, role_name) FROM identity_user_roles WHERE user_id = $1 AND tenant_id = $2), ARRAY[]::TEXT[]) WHERE id = $1 AND tenant_id = $2",
            vec![user_id.into(), tenant_id.into()],
        ),
        sea_orm::DatabaseBackend::MySql => (
            "UPDATE users SET roles = COALESCE((SELECT JSON_ARRAYAGG(role_name) FROM (SELECT role_name FROM identity_user_roles WHERE user_id = ? AND tenant_id = ? ORDER BY position, role_name) ordered_roles), JSON_ARRAY()) WHERE id = ? AND tenant_id = ?",
            vec![
                user_id.into(),
                tenant_id.into(),
                user_id.into(),
                tenant_id.into(),
            ],
        ),
        sea_orm::DatabaseBackend::Sqlite => (
            "UPDATE users SET roles = COALESCE((SELECT json_group_array(role_name) FROM (SELECT role_name FROM identity_user_roles WHERE user_id = ? AND tenant_id = ? ORDER BY position, role_name)), '[]') WHERE id = ? AND tenant_id = ?",
            vec![
                user_id.into(),
                tenant_id.into(),
                user_id.into(),
                tenant_id.into(),
            ],
        ),
    };
    transaction
        .execute(Statement::from_sql_and_values(
            transaction.get_database_backend(),
            sql,
            values,
        ))
        .await
        .map_err(db_error)?;
    Ok(())
}

impl SeaOrmAuthRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn registration_mode(&self) -> Result<RegistrationMode, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let value = entities::application_setting::Entity::find_by_id("registration_mode")
            .one(&transaction)
            .await
            .map_err(db_error)?
            .map(|setting| setting.value)
            .unwrap_or_else(|| "closed".to_string());
        let mode = RegistrationMode::parse(&value)?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(mode)
    }

    pub async fn set_registration_mode(
        &self,
        mode: RegistrationMode,
        actor: &str,
        now: DateTime<Utc>,
    ) -> Result<(), String> {
        use entities::application_setting;
        let transaction = begin_global_transaction(&self.connection).await?;
        if let Some(setting) = application_setting::Entity::find_by_id("registration_mode")
            .one(&transaction)
            .await
            .map_err(db_error)?
        {
            let mut active: application_setting::ActiveModel = setting.into();
            active.value = Set(mode.as_str().to_string());
            active.updated_at = Set(now);
            active.updated_by = Set(Some(actor.to_string()));
            active.update(&transaction).await.map_err(db_error)?;
        } else {
            application_setting::ActiveModel {
                key: Set("registration_mode".to_string()),
                value: Set(mode.as_str().to_string()),
                updated_at: Set(now),
                updated_by: Set(Some(actor.to_string())),
            }
            .insert(&transaction)
            .await
            .map_err(db_error)?;
        }
        transaction.commit().await.map_err(db_error)
    }

    pub async fn create_email_challenge(
        &self,
        challenge: NewEmailChallenge,
    ) -> Result<EmailChallengeCreation, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let email = challenge.email.clone();
        entities::email_challenge::Entity::delete_many()
            .filter(entities::email_challenge::Column::Email.eq(&email))
            .filter(
                Condition::any()
                    .add(entities::email_challenge::Column::VerifiedAt.is_not_null())
                    .add(entities::email_challenge::Column::ExpiresAt.lte(challenge.created_at))
                    .add(entities::email_challenge::Column::ResendAfter.lte(challenge.created_at)),
            )
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        let inserted = entities::email_challenge::ActiveModel {
            id: Set(challenge.id),
            email: Set(email.clone()),
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
        .insert(&transaction)
        .await;
        match inserted {
            Ok(_) => {
                transaction.commit().await.map_err(db_error)?;
                Ok(EmailChallengeCreation::Created)
            }
            Err(error) if is_unique_violation(&error) => {
                transaction.rollback().await.map_err(db_error)?;
                let retry_after_seconds = self
                    .latest_email_challenge(&email)
                    .await?
                    .map(|existing| {
                        (existing.resend_after - Utc::now()).num_seconds().max(1) as u64
                    })
                    .unwrap_or(1);
                Ok(EmailChallengeCreation::Throttled {
                    retry_after_seconds,
                })
            }
            Err(error) => {
                transaction.rollback().await.map_err(db_error)?;
                Err(db_error(error))
            }
        }
    }

    pub async fn email_challenge(
        &self,
        id: &str,
    ) -> Result<Option<entities::email_challenge::Model>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let challenge = entities::email_challenge::Entity::find_by_id(id)
            .one(&transaction)
            .await
            .map_err(db_error)?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(challenge)
    }

    pub async fn latest_email_challenge(
        &self,
        email: &str,
    ) -> Result<Option<entities::email_challenge::Model>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let challenge = entities::email_challenge::Entity::find()
            .filter(entities::email_challenge::Column::Email.eq(email))
            .order_by_desc(entities::email_challenge::Column::CreatedAt)
            .one(&transaction)
            .await
            .map_err(db_error)?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(challenge)
    }

    pub async fn delete_email_challenge(&self, id: &str) -> Result<(), String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        entities::email_challenge::Entity::delete_by_id(id)
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        transaction.commit().await.map_err(db_error)
    }

    pub async fn registration_ticket_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<entities::registration_ticket::Model>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let tickets = entities::registration_ticket::Entity::find()
            .filter(entities::registration_ticket::Column::TokenHash.eq(token_hash))
            .limit(2)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let ticket = (tickets.len() == 1).then(|| tickets[0].clone());
        transaction.rollback().await.map_err(db_error)?;
        Ok(ticket)
    }

    pub async fn record_challenge_failure(&self, id: &str, attempts: i32) -> Result<bool, String> {
        use entities::email_challenge;
        let transaction = begin_global_transaction(&self.connection).await?;
        let result = email_challenge::Entity::update_many()
            .col_expr(
                email_challenge::Column::Attempts,
                sea_orm::sea_query::Expr::value(attempts + 1),
            )
            .filter(email_challenge::Column::Id.eq(id))
            .filter(email_challenge::Column::Attempts.eq(attempts))
            .filter(email_challenge::Column::VerifiedAt.is_null())
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        transaction.commit().await.map_err(db_error)?;
        Ok(result.rows_affected == 1)
    }

    pub async fn verify_challenge_and_issue_ticket(
        &self,
        challenge_id: &str,
        expected_code_hash: &str,
        ticket: NewRegistrationTicket,
    ) -> Result<(), String> {
        use entities::{email_challenge, registration_ticket};
        let transaction = begin_global_transaction(&self.connection).await?;
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
            Some(invitation_id) => {
                claim.filter(email_challenge::Column::InvitationId.eq(invitation_id))
            }
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
        let transaction = begin_global_transaction(&self.connection).await?;
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
            RegistrationMode::InviteOnly => {
                invitation.ok_or_else(|| "invitation unavailable".to_string())?
            }
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
            active: Set(user.active),
            tenant_id: Set(tenant_id.clone()),
            oidc_subject: Set(user.oidc_subject.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        replace_user_roles(&transaction, &user.id, &tenant_id, &user.roles).await?;

        transaction.commit().await.map_err(db_error)?;
        Ok(user)
    }

    pub async fn public_oidc_providers(&self) -> Result<Vec<PublicOidcProvider>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let providers = entities::oidc_provider::Entity::find()
            .filter(entities::oidc_provider::Column::Enabled.eq(true))
            .order_by_asc(entities::oidc_provider::Column::DisplayName)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let providers = providers
            .into_iter()
            .map(|provider| PublicOidcProvider {
                key: provider.key,
                display_name: provider.display_name,
                provider_kind: provider.provider_kind,
            })
            .collect();
        transaction.rollback().await.map_err(db_error)?;
        Ok(providers)
    }

    pub async fn admin_oidc_providers(&self) -> Result<Vec<AdminOidcProvider>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let providers = entities::oidc_provider::Entity::find()
            .order_by_asc(entities::oidc_provider::Column::DisplayName)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let providers = providers
            .into_iter()
            .map(|provider| AdminOidcProvider {
                key: provider.key,
                display_name: provider.display_name,
                provider_kind: provider.provider_kind,
                issuer: provider.issuer,
                configured: !provider.client_id.trim().is_empty()
                    && !provider.secret_ref.trim().is_empty()
                    && std::env::var(&provider.secret_ref)
                        .is_ok_and(|secret| !secret.trim().is_empty()),
                enabled: provider.enabled,
            })
            .collect();
        transaction.rollback().await.map_err(db_error)?;
        Ok(providers)
    }

    pub async fn oidc_provider(&self, key: &str) -> Result<Option<OidcProviderConfig>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let provider = entities::oidc_provider::Entity::find_by_id(key)
            .one(&transaction)
            .await
            .map_err(db_error)?
            .map(|provider| OidcProviderConfig {
                key: provider.key,
                display_name: provider.display_name,
                issuer: provider.issuer,
                client_id: provider.client_id,
                secret_ref: provider.secret_ref,
                enabled: provider.enabled,
            });
        transaction.rollback().await.map_err(db_error)?;
        Ok(provider)
    }

    pub async fn set_oidc_provider_enabled(
        &self,
        key: &str,
        enabled: bool,
        now: DateTime<Utc>,
    ) -> Result<bool, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let Some(provider) = entities::oidc_provider::Entity::find_by_id(key)
            .one(&transaction)
            .await
            .map_err(db_error)?
        else {
            transaction.rollback().await.map_err(db_error)?;
            return Ok(false);
        };
        let mut active: entities::oidc_provider::ActiveModel = provider.into();
        active.enabled = Set(enabled);
        active.updated_at = Set(now);
        active.update(&transaction).await.map_err(db_error)?;
        transaction.commit().await.map_err(db_error)?;
        Ok(true)
    }

    pub async fn sync_configured_oidc_providers_from_environment(&self) -> Result<(), String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let mut configured = Vec::new();
        let google_client_id = std::env::var("OHC_OIDC_GOOGLE_CLIENT_ID").ok();
        let google_secret = std::env::var("OHC_OIDC_GOOGLE_CLIENT_SECRET").ok();
        if let (Some(client_id), Some(secret)) = (google_client_id, google_secret) {
            if !client_id.trim().is_empty() && !secret.trim().is_empty() {
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
        let keycloak_issuer = std::env::var("OHC_OIDC_KEYCLOAK_ISSUER").ok();
        let keycloak_client_id = std::env::var("OHC_OIDC_KEYCLOAK_CLIENT_ID").ok();
        let keycloak_secret = std::env::var("OHC_OIDC_KEYCLOAK_CLIENT_SECRET").ok();
        if let (Some(issuer), Some(client_id), Some(secret)) =
            (keycloak_issuer, keycloak_client_id, keycloak_secret)
        {
            if !issuer.trim().is_empty()
                && !client_id.trim().is_empty()
                && !secret.trim().is_empty()
            {
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
                    .one(&transaction)
                    .await
                    .map_err(db_error)?
                {
                    let mut active: entities::oidc_provider::ActiveModel = existing.into();
                    active.client_id = Set(String::new());
                    active.secret_ref = Set(String::new());
                    active.enabled = Set(false);
                    active.updated_at = Set(Utc::now());
                    active.update(&transaction).await.map_err(db_error)?;
                }
            }
        }
        for (key, display_name, provider_kind, issuer, client_id, secret_ref) in configured {
            if let Some(existing) = entities::oidc_provider::Entity::find_by_id(&key)
                .one(&transaction)
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
                active.update(&transaction).await.map_err(db_error)?;
            } else {
                let now = Utc::now();
                let backend = transaction.get_database_backend();
                let quote = |identifier: &str| match backend {
                    sea_orm::DatabaseBackend::MySql => format!("`{identifier}`"),
                    _ => format!("\"{identifier}\""),
                };
                let placeholders: Vec<String> = match backend {
                    sea_orm::DatabaseBackend::Postgres => {
                        (1..=10).map(|index| format!("${index}")).collect()
                    }
                    _ => vec!["?".to_string(); 10],
                };
                let keyword = match backend {
                    sea_orm::DatabaseBackend::MySql => "INSERT IGNORE",
                    sea_orm::DatabaseBackend::Sqlite => "INSERT OR IGNORE",
                    _ => "INSERT",
                };
                let mut sql = format!(
                    "{keyword} INTO {} ({}) VALUES ({})",
                    quote("oidc_providers"),
                    [
                        "key",
                        "display_name",
                        "provider_kind",
                        "issuer",
                        "client_id",
                        "scopes",
                        "secret_ref",
                        "enabled",
                        "created_at",
                        "updated_at",
                    ]
                    .iter()
                    .map(|column| quote(column))
                    .collect::<Vec<_>>()
                    .join(", "),
                    placeholders.join(", ")
                );
                if backend == sea_orm::DatabaseBackend::Postgres {
                    sql.push_str(" ON CONFLICT DO NOTHING");
                }
                transaction
                    .execute(Statement::from_sql_and_values(
                        backend,
                        sql,
                        vec![
                            key.into(),
                            display_name.into(),
                            provider_kind.into(),
                            issuer.trim_end_matches('/').to_string().into(),
                            client_id.into(),
                            serde_json::json!(["openid", "email", "profile"]).into(),
                            secret_ref.into(),
                            false.into(),
                            now.into(),
                            now.into(),
                        ],
                    ))
                    .await
                    .map_err(db_error)?;
            }
        }
        transaction.commit().await.map_err(db_error)
    }

    pub async fn user_for_external_identity(
        &self,
        provider_key: &str,
        issuer: &str,
        subject: &str,
    ) -> Result<Option<User>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let identities = entities::external_identity::Entity::find()
            .filter(entities::external_identity::Column::ProviderKey.eq(provider_key))
            .filter(entities::external_identity::Column::Issuer.eq(issuer))
            .filter(entities::external_identity::Column::Subject.eq(subject))
            .limit(2)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        if identities.len() != 1 {
            transaction.rollback().await.map_err(db_error)?;
            return Ok(None);
        }
        let user = entities::user::Entity::find_by_id(&identities[0].user_id)
            .one(&transaction)
            .await
            .map_err(db_error)?;
        let user = match user {
            Some(user) => Some(model_to_user(user, &transaction).await?),
            None => None,
        };
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    pub async fn active_invitation_id_by_token(
        &self,
        email: &str,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<String>, String> {
        let transaction = begin_global_transaction(&self.connection).await?;
        let invitations = entities::invitation::Entity::find()
            .filter(entities::invitation::Column::Email.eq(email))
            .filter(entities::invitation::Column::TokenHash.eq(token_hash))
            .filter(entities::invitation::Column::ConsumedAt.is_null())
            .filter(entities::invitation::Column::ExpiresAt.gt(now))
            .limit(2)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let invitation = (invitations.len() == 1).then(|| invitations[0].id.clone());
        transaction.rollback().await.map_err(db_error)?;
        Ok(invitation)
    }

    pub async fn create_oidc_user(
        &self,
        mut user: User,
        provider_key: &str,
        issuer: &str,
        subject: &str,
    ) -> Result<User, String> {
        use entities::{external_identity, invitation, user as user_entity};
        let transaction = begin_global_transaction(&self.connection).await?;
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
            active: Set(true),
            tenant_id: Set(tenant_id.clone()),
            oidc_subject: Set(user.oidc_subject.clone()),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        replace_user_roles(&transaction, &user.id, &tenant_id, &user.roles).await?;
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

async fn model_to_user<C>(model: entities::user::Model, connection: &C) -> Result<User, String>
where
    C: ConnectionTrait,
{
    let roles = roles_for_user(connection, &model.id).await?;
    Ok(User {
        id: model.id,
        username: model.username,
        email: model.email,
        password_hash: model.password_hash,
        roles,
        active: model.active,
        organization_id: Some(model.tenant_id),
        created_at: model.created_at,
        updated_at: model.updated_at,
        oidc_subject: model.oidc_subject,
    })
}

fn db_error(error: sea_orm::DbErr) -> String {
    tracing::error!(event = "auth.persistence.unavailable", error_kind = ?error.sql_err());
    "authentication persistence unavailable".to_string()
}

fn is_unique_violation(error: &sea_orm::DbErr) -> bool {
    matches!(
        error.sql_err(),
        Some(sea_orm::SqlErr::UniqueConstraintViolation(_))
    )
}

#[async_trait]
impl UserRepository for SeaOrmAuthRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let mut user = user;
        user.organization_id = Some(org_id.to_string());
        let normalized_email = crate::validation::normalize_email(&user.email)
            .map_err(|_| "email already registered".to_string())?;
        user.email = normalized_email.clone();
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        claim_identity_email(
            &transaction,
            &normalized_email,
            &user.id,
            user.created_at,
            "email already registered",
        )
        .await?;
        entities::user::ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            active: Set(user.active),
            tenant_id: Set(org_id.to_string()),
            oidc_subject: Set(user.oidc_subject),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
        .insert(&transaction)
        .await
        .map_err(db_error)?;
        replace_user_roles(&transaction, &user.id, org_id, &user.roles).await?;
        transaction.commit().await.map_err(db_error)
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let model = entities::user::Entity::find_by_id(id)
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let user = model_to_user(model, &transaction).await?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let user = model_to_user(model, &transaction).await?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::Email.eq(email))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let user = model_to_user(model, &transaction).await?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    async fn get_by_login_identifier(
        &self,
        identifier: &str,
        org_id: &str,
    ) -> Result<Option<User>, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let models = entities::user::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(entities::user::Column::Username.eq(identifier))
                    .add(entities::user::Column::Email.eq(identifier)),
            )
            .filter(entities::user::Column::TenantId.eq(org_id))
            .filter(entities::user::Column::Active.eq(true))
            .limit(2)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let user = if models.len() == 1 {
            Some(model_to_user(models[0].clone(), &transaction).await?)
        } else {
            None
        };
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    async fn get_by_oidc_subject(&self, subject: &str, org_id: &str) -> Result<User, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let model = entities::user::Entity::find()
            .filter(entities::user::Column::OidcSubject.eq(subject))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "user not found".to_string())?;
        let user = model_to_user(model, &transaction).await?;
        transaction.rollback().await.map_err(db_error)?;
        Ok(user)
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let models = entities::user::Entity::find()
            .filter(entities::user::Column::TenantId.eq(org_id))
            .order_by_asc(entities::user::Column::CreatedAt)
            .all(&transaction)
            .await
            .map_err(db_error)?;
        let mut users = Vec::with_capacity(models.len());
        for model in models {
            users.push(model_to_user(model, &transaction).await?);
        }
        transaction.rollback().await.map_err(db_error)?;
        Ok(users)
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
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
        active.active = Set(user.active);
        active.oidc_subject = Set(user.oidc_subject);
        active.updated_at = Set(user.updated_at);
        active.update(&transaction).await.map_err(db_error)?;
        replace_user_roles(&transaction, &user.id, org_id, &user.roles).await?;
        transaction.commit().await.map_err(db_error)
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        entities::identity_user_role::Entity::delete_many()
            .filter(entities::identity_user_role::Column::UserId.eq(id))
            .filter(entities::identity_user_role::Column::TenantId.eq(org_id))
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        entities::user::Entity::delete_many()
            .filter(entities::user::Column::Id.eq(id))
            .filter(entities::user::Column::TenantId.eq(org_id))
            .exec(&transaction)
            .await
            .map_err(db_error)?;
        transaction.commit().await.map_err(db_error)
    }

    async fn revoke_token(
        &self,
        jti: String,
        exp: DateTime<Utc>,
        org_id: &str,
    ) -> Result<(), String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        if entities::revoked_token::Entity::find_by_id(&jti)
            .filter(entities::revoked_token::Column::TenantId.eq(org_id))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .is_none()
        {
            entities::revoked_token::ActiveModel {
                jti: Set(jti),
                tenant_id: Set(org_id.to_string()),
                expires_at: Set(exp),
            }
            .insert(&transaction)
            .await
            .map_err(db_error)?;
        }
        transaction.commit().await.map_err(db_error)
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        let transaction = begin_tenant_transaction(&self.connection, org_id).await?;
        let revoked = entities::revoked_token::Entity::find_by_id(jti)
            .filter(entities::revoked_token::Column::TenantId.eq(org_id))
            .filter(entities::revoked_token::Column::ExpiresAt.gt(Utc::now()))
            .one(&transaction)
            .await
            .map_err(db_error)?
            .is_some();
        transaction.rollback().await.map_err(db_error)?;
        Ok(revoked)
    }
}

