//! Durable public definitions and inactive, owner-scoped installation snapshots.
//! This module has no agent runtime, provider, tool-grant or dispatch dependency.
use super::{capabilities::DatabaseBackend, connection::AppDatabase};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sea_orm::{
    ConnectionTrait, DatabaseTransaction, DbErr, IsolationLevel, Statement, TransactionTrait,
    TryGetableMany, Value,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub enum DefinitionStore {
    Database(AppDatabase),
    Unavailable,
}
#[derive(Clone, Debug)]
pub struct Owner {
    pub tenant: String,
    pub user: String,
}
#[derive(Debug)]
pub enum Error {
    Invalid,
    Forbidden,
    Conflict,
    NotFound,
    Unavailable,
    Database(DbErr),
    Corrupt,
}
impl From<DbErr> for Error {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}
impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::Corrupt
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishRequest {
    pub request_id: Uuid,
    pub name: String,
    pub description: String,
    pub role: String,
    pub system_prompt: String,
    pub visibility: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallRequest {
    pub request_id: Uuid,
    pub version: i64,
    pub digest: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub id: String,
    pub version: i64,
    pub digest: String,
    pub name: String,
    pub description: String,
    pub role: String,
    pub system_prompt: String,
    pub visibility: String,
    pub source: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Installation {
    pub id: String,
    pub definition_id: String,
    pub version: i64,
    pub digest: String,
    pub role_key: String,
    pub name: String,
    pub role: String,
    pub system_prompt: String,
    pub status: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub success: bool,
    pub status: String,
    pub request_id: String,
    pub organization_id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Definition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation: Option<Installation>,
    pub replayed: bool,
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListQuery {
    pub q: Option<String>,
    pub cursor: Option<String>,
    pub installation_cursor: Option<String>,
    pub limit: Option<usize>,
}
#[derive(Debug, Serialize)]
pub struct Catalogue {
    pub definitions: Vec<Definition>,
    pub installations: Vec<Installation>,
    pub next_cursor: Option<String>,
    pub next_installation_cursor: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    kind: String,
    scope: String,
    id: String,
    version: i64,
}
const COLLECTION_BYTES: usize = 800_000;
fn hash(value: &impl Serialize) -> Result<String, Error> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(value)?)))
}
fn valid_text(value: &str, max: usize, required: bool) -> bool {
    (!required || !value.trim().is_empty()) && value.chars().count() <= max && !value.contains('\0')
}
impl PublishRequest {
    pub fn validate(&self) -> Result<(), Error> {
        if !valid_text(&self.name, 120, true)
            || !valid_text(&self.role, 120, true)
            || !valid_text(&self.description, 2000, false)
            || !valid_text(&self.system_prompt, 16000, true)
            || self.visibility != "public"
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn definition_digest(value: &Definition) -> Result<String, Error> {
    hash(&(
        &value.name,
        &value.description,
        &value.role,
        &value.system_prompt,
        &value.visibility,
        &value.source,
        value.version,
    ))
}
fn decode_definition(bytes: &str) -> Result<Definition, Error> {
    let value: Definition = serde_json::from_str(bytes)?;
    PublishRequest {
        request_id: Uuid::nil(),
        name: value.name.clone(),
        description: value.description.clone(),
        role: value.role.clone(),
        system_prompt: value.system_prompt.clone(),
        visibility: value.visibility.clone(),
    }
    .validate()
    .map_err(|_| Error::Corrupt)?;
    if Uuid::parse_str(&value.id).is_err()
        || value.version < 1
        || !["first_party", "community"].contains(&value.source.as_str())
        || value.digest != definition_digest(&value)?
    {
        return Err(Error::Corrupt);
    }
    Ok(value)
}
fn checked_receipt(bytes: &str, owner: &Owner, request_id: Uuid) -> Result<Receipt, Error> {
    let mut receipt: Receipt = serde_json::from_str(bytes)?;
    if !receipt.success
        || receipt.organization_id != owner.tenant
        || receipt.user_id != owner.user
        || receipt.request_id != request_id.to_string()
        || !["published", "installed_inactive"].contains(&receipt.status.as_str())
    {
        return Err(Error::Corrupt);
    }
    match (
        &receipt.definition,
        &receipt.installation,
        receipt.status.as_str(),
    ) {
        (Some(definition), None, "published") => {
            decode_definition(&serde_json::to_string(definition)?)?;
        }
        (None, Some(installation), "installed_inactive") => {
            decode_installation(&serde_json::to_string(installation)?)?;
        }
        _ => return Err(Error::Corrupt),
    }
    receipt.replayed = true;
    Ok(receipt)
}
fn cursor(value: Option<&str>, kind: &str, scope: &str) -> Result<(String, i64), Error> {
    let Some(value) = value else {
        return Ok((String::new(), 0));
    };
    if value.len() > 2048 {
        return Err(Error::Invalid);
    }
    let bytes = URL_SAFE_NO_PAD.decode(value).map_err(|_| Error::Invalid)?;
    let c: Cursor = serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)?;
    if c.kind != kind || c.scope != scope || Uuid::parse_str(&c.id).is_err() || c.version < 1 {
        return Err(Error::Invalid);
    }
    Ok((c.id, c.version))
}
fn encode_cursor(kind: &str, scope: &str, id: &str, version: i64) -> Result<String, Error> {
    Ok(URL_SAFE_NO_PAD.encode(serde_json::to_vec(&Cursor {
        kind: kind.into(),
        scope: scope.into(),
        id: id.into(),
        version,
    })?))
}
// Bound statements use the configured portable connection and the same transaction.
// Tuple decoding fails closed on malformed stored rows.
fn statement(tx: &DatabaseTransaction, sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(tx.get_database_backend(), sql, values)
}
async fn execute(tx: &DatabaseTransaction, sql: &str, values: Vec<Value>) -> Result<(), Error> {
    tx.execute(statement(tx, sql, values)).await?;
    Ok(())
}
async fn fetch_optional<T: TryGetableMany>(
    tx: &DatabaseTransaction,
    sql: &str,
    values: Vec<Value>,
) -> Result<Option<T>, Error> {
    tx.query_one(statement(tx, sql, values))
        .await?
        .map(|row| row.try_get_many_by_index().map_err(Error::from))
        .transpose()
}
async fn fetch_one<T: TryGetableMany>(
    tx: &DatabaseTransaction,
    sql: &str,
    values: Vec<Value>,
) -> Result<T, Error> {
    fetch_optional(tx, sql, values).await?.ok_or_else(|| {
        Error::Database(DbErr::RecordNotFound(
            "required definition row missing".into(),
        ))
    })
}
async fn fetch_all<T: TryGetableMany>(
    tx: &DatabaseTransaction,
    sql: &str,
    values: Vec<Value>,
) -> Result<Vec<T>, Error> {
    tx.query_all(statement(tx, sql, values))
        .await?
        .into_iter()
        .map(|row| row.try_get_many_by_index().map_err(Error::from))
        .collect()
}
impl DefinitionStore {
    // Middleware validation cannot fence revocation between verification and commit.
    async fn transaction(
        &self,
        owner: &Owner,
        write: bool,
        lock: Option<&str>,
    ) -> Result<DatabaseTransaction, Error> {
        if !valid_text(&owner.tenant, 512, true)
            || owner.tenant.trim() != owner.tenant
            || owner.tenant.eq_ignore_ascii_case("system")
            || !valid_text(&owner.user, 512, true)
        {
            return Err(Error::Invalid);
        }
        let Self::Database(database) = self else {
            return Err(Error::Unavailable);
        };
        let tx = match database.backend() {
            DatabaseBackend::Postgres => {
                // A fresh statement snapshot after waiting for the authority gate is required.
                let tx = database
                    .connection()
                    .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
                    .await?;
                execute(&tx, "SELECT set_config('role', 'none', true), set_config('app.current_tenant', $1, true)", vec![(&owner.tenant).into()]).await?;
                execute(
                    &tx,
                    "SELECT set_config('app.current_actor',$1,true)",
                    vec![(&owner.user).into()],
                )
                .await?;
                if write {
                    execute(
                        &tx,
                        "SELECT pg_advisory_xact_lock_shared(57129048260865031)",
                        vec![],
                    )
                    .await?;
                }
                tx
            }
            DatabaseBackend::Sqlite => {
                let tx = database.connection().begin().await?;
                if write {
                    // SeaORM begins a deferred transaction. Acquire SQLite write intent before
                    // ANY read snapshot; this changes no rows and invokes no row trigger.
                    execute(
                        &tx,
                        "UPDATE agent_definition_operations SET request_id=request_id WHERE 0",
                        vec![],
                    )
                    .await?;
                }
                let enabled: i64 = fetch_one(&tx, "PRAGMA foreign_keys", vec![]).await?;
                if enabled != 1 {
                    return Err(Error::Unavailable);
                }
                tx
            }
            DatabaseBackend::MySql => return Err(Error::Unavailable),
        };
        let suffix = if database.backend() == DatabaseBackend::Postgres {
            " FOR SHARE"
        } else {
            ""
        };
        let identity: Option<(bool,bool)> = fetch_optional(&tx, &format!("SELECT COALESCE(active,FALSE),marketplace_eligible FROM users WHERE id=$1 AND tenant_id=$2{suffix}"), vec![(&owner.user).into(),(&owner.tenant).into()]).await?;
        let Some((true, eligible)) = identity else {
            return Err(Error::Forbidden);
        };
        if write {
            let roles: Vec<String> = fetch_all(
                &tx,
                "SELECT role_name FROM identity_user_roles WHERE user_id=$1 AND tenant_id=$2",
                vec![(&owner.user).into(), (&owner.tenant).into()],
            )
            .await?;
            if !eligible
                || !roles.iter().any(|role| {
                    role.eq_ignore_ascii_case("ADMIN") || role.eq_ignore_ascii_case("OWNER")
                })
            {
                return Err(Error::Forbidden);
            }
        }
        if database.backend() == DatabaseBackend::Postgres
            && let Some(key) = lock
        {
            execute(
                &tx,
                "SELECT pg_advisory_xact_lock(hashtextextended($1,0))",
                vec![key.into()],
            )
            .await?;
        }
        Ok(tx)
    }
}
impl DefinitionStore {
    pub async fn publish(&self, owner: &Owner, request: &PublishRequest) -> Result<Receipt, Error> {
        request.validate()?;
        let fingerprint = hash(&(
            "publish",
            &request.name,
            &request.description,
            &request.role,
            &request.system_prompt,
            &request.visibility,
        ))?;
        let key = serde_json::to_string(&(&owner.tenant, &owner.user, request.request_id))?;
        {
            let tx = self.transaction(owner, true, Some(&key)).await?;
            if let Some((prior,receipt))=fetch_optional::<(String,String)>(&tx, "SELECT fingerprint,receipt FROM agent_definition_operations WHERE tenant_id=$1 AND user_id=$2 AND request_id=$3", vec![(&owner.tenant).into(),(&owner.user).into(),(request.request_id.to_string()).into()]).await? {
                if prior!=fingerprint {return Err(Error::Conflict)}
                return checked_receipt(&receipt,owner,request.request_id);
            }
            let mut definition = Definition {
                id: Uuid::new_v4().to_string(),
                version: 1,
                digest: String::new(),
                name: request.name.clone(),
                description: request.description.clone(),
                role: request.role.clone(),
                system_prompt: request.system_prompt.clone(),
                visibility: "public".into(),
                source: "community".into(),
            };
            definition.digest = definition_digest(&definition)?;
            let search = format!(
                "{} {} {}",
                definition.name, definition.description, definition.role
            )
            .to_lowercase();
            execute(&tx, "INSERT INTO agent_definitions(id,version,digest,document,search_text,source,authority_key) VALUES($1,$2,$3,$4,$5,'community',(SELECT marketplace_authority_key FROM users WHERE id=$6 AND tenant_id=$7))", vec![(&definition.id).into(),(definition.version).into(),(&definition.digest).into(),(serde_json::to_string(&definition)?).into(),(search).into(),(&owner.user).into(),(&owner.tenant).into()]).await?;
            execute(&tx, "INSERT INTO agent_definition_publishers(definition_id,version,tenant_id,user_id) VALUES($1,$2,$3,$4)", vec![(&definition.id).into(),(definition.version).into(),(&owner.tenant).into(),(&owner.user).into()]).await?;
            let receipt = Receipt {
                success: true,
                status: "published".into(),
                request_id: request.request_id.to_string(),
                organization_id: owner.tenant.clone(),
                user_id: owner.user.clone(),
                definition: Some(definition),
                installation: None,
                replayed: false,
            };
            execute(&tx, "INSERT INTO agent_definition_operations(tenant_id,user_id,request_id,fingerprint,receipt) VALUES($1,$2,$3,$4,$5)", vec![(&owner.tenant).into(),(&owner.user).into(),(request.request_id.to_string()).into(),(&fingerprint).into(),(serde_json::to_string(&receipt)?).into()]).await?;
            tx.commit().await?;
            Ok(receipt)
        }
    }
}
impl DefinitionStore {
    pub async fn install(
        &self,
        owner: &Owner,
        id: Uuid,
        request: &InstallRequest,
    ) -> Result<Receipt, Error> {
        if request.version < 1 || !valid_digest(&request.digest) {
            return Err(Error::Invalid);
        }
        let fingerprint = hash(&("install", id, request.version, &request.digest))?;
        let key = serde_json::to_string(&(&owner.tenant, &owner.user, request.request_id))?;
        {
            let tx = self.transaction(owner, true, Some(&key)).await?;
            if let Some((prior,receipt))=fetch_optional::<(String,String)>(&tx, "SELECT fingerprint,receipt FROM agent_definition_operations WHERE tenant_id=$1 AND user_id=$2 AND request_id=$3", vec![(&owner.tenant).into(),(&owner.user).into(),(request.request_id.to_string()).into()]).await? {
                if prior!=fingerprint {return Err(Error::Conflict)}
                return checked_receipt(&receipt,owner,request.request_id);
            }
            let document: Option<String> = fetch_optional(&tx, "SELECT document FROM agent_definitions WHERE id=$1 AND version=$2 AND (source='first_party' OR EXISTS(SELECT 1 FROM agent_definition_authorities a WHERE a.authority_key=agent_definitions.authority_key AND a.eligible))", vec![(id.to_string()).into(),(request.version).into()]).await?;
            let definition = decode_definition(&document.ok_or(Error::Conflict)?)?;
            if definition.id != id.to_string()
                || definition.version != request.version
                || definition.digest != request.digest
            {
                return Err(Error::Conflict);
            }
            let installation_id = Uuid::new_v4().to_string();
            let mut blueprint = crate::domain::blueprint::SkillBlueprint {
                domain: definition.id.clone(),
                roles: vec![crate::domain::blueprint::RoleDefinition {
                    id: "agent".into(),
                    title: definition.role.clone(),
                    context: definition.system_prompt.clone(),
                    tools: vec![],
                    reports_to: String::new(),
                }],
            };
            blueprint.validate().map_err(|_| Error::Corrupt)?;
            blueprint.namespace_roles(&format!("marketplace/{installation_id}"));
            let proposed = Installation {
                id: installation_id,
                definition_id: definition.id.clone(),
                version: definition.version,
                digest: definition.digest.clone(),
                role_key: blueprint.roles[0].id.clone(),
                name: definition.name.clone(),
                role: definition.role.clone(),
                system_prompt: definition.system_prompt.clone(),
                status: "installed_inactive".into(),
            };
            execute(&tx, "INSERT INTO agent_definition_installations(id,tenant_id,user_id,definition_id,version,document) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(tenant_id,user_id,definition_id,version) DO NOTHING", vec![(&proposed.id).into(),(&owner.tenant).into(),(&owner.user).into(),(&definition.id).into(),(definition.version).into(),(serde_json::to_string(&proposed)?).into()]).await?;
            let document:String=fetch_one(&tx, "SELECT document FROM agent_definition_installations WHERE tenant_id=$1 AND user_id=$2 AND definition_id=$3 AND version=$4", vec![(&owner.tenant).into(),(&owner.user).into(),(&definition.id).into(),(definition.version).into()]).await?;
            let installation = decode_installation(&document)?;
            if installation.definition_id != definition.id
                || installation.version != definition.version
                || installation.digest != definition.digest
                || installation.name != definition.name
                || installation.role != definition.role
                || installation.system_prompt != definition.system_prompt
            {
                return Err(Error::Corrupt);
            }
            let receipt = Receipt {
                success: true,
                status: "installed_inactive".into(),
                request_id: request.request_id.to_string(),
                organization_id: owner.tenant.clone(),
                user_id: owner.user.clone(),
                definition: None,
                installation: Some(installation),
                replayed: false,
            };
            execute(&tx, "INSERT INTO agent_definition_operations(tenant_id,user_id,request_id,fingerprint,receipt) VALUES($1,$2,$3,$4,$5)", vec![(&owner.tenant).into(),(&owner.user).into(),(request.request_id.to_string()).into(),(&fingerprint).into(),(serde_json::to_string(&receipt)?).into()]).await?;
            tx.commit().await?;
            Ok(receipt)
        }
    }
    pub async fn operation(&self, owner: &Owner, request_id: Uuid) -> Result<Receipt, Error> {
        {
            let tx = self.transaction(owner, false, None).await?;
            let receipt:Option<String>=fetch_optional(&tx, "SELECT receipt FROM agent_definition_operations WHERE tenant_id=$1 AND user_id=$2 AND request_id=$3", vec![(&owner.tenant).into(),(&owner.user).into(),(request_id.to_string()).into()]).await?;
            checked_receipt(&receipt.ok_or(Error::NotFound)?, owner, request_id)
        }
    }
    pub async fn list(&self, owner: &Owner, query: &ListQuery) -> Result<Catalogue, Error> {
        let q = query.q.as_deref().unwrap_or_default();
        if !valid_text(q, 256, false) || !matches!(query.limit, None | Some(1..=100)) {
            return Err(Error::Invalid);
        }
        let limit = query.limit.unwrap_or(50);
        let definition_scope = hash(&(&owner.tenant, &owner.user, q))?;
        let installation_scope = hash(&(&owner.tenant, &owner.user))?;
        let (after_id, after_version) =
            cursor(query.cursor.as_deref(), "definitions", &definition_scope)?;
        let (after_installation, _) = cursor(
            query.installation_cursor.as_deref(),
            "installations",
            &installation_scope,
        )?;
        let pattern = format!(
            "%{}%",
            q.to_lowercase()
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        );
        {
            let tx = self.transaction(owner, false, None).await?;
            let rows=fetch_all::<(String,i64,String,String)>(&tx, "SELECT id,version,digest,document FROM agent_definitions WHERE (source='first_party' OR EXISTS(SELECT 1 FROM agent_definition_authorities a WHERE a.authority_key=agent_definitions.authority_key AND a.eligible)) AND (id>$1 OR (id=$1 AND version>$2)) AND search_text LIKE $3 ESCAPE '\\' ORDER BY id,version LIMIT $4", vec![(&after_id).into(),(after_version).into(),(&pattern).into(),((limit+1) as i64).into()]).await?;
            let mut definitions = vec![];
            let mut bytes = 0usize;
            let mut more = false;
            for (id, version, digest, document) in rows {
                if definitions.len() == limit || bytes + document.len() > COLLECTION_BYTES {
                    more = true;
                    break;
                }
                let definition = decode_definition(&document)?;
                if definition.id != id
                    || definition.version != version
                    || definition.digest != digest
                {
                    return Err(Error::Corrupt);
                }
                bytes += document.len();
                definitions.push(definition);
            }
            let next_cursor = if more {
                let last = definitions.last().ok_or(Error::Corrupt)?;
                Some(encode_cursor(
                    "definitions",
                    &definition_scope,
                    &last.id,
                    last.version,
                )?)
            } else {
                None
            };
            let rows=fetch_all::<(String,String,i64,String)>(&tx, "SELECT id,definition_id,version,document FROM agent_definition_installations WHERE tenant_id=$1 AND user_id=$2 AND id>$3 ORDER BY id LIMIT $4", vec![(&owner.tenant).into(),(&owner.user).into(),(&after_installation).into(),((limit+1) as i64).into()]).await?;
            let mut installations = vec![];
            let mut bytes = 0usize;
            let mut more = false;
            for (id, definition_id, version, document) in rows {
                if installations.len() == limit || bytes + document.len() > COLLECTION_BYTES {
                    more = true;
                    break;
                }
                let installation = decode_installation(&document)?;
                if installation.id != id
                    || installation.definition_id != definition_id
                    || installation.version != version
                {
                    return Err(Error::Corrupt);
                }
                bytes += document.len();
                installations.push(installation);
            }
            let next_installation_cursor = if more {
                let last = installations.last().ok_or(Error::Corrupt)?;
                Some(encode_cursor(
                    "installations",
                    &installation_scope,
                    &last.id,
                    last.version,
                )?)
            } else {
                None
            };
            tx.commit().await?;
            Ok(Catalogue {
                definitions,
                installations,
                next_cursor,
                next_installation_cursor,
            })
        }
    }
}
fn decode_installation(bytes: &str) -> Result<Installation, Error> {
    let value: Installation = serde_json::from_str(bytes)?;
    if Uuid::parse_str(&value.id).is_err()
        || Uuid::parse_str(&value.definition_id).is_err()
        || value.version < 1
        || !valid_digest(&value.digest)
        || value.role_key != format!("marketplace/{}/agent", value.id)
        || value.status != "installed_inactive"
        || !valid_text(&value.name, 120, true)
        || !valid_text(&value.role, 120, true)
        || !valid_text(&value.system_prompt, 16000, true)
    {
        return Err(Error::Corrupt);
    }
    Ok(value)
}
