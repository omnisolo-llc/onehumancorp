use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

const MIN_TRIGGER_CENTS: i64 = 50_000;
const MAX_ADVANCE_CENTS: i64 = 500_000;
const MIN_ADVANCE_CENTS: i64 = 10_000;
const DEFAULT_REPAYMENT_BPS: i32 = 1_000;
const FLAT_FEE_BPS: i32 = 1_000;

static CAPITAL_ENGINE: LazyLock<Mutex<CapitalEngine>> =
    LazyLock::new(|| Mutex::new(CapitalEngine::default()));

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LedgerEventKind {
    Sale,
    Refund,
    Booking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub tenant_id: String,
    pub amount_cents: i64,
    pub occurred_at: DateTime<Utc>,
    pub kind: LedgerEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub tenant_id: String,
    pub pre_approved_limit_cents: i64,
    pub health_score: f64,
    pub trailing_revenue_cents: i64,
    pub refund_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalOffer {
    pub id: String,
    pub tenant_id: String,
    pub trigger_event_id: String,
    pub trigger_event_type: String,
    pub advance_amount_cents: i64,
    pub flat_fee_cents: i64,
    pub repayment_percentage: f64,
    pub total_repayment_cents: i64,
    pub plain_language_terms: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapitalContractStatus {
    Active,
    Repaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalContract {
    pub id: String,
    pub tenant_id: String,
    pub advance_amount_cents: i64,
    pub flat_fee_cents: i64,
    pub repayment_percentage: f64,
    pub total_repayment_cents: i64,
    pub repaid_cents: i64,
    pub wallet_credit_cents: i64,
    pub status: CapitalContractStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitPaymentRoute {
    pub tenant_id: String,
    pub sale_id: String,
    pub sale_amount_cents: i64,
    pub merchant_cents: i64,
    pub repayment_cents: i64,
    pub remaining_repayment_cents: i64,
    pub contract_status: Option<CapitalContractStatus>,
}

#[derive(Default)]
struct CapitalEngine {
    offers: HashMap<String, CapitalOffer>,
    contracts: HashMap<String, CapitalContract>,
}

pub fn assess_tenant_limit(tenant_id: &str, events: &[LedgerEvent]) -> RiskAssessment {
    let cutoff = Utc::now() - Duration::days(90);
    let tenant_events = events
        .iter()
        .filter(|event| event.tenant_id == tenant_id && event.occurred_at >= cutoff);

    let mut revenue_cents = 0_i64;
    let mut refund_cents = 0_i64;

    for event in tenant_events {
        match event.kind {
            LedgerEventKind::Sale | LedgerEventKind::Booking => {
                revenue_cents += event.amount_cents.max(0);
            }
            LedgerEventKind::Refund => {
                refund_cents += event.amount_cents.abs();
            }
        }
    }

    let refund_rate = if revenue_cents > 0 {
        refund_cents as f64 / revenue_cents as f64
    } else {
        0.0
    };
    let health_score = (1.0 - refund_rate).clamp(0.0, 1.0);
    let limit = ((revenue_cents as f64 * 0.25 * health_score).round() as i64)
        .clamp(0, MAX_ADVANCE_CENTS);

    RiskAssessment {
        tenant_id: tenant_id.to_string(),
        pre_approved_limit_cents: limit,
        health_score,
        trailing_revenue_cents: revenue_cents,
        refund_rate,
    }
}

pub fn trigger_contextual_offer(
    tenant_id: &str,
    trigger_event_id: &str,
    trigger_event_type: &str,
    trigger_amount_cents: i64,
    events: &[LedgerEvent],
) -> Option<CapitalOffer> {
    if trigger_amount_cents < MIN_TRIGGER_CENTS {
        return None;
    }

    let assessment = assess_tenant_limit(tenant_id, events);
    if assessment.pre_approved_limit_cents < MIN_ADVANCE_CENTS {
        return None;
    }

    let advance = (trigger_amount_cents / 4)
        .clamp(MIN_ADVANCE_CENTS, assessment.pre_approved_limit_cents)
        .min(MAX_ADVANCE_CENTS);
    let flat_fee = advance * FLAT_FEE_BPS as i64 / 10_000;
    let total = advance + flat_fee;
    let offer = CapitalOffer {
        id: format!("boost_{}", uuid::Uuid::new_v4()),
        tenant_id: tenant_id.to_string(),
        trigger_event_id: trigger_event_id.to_string(),
        trigger_event_type: trigger_event_type.to_string(),
        advance_amount_cents: advance,
        flat_fee_cents: flat_fee,
        repayment_percentage: DEFAULT_REPAYMENT_BPS as f64 / 100.0,
        total_repayment_cents: total,
        plain_language_terms: format!(
            "Take ${:.2} instantly to your OHC Wallet. We'll keep 10% of future sales until ${:.2} is repaid. No hidden fees.",
            advance as f64 / 100.0,
            total as f64 / 100.0
        ),
        expires_at: Utc::now() + Duration::hours(24),
    };

    CAPITAL_ENGINE
        .lock()
        .expect("capital engine lock poisoned")
        .offers
        .insert(offer.id.clone(), offer.clone());

    Some(offer)
}

pub fn approve_offer(tenant_id: &str, offer_id: &str) -> Option<CapitalContract> {
    let mut engine = CAPITAL_ENGINE.lock().expect("capital engine lock poisoned");
    let offer = engine.offers.get(offer_id)?.clone();
    if offer.tenant_id != tenant_id || offer.expires_at < Utc::now() {
        return None;
    }

    let contract = CapitalContract {
        id: format!("contract_{}", uuid::Uuid::new_v4()),
        tenant_id: tenant_id.to_string(),
        advance_amount_cents: offer.advance_amount_cents,
        flat_fee_cents: offer.flat_fee_cents,
        repayment_percentage: offer.repayment_percentage,
        total_repayment_cents: offer.total_repayment_cents,
        repaid_cents: 0,
        wallet_credit_cents: offer.advance_amount_cents,
        status: CapitalContractStatus::Active,
    };
    engine
        .contracts
        .insert(tenant_id.to_string(), contract.clone());
    Some(contract)
}

pub fn route_sale_repayment(
    tenant_id: &str,
    sale_id: &str,
    sale_amount_cents: i64,
) -> SplitPaymentRoute {
    let mut engine = CAPITAL_ENGINE.lock().expect("capital engine lock poisoned");
    let Some(contract) = engine.contracts.get_mut(tenant_id) else {
        return SplitPaymentRoute {
            tenant_id: tenant_id.to_string(),
            sale_id: sale_id.to_string(),
            sale_amount_cents,
            merchant_cents: sale_amount_cents,
            repayment_cents: 0,
            remaining_repayment_cents: 0,
            contract_status: None,
        };
    };

    if contract.status == CapitalContractStatus::Repaid {
        return SplitPaymentRoute {
            tenant_id: tenant_id.to_string(),
            sale_id: sale_id.to_string(),
            sale_amount_cents,
            merchant_cents: sale_amount_cents,
            repayment_cents: 0,
            remaining_repayment_cents: 0,
            contract_status: Some(contract.status.clone()),
        };
    }

    let remaining = contract.total_repayment_cents - contract.repaid_cents;
    let proposed = (sale_amount_cents * DEFAULT_REPAYMENT_BPS as i64 / 10_000).max(0);
    let repayment = proposed.min(remaining);
    contract.repaid_cents += repayment;
    if contract.repaid_cents >= contract.total_repayment_cents {
        contract.status = CapitalContractStatus::Repaid;
    }

    SplitPaymentRoute {
        tenant_id: tenant_id.to_string(),
        sale_id: sale_id.to_string(),
        sale_amount_cents,
        merchant_cents: sale_amount_cents - repayment,
        repayment_cents: repayment,
        remaining_repayment_cents: (contract.total_repayment_cents - contract.repaid_cents).max(0),
        contract_status: Some(contract.status.clone()),
    }
}

pub fn active_contract(tenant_id: &str) -> Option<CapitalContract> {
    CAPITAL_ENGINE
        .lock()
        .expect("capital engine lock poisoned")
        .contracts
        .get(tenant_id)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_assessment_ignores_other_tenants() {
        let events = vec![
            LedgerEvent {
                tenant_id: "tenant-a".to_string(),
                amount_cents: 100_000,
                occurred_at: Utc::now(),
                kind: LedgerEventKind::Sale,
            },
            LedgerEvent {
                tenant_id: "tenant-b".to_string(),
                amount_cents: 1_000_000,
                occurred_at: Utc::now(),
                kind: LedgerEventKind::Sale,
            },
        ];

        let assessment = assess_tenant_limit("tenant-a", &events);
        assert_eq!(assessment.trailing_revenue_cents, 100_000);
        assert_eq!(assessment.pre_approved_limit_cents, 25_000);
    }

    #[test]
    fn split_payment_routes_until_contract_is_repaid() {
        let tenant_id = format!("tenant-{}", uuid::Uuid::new_v4());
        let events = vec![LedgerEvent {
            tenant_id: tenant_id.clone(),
            amount_cents: 200_000,
            occurred_at: Utc::now(),
            kind: LedgerEventKind::Sale,
        }];
        let offer = trigger_contextual_offer(&tenant_id, "booking-1", "booking.created", 120_000, &events)
            .expect("expected offer");
        let contract = approve_offer(&tenant_id, &offer.id).expect("expected contract");
        assert_eq!(contract.wallet_credit_cents, offer.advance_amount_cents);

        let route = route_sale_repayment(&tenant_id, "sale-1", 500_000);
        assert!(route.repayment_cents > 0);
        assert_eq!(route.merchant_cents + route.repayment_cents, 500_000);
    }
}
