use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use omnisolo_builtin_agent::gpt_researcher::ResearcherLlmClient;
use omnisolo_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Proposal {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub status: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub checkout_url: Option<String>,
    pub project_scope: Option<String>,
    pub milestones: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProposalLineItem {
    pub id: String,
    pub proposal_id: String,
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftAgentRequest {
    pub inquiry: String,
    pub customer_id: String,
}

const GET_PROPOSAL_SQL: &str = "SELECT * FROM proposals WHERE id = $1 AND tenant_id = $2";
const GET_LINE_ITEMS_SQL: &str = "SELECT pli.* FROM proposal_line_items pli JOIN proposals p ON p.id = pli.proposal_id WHERE pli.proposal_id = $1 AND p.tenant_id = $2";
const APPROVE_PROPOSAL_SQL: &str = "UPDATE proposals SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2 AND status = 'DRAFT' AND total_amount_cents > 0 RETURNING *";

fn authenticated_tenant(claims: &::server_common::Claims) -> Result<&str, StatusCode> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)
}

#[derive(Serialize)]
pub struct ProposalResponse {
    pub proposal: Proposal,
    pub line_items: Vec<ProposalLineItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LineItemRequest {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

#[derive(Deserialize)]
struct NarrativeDraftRequest {
    topic: String,
}

#[derive(Serialize)]
struct NarrativeDraftResponse {
    proposal: String,
}

struct AdapterLlm {}

#[async_trait::async_trait]
impl ResearcherLlmClient for AdapterLlm {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut prompt = req.system.clone();
        for msg in &req.messages {
            prompt.push_str("\n\n");
            prompt.push_str(&msg.content);
        }

        let observed = crate::minimax::LocalLLMClient::new()
            .reason_with_usage(&prompt, req.max_tokens)
            .await?;
        let counts = observed
            .counts
            .ok_or("Local provider omitted usage; draft accounting requires reconciliation")?;
        let input_tokens =
            i32::try_from(counts.input).map_err(|_| "Local input usage exceeds supported range")?;
        let output_tokens = i32::try_from(counts.output)
            .map_err(|_| "Local output usage exceeds supported range")?;
        let cache_read_input_tokens = i32::try_from(counts.cached_input)
            .map_err(|_| "Local cache usage exceeds supported range")?;
        Ok(ChatResponse {
            message: Message::assistant(observed.text),
            usage: Usage {
                input_tokens,
                output_tokens,
                cache_read_input_tokens,
                cache_creation_input_tokens: 0,
            },
            stop_reason: observed.stop_reason,
            // Ollama does not provide an OpenAI-style response id. Never invent one.
            response_id: None,
        })
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    router_with_narrative_llm(Arc::new(AdapterLlm {}))
}

fn router_with_narrative_llm<S>(llm: Arc<dyn ResearcherLlmClient>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/draft", post(draft_narrative))
        .route("/intake", post(client_intake))
        .route("/draft_agent", post(draft_agent))
        .route("/{id}", get(get_proposal))
        .route("/{id}/approve", post(approve_proposal))
        .route("/social/list", get(list_social_post_proposals))
        .layer(Extension(llm))
}

async fn draft_narrative(
    Extension(llm): Extension<Arc<dyn ResearcherLlmClient>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<NarrativeDraftRequest>,
) -> axum::response::Response {
    draft_narrative_with_llm(llm.as_ref(), &claims, payload).await
}

async fn draft_narrative_with_llm(
    llm: &dyn ResearcherLlmClient,
    claims: &::server_common::Claims,
    payload: NarrativeDraftRequest,
) -> axum::response::Response {
    if claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|organization_id| !organization_id.is_empty())
        .is_none()
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let topic = payload.topic.trim();
    if topic.is_empty() || topic.chars().count() > 4_000 {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let request = ChatRequest {
        model: "default-model".to_string(),
        system: "You draft concise, client-ready business proposals. Use these sections: Executive Summary, Scope, Milestones, Investment, and Next Steps. Be concrete, professional, and do not invent pricing or commitments not supplied by the user.".to_string(),
        messages: vec![Message::user(topic)],
        temperature: 0.2,
        max_tokens: 900,
        tools: vec![],
    };

    let response = match llm.chat(request).await {
        Ok(response) => response,
        Err(_) => {
            tracing::error!("Narrative proposal model request failed");
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };
    let proposal = response.message.content.trim();
    if proposal.is_empty() {
        tracing::error!("Narrative proposal model returned an empty response");
        return StatusCode::BAD_GATEWAY.into_response();
    }

    (
        StatusCode::OK,
        Json(NarrativeDraftResponse {
            proposal: proposal.to_string(),
        }),
    )
        .into_response()
}

async fn draft_agent(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<DraftAgentRequest>,
) -> impl IntoResponse {
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id.to_string(),
        Err(status) => return status.into_response(),
    };
    let llm = Arc::new(AdapterLlm {});
    let system_prompt = "You are an expert quoting AI. Given a customer inquiry, generate a JSON array of line items representing a proposal for the requested work. Each object must have: 'description' (string), 'unit_price_cents' (integer), 'quantity' (integer), 'is_optional' (boolean). Return ONLY the raw JSON array.".to_string();

    let req = ChatRequest {
        model: "default-model".to_string(),
        system: system_prompt,
        messages: vec![Message::user(payload.inquiry.clone())],
        temperature: 0.1,
        max_tokens: 1024,
        tools: vec![],
    };

    let res = match llm.chat(req).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("LLM Failed: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let json_str = res.message.content.trim();
    let json_str = json_str.strip_prefix("```json").unwrap_or(json_str);
    let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();

    let line_items: Vec<LineItemRequest> = match serde_json::from_str(json_str) {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to parse proposal model output: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let _total_amount_cents = match checked_proposal_total(&line_items) {
        Ok(total) => total,
        Err(message) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({"error": message})),
            )
                .into_response();
        }
    };
    // A model-generated draft cannot invent the owner's deposit policy.
    let _required_deposit_cents = 0;

    let proposal_id = Uuid::new_v4().to_string();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin tx: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Overwrite total and deposit to 0 and set status to NEEDS_PRICING to prevent committing fabricated pricing
    let total_amount_cents = 0;
    let required_deposit_cents = 0;

    let insert_res = sqlx::query(
        "INSERT INTO proposals (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, created_at, updated_at) VALUES ($1, $2, $3, 'NEEDS_PRICING', $4, $5, NOW(), NOW())"
    )
    .bind(&proposal_id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(total_amount_cents)
    .bind(required_deposit_cents)
    .execute(&mut *tx)
    .await;

    if let Err(e) = insert_res {
        tracing::error!("Failed to insert proposal: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in line_items {
        let item_id = Uuid::new_v4().to_string();
        let res = sqlx::query(
            "INSERT INTO proposal_line_items (id, proposal_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
        )
        .bind(&item_id)
        .bind(&proposal_id)
        .bind(&item.description)
        .bind(0_i64) // Zero out AI-fabricated pricing
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert new proposal line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit tx: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"id": proposal_id}))).into_response()
}

async fn get_proposal(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return status.into_response(),
    };
    let (proposal_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Proposal>(GET_PROPOSAL_SQL)
            .bind(&id)
            .bind(tenant_id)
            .fetch_optional(&pool),
        sqlx::query_as::<_, ProposalLineItem>(GET_LINE_ITEMS_SQL)
            .bind(&id)
            .bind(tenant_id)
            .fetch_all(&pool)
    );

    let proposal = match proposal_res {
        Ok(Some(p)) => p,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch proposal: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch proposal line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ProposalResponse {
            proposal,
            line_items,
        }),
    )
        .into_response()
}

async fn approve_proposal(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return StatusCode::FORBIDDEN.into_response();
    }
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id.to_string(),
        Err(status) => return status.into_response(),
    };
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin tx: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let proposal = match sqlx::query_as::<_, Proposal>(APPROVE_PROPOSAL_SQL)
        .bind(&id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to approve proposal: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let line_items = match sqlx::query_as::<_, ProposalLineItem>(GET_LINE_ITEMS_SQL)
        .bind(&id)
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch proposal line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let priced_items: Vec<LineItemRequest> = line_items
        .iter()
        .map(|item| LineItemRequest {
            description: item.description.clone(),
            unit_price_cents: item.unit_price_cents,
            quantity: item.quantity,
            is_optional: item.is_optional,
        })
        .collect();
    if checked_proposal_total(&priced_items).ok() != Some(proposal.total_amount_cents)
        || proposal.required_deposit_cents < 0
        || proposal.required_deposit_cents > proposal.total_amount_cents
    {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }
    let amount_usd = (proposal.total_amount_cents as f64) / 100.0;
    let db_view = crate::db::DB {
        pool: pool.clone(),
        store: crate::db::DbStore::Postgres,
    };
    let stripe_key = match crate::api::tool_integrations::stripe_key_for_tenant(&db_view,&tenant_id).await {
        Ok(key) => key,
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE,Json(serde_json::json!({"error":"A verified tenant payment connection is required; proposal remains a draft"}))).into_response(),
    };
    let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    if stripe_client.require_api_key().is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error":"Payment provider is not configured; proposal remains a draft"}))).into_response();
    }
    use sha2::{Digest, Sha256};
    let operation_id = format!(
        "proposal:{:x}",
        Sha256::digest(format!("{}:{}", tenant_id, proposal.id))
    );
    let checkout_url = match stripe_client
        .create_checkout_session_idempotent(
            crate::integrations::stripe::safe_checkout::CheckoutRequest {
                name: &format!("Proposal #{}", proposal.id),
                reference: &proposal.id,
                amount_cents: proposal.total_amount_cents,
                interval: None,
                product: Some(&proposal.id),
                currency: "usd",
                operation_id: &operation_id,
            },
        )
        .await
    {
        Ok(receipt) => receipt.url,
        Err(_) => {
            // Unknown external writes must not become a successful invoice or
            // be retried autonomously as a second payment request.
            if sqlx::query("UPDATE proposals SET status='PAYMENT_RECONCILIATION_REQUIRED',updated_at=NOW() WHERE id=$1 AND tenant_id=$2")
                .bind(&id).bind(&tenant_id).execute(&mut *tx).await.is_err() || tx.commit().await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            return (StatusCode::BAD_GATEWAY, Json(serde_json::json!({"error":"Payment setup requires reconciliation; no payment or invoice success is claimed","operation_id":operation_id}))).into_response();
        }
    };

    if !checkout_url.is_empty() {
        let _ =
            sqlx::query("UPDATE proposals SET checkout_url = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(&checkout_url)
                .bind(&proposal.id)
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await;
    }

    let invoice_id = Uuid::new_v4().to_string();
    let due_date = chrono::Utc::now() + chrono::Duration::days(30);

    let insert_invoice_res = sqlx::query(
        "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link, total_amount_cents, amount_paid_cents, payment_status, created_at, updated_at) VALUES ($1, $2, $3, $4, 'draft', $5, 'USD', $6, $7, $8, 0, 'unpaid', NOW(), NOW())"
    )
    .bind(&invoice_id)
    .bind(&proposal.tenant_id)
    .bind(&proposal.customer_id)
    .bind("Client") // simplified
    .bind(due_date)
    .bind(amount_usd)
    .bind(&checkout_url)
    .bind(i32::try_from(proposal.total_amount_cents).expect("checked proposal total"))
    .execute(&mut *tx)
    .await;

    if let Err(e) = insert_invoice_res {
        tracing::error!("Failed to auto-generate invoice: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in line_items.iter().filter(|item| !item.is_optional) {
        let item_id = Uuid::new_v4().to_string();
        let amount = (item.unit_price_cents as f64) / 100.0;
        let res = sqlx::query(
            "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())"
        )
        .bind(&item_id)
        .bind(&proposal.tenant_id)
        .bind(&invoice_id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(amount)
        .bind(amount * item.quantity as f64)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to auto-generate invoice line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit tx: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut p = proposal;
    p.checkout_url = Some(checkout_url);

    (
        StatusCode::OK,
        Json(ProposalResponse {
            proposal: p,
            line_items,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intake_preserves_inquiry_without_inventing_a_price() {
        let input: ClientIntakeRequest = serde_json::from_value(serde_json::json!({
            "inquiry":"Repair the garden gate", "customer_id":"client-1"
        }))
        .unwrap();
        assert_eq!(validate_intake(&input), Ok((0, "NEEDS_PRICING")));
        assert_eq!(input.inquiry, "Repair the garden gate");
    }

    #[test]
    fn intake_totals_are_checked_and_optional_items_are_not_charged() {
        let input: ClientIntakeRequest = serde_json::from_value(serde_json::json!({
            "inquiry":"Repair a gate", "customer_id":"client-1", "required_deposit_cents":1200,
            "line_items":[
                {"description":"Labor", "quantity":3,"unit_price_cents":1500,"is_optional":false},
                {"description":"Painting", "quantity":1,"unit_price_cents":2000,"is_optional":true}
            ]
        }))
        .unwrap();
        assert_eq!(validate_intake(&input), Ok((4500, "DRAFT")));
        let mut invalid = input;
        invalid.required_deposit_cents = 4501;
        assert!(validate_intake(&invalid).is_err());
        invalid.required_deposit_cents = 0;
        invalid.line_items[0].unit_price_cents = i64::MAX;
        assert!(validate_intake(&invalid).is_err());
        invalid.line_items[0].unit_price_cents = -1;
        assert!(validate_intake(&invalid).is_err());
    }
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn approval_requires_owner_authority_before_database_or_payment_access() {
        let response = narrative_app(Some("tenant-a"))
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/proposal-1/approve")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert!(APPROVE_PROPOSAL_SQL.contains("status = 'DRAFT'"));
        assert!(APPROVE_PROPOSAL_SQL.contains("total_amount_cents > 0"));
    }

    struct NarrativeTestLlm;

    #[async_trait::async_trait]
    impl ResearcherLlmClient for NarrativeTestLlm {
        async fn chat(
            &self,
            request: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            assert_eq!(request.max_tokens, 900);
            assert!(request.tools.is_empty());
            for section in [
                "Executive Summary",
                "Scope",
                "Milestones",
                "Investment",
                "Next Steps",
            ] {
                assert!(request.system.contains(section));
            }
            let topic = request.messages.last().unwrap().content.as_str();
            assert_eq!(topic, topic.trim());
            Ok(ChatResponse {
                message: Message::assistant(format!("Proposal for {topic}")),
                usage: Usage {
                    input_tokens: 10,
                    output_tokens: 20,
                    ..Default::default()
                },
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    struct FailingNarrativeLlm;

    #[async_trait::async_trait]
    impl ResearcherLlmClient for FailingNarrativeLlm {
        async fn chat(
            &self,
            _request: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err(std::io::Error::other("provider-secret-detail").into())
        }
    }

    fn claims(organization_id: Option<&str>) -> ::server_common::Claims {
        ::server_common::Claims {
            sub: "proposal-user".to_string(),
            exp: i64::MAX,
            iat: 0,
            organization_id: organization_id.map(str::to_string),
            username: "proposal-user".to_string(),
            email: "proposal@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "proposal-test".to_string(),
        }
    }

    #[test]
    fn proposal_access_is_scoped_to_authenticated_tenant() {
        assert_eq!(
            authenticated_tenant(&claims(Some("tenant-a"))).unwrap(),
            "tenant-a"
        );
        assert_eq!(
            authenticated_tenant(&claims(None)),
            Err(StatusCode::UNAUTHORIZED)
        );

        for sql in [GET_PROPOSAL_SQL, GET_LINE_ITEMS_SQL, APPROVE_PROPOSAL_SQL] {
            assert!(
                sql.contains("tenant_id"),
                "query must predicate authenticated tenant: {sql}"
            );
        }
        assert!(GET_LINE_ITEMS_SQL.contains("JOIN proposals"));
        assert!(APPROVE_PROPOSAL_SQL.contains("tenant_id = $2"));

        let forged_body = serde_json::from_str::<DraftAgentRequest>(
            r#"{"inquiry":"quote","customer_id":"customer-a","tenant_id":"tenant-b"}"#,
        );
        assert!(
            forged_body.is_err(),
            "draft requests must not accept a caller-supplied tenant"
        );
    }

    fn narrative_app(organization_id: Option<&str>) -> Router {
        narrative_app_with_llm(organization_id, Arc::new(NarrativeTestLlm))
    }

    fn narrative_app_with_llm(
        organization_id: Option<&str>,
        llm: Arc<dyn ResearcherLlmClient>,
    ) -> Router {
        let pool = PgPool::connect_lazy("postgres://localhost/unused").unwrap();
        router_with_narrative_llm(llm)
            .with_state(pool)
            .layer(Extension(claims(organization_id)))
    }

    #[tokio::test]
    async fn narrative_draft_returns_a_bounded_proposal_contract() {
        let response = narrative_app(Some("tenant-a"))
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/draft")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"topic":"  Bakery website  "}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            body["proposal"]
                .as_str()
                .unwrap()
                .contains("Bakery website")
        );
    }

    #[tokio::test]
    async fn narrative_draft_requires_a_non_blank_organization() {
        for organization_id in [None, Some("  ")] {
            let response = narrative_app(organization_id)
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/draft")
                        .header("content-type", "application/json")
                        .body(Body::from(r#"{"topic":"Bakery website"}"#))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }

    #[tokio::test]
    async fn narrative_draft_rejects_empty_and_overlong_topics() {
        for topic in ["  ".to_string(), "x".repeat(4_001)] {
            let response = narrative_app(Some("tenant-a"))
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/draft")
                        .header("content-type", "application/json")
                        .body(Body::from(
                            serde_json::json!({ "topic": topic }).to_string(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
    }

    #[tokio::test]
    async fn narrative_draft_maps_model_failures_to_a_private_bad_gateway() {
        let response = narrative_app_with_llm(Some("tenant-a"), Arc::new(FailingNarrativeLlm))
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/draft")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"topic":"Bakery website"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap();
        assert!(!String::from_utf8_lossy(&body).contains("provider-secret-detail"));
    }

    #[tokio::test]
    async fn test_draft_agent_route_exists() {
        let pool = match sqlx::PgPool::connect_lazy(
            "postgres://postgres:postgres@localhost:5432/postgres",
        ) {
            Ok(p) => p,
            Err(_) => return,
        };
        let app = router().with_state(pool);

        let req = Request::builder()
            .method("POST")
            .uri("/draft_agent")
            .header("Content-Type", "application/json")
            .body(Body::from(
                r#"{"inquiry": "test", "customer_id": "cust1", "tenant_id": "tenant1"}"#,
            ))
            .unwrap();

        let _res = app.oneshot(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_proposal_route_exists() {
        let pool = match sqlx::PgPool::connect_lazy(
            "postgres://postgres:postgres@localhost:5432/postgres",
        ) {
            Ok(p) => p,
            Err(_) => return,
        };
        let app = router().with_state(pool);

        let req = Request::builder()
            .method("GET")
            .uri("/123")
            .body(Body::empty())
            .unwrap();

        let _res = app.oneshot(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_approve_proposal_route_exists() {
        let pool = match sqlx::PgPool::connect_lazy(
            "postgres://postgres:postgres@localhost:5432/postgres",
        ) {
            Ok(p) => p,
            Err(_) => return,
        };
        let app = router().with_state(pool);

        let req = Request::builder()
            .method("POST")
            .uri("/123/approve")
            .body(Body::empty())
            .unwrap();

        let _res = app.oneshot(req).await.unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SocialPostProposal {
    pub id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub content: String,
    pub image_url: Option<String>,
    pub seo_alt_text: Option<String>,
    pub seo_meta_description: Option<String>,
    pub status: String,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

pub async fn list_social_post_proposals(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(organization_id) => organization_id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let proposals_res = sqlx::query_as::<_, SocialPostProposal>(
        "SELECT * FROM social_post_proposals WHERE tenant_id = $1 ORDER BY created_at_unix DESC LIMIT 50"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    match proposals_res {
        Ok(proposals) => (
            StatusCode::OK,
            Json(serde_json::json!({ "proposals": proposals })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch social post proposals: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod social_tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_social_post_proposals_route_exists() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/postgres")
            .expect("test database URL should parse");
        let app = router().with_state(pool);

        let req = Request::builder()
            .method("GET")
            .uri("/social/list?tenant_id=default")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_ne!(res.status(), StatusCode::NOT_FOUND);
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientIntakeRequest {
    pub inquiry: String,
    pub customer_id: String,
    #[serde(default)]
    pub line_items: Vec<LineItemRequest>,
    #[serde(default)]
    pub required_deposit_cents: i64,
}

fn checked_proposal_total(items: &[LineItemRequest]) -> Result<i64, &'static str> {
    if items.len() > 100 {
        return Err("At most 100 line items are supported");
    }
    items.iter().try_fold(0_i64, |total, item| {
        if item.description.trim().is_empty()
            || item.description.len() > 4000
            || item.quantity <= 0
            || item.quantity > 1_000_000
            || item.unit_price_cents < 0
        {
            return Err(
                "Line items require a description, positive bounded quantity and nonnegative price",
            );
        }
        let amount = item
            .unit_price_cents
            .checked_mul(i64::from(item.quantity))
            .ok_or("Line item amount is too large")?;
        // Optional items are not selected commitments and are excluded from totals.
        let amount = if item.is_optional { 0 } else { amount };
        total
            .checked_add(amount)
            .filter(|value| *value <= 99_999_999)
            .ok_or("Proposal total exceeds the supported amount")
    })
}

fn validate_intake(payload: &ClientIntakeRequest) -> Result<(i64, &'static str), &'static str> {
    if payload.inquiry.trim().is_empty()
        || payload.inquiry.chars().count() > 4000
        || payload.customer_id.trim().is_empty()
        || payload.customer_id.len() > 255
    {
        return Err("A bounded inquiry and customer identity are required");
    }
    let total = checked_proposal_total(&payload.line_items)?;
    if payload.required_deposit_cents < 0 || payload.required_deposit_cents > total {
        return Err("Deposit must be between zero and the agreed total");
    }
    Ok((
        total,
        if payload.line_items.is_empty() {
            "NEEDS_PRICING"
        } else {
            "DRAFT"
        },
    ))
}

pub async fn client_intake(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant) => tenant.to_owned(),
        Err(status) => return status.into_response(),
    };
    let (total_amount_cents, draft_status) = match validate_intake(&payload) {
        Ok(validated) => validated,
        Err(message) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({"error": message})),
            )
                .into_response();
        }
    };
    if !payload.line_items.is_empty()
        && !claims
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return StatusCode::FORBIDDEN.into_response();
    }
    let id = Uuid::new_v4().to_string();
    let required_deposit_cents = payload.required_deposit_cents;
    let project_scope = Some(payload.inquiry.trim().to_owned());
    // No invented stages, deadlines or scope commitments.
    let milestones = Some(serde_json::json!([]));

    let proposal = Proposal {
        id: id.clone(),
        tenant_id: tenant_id.clone(),
        customer_id: payload.customer_id.clone(),
        status: draft_status.to_string(),
        total_amount_cents,
        required_deposit_cents,
        checkout_url: None,
        project_scope: project_scope.clone(),
        milestones: milestones.clone(),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO proposals (
            id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, project_scope, milestones
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&proposal.id)
    .bind(&proposal.tenant_id)
    .bind(&proposal.customer_id)
    .bind(&proposal.status)
    .bind(proposal.total_amount_cents)
    .bind(proposal.required_deposit_cents)
    .bind(&proposal.project_scope)
    .bind(&proposal.milestones)
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Failed to insert proposal: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut saved_items = Vec::new();
    for item in payload.line_items {
        let item_id = Uuid::new_v4().to_string();
        if sqlx::query("INSERT INTO proposal_line_items (id, proposal_id, description, unit_price_cents, quantity, is_optional) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(&item_id).bind(&id).bind(&item.description).bind(item.unit_price_cents)
            .bind(item.quantity).bind(item.is_optional).execute(&mut *tx).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        saved_items.push(ProposalLineItem {
            id: item_id,
            proposal_id: id.clone(),
            description: item.description,
            unit_price_cents: item.unit_price_cents,
            quantity: item.quantity,
            is_optional: item.is_optional,
            created_at: None,
            updated_at: None,
        });
    }
    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit tx: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (
        StatusCode::OK,
        Json(ProposalResponse {
            proposal,
            line_items: saved_items,
        }),
    )
        .into_response()
}
