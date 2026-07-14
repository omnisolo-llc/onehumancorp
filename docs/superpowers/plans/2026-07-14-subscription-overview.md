# Subscription Overview Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide the authenticated subscriptions page with one real, tenant-scoped overview response containing plans, subscribers, and fulfillment batches.

**Architecture:** The Rust subscription router owns the aggregate contract. Shared typed query helpers serve both the existing collection endpoints and the new overview endpoint; the overview runs its three independent reads concurrently. The Next route remains a bounded authenticated proxy and the React page consumes the typed Rust field names.

**Tech Stack:** Rust/Axum, SQLx/PostgreSQL, Next.js App Router, React, Vitest/Testing Library.

---

### Task 1: Tenant-scoped Rust overview endpoint

**Files:**
- Modify: `src/server/api/subscription.rs`

- [ ] **Step 1: Write failing Rust tests**

Add router tests that create isolated PostgreSQL tables for `subscription_plans`,
`products`, `subscriptions`, and `fulfillment_batches`, seed tenant A and B rows,
and call `GET /` with tenant A claims. Assert status `200` and this exact shape:

```rust
assert_eq!(body["plans"].as_array().unwrap().len(), 1);
assert_eq!(body["subscribers"].as_array().unwrap().len(), 1);
assert_eq!(body["batches"].as_array().unwrap().len(), 1);
assert_eq!(body["plans"][0]["name"], "Tenant A plan");
```

Add a missing-organization test that asserts `401` in multitenant mode.

- [ ] **Step 2: Run the tests and verify RED**

Run:

```bash
OHC_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:<port>/ohc_test \
  cargo test --lib api::subscription::tests::subscription_overview -- --nocapture
```

Expected: failure because `GET /` is not registered.

- [ ] **Step 3: Extract strict tenant authority and typed query helpers**

Add one authority function and three helpers with these interfaces:

```rust
fn subscription_tenant(claims: &server_common::Claims) -> Result<String, StatusCode>;

async fn load_plans(pool: &sqlx::PgPool, tenant: &str)
    -> Result<Vec<SubscriptionPlanResponse>, sqlx::Error>;
async fn load_subscribers(pool: &sqlx::PgPool, tenant: &str)
    -> Result<Vec<SubscriberResponse>, sqlx::Error>;
async fn load_fulfillment_batches(pool: &sqlx::PgPool, tenant: &str)
    -> Result<Vec<FulfillmentBatchResponse>, sqlx::Error>;
```

In multitenant mode, accept only a nonblank claims organization. In explicitly
single-tenant mode, return `server_common::auth_utils::get_default_tenant()`.
Move the existing SQL and row mapping into these helpers without changing their
tenant predicates.

- [ ] **Step 4: Add the aggregate response and handler**

```rust
#[derive(Serialize)]
struct SubscriptionOverviewResponse {
    plans: Vec<SubscriptionPlanResponse>,
    subscribers: Vec<SubscriberResponse>,
    batches: Vec<FulfillmentBatchResponse>,
}

async fn get_subscription_overview(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<server_common::Claims>,
) -> impl IntoResponse {
    let tenant = match subscription_tenant(&claims) {
        Ok(tenant) => tenant,
        Err(status) => return status.into_response(),
    };
    let (plans, subscribers, batches) = tokio::join!(
        load_plans(&hub.pool, &tenant),
        load_subscribers(&hub.pool, &tenant),
        load_fulfillment_batches(&hub.pool, &tenant),
    );
    match (plans, subscribers, batches) {
        (Ok(plans), Ok(subscribers), Ok(batches)) =>
            (StatusCode::OK, Json(SubscriptionOverviewResponse {
                plans, subscribers, batches,
            })).into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    }
}
```

Register `.route("/", get(get_subscription_overview))`. Refactor the three
existing GET handlers to call the same helpers so their behavior cannot drift.

- [ ] **Step 5: Run focused Rust tests**

Run the overview test and then:

```bash
OHC_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:<port>/ohc_test \
  cargo test --lib api::subscription::tests -- --nocapture
cargo check -p ohc-mono --lib
```

Expected: all subscription tests pass; compile exits zero.

### Task 2: Align the subscriptions page with the backend contract

**Files:**
- Modify: `src/ui/next/src/app/subscriptions/page.tsx`
- Create: `src/ui/next/src/app/subscriptions/page.test.tsx`

- [ ] **Step 1: Write a failing page test**

Mock `/api/subscriptions` with:

```ts
{
  plans: [{ id: "p1", name: "Coffee", description: "", amount: 2500,
            interval: "month", active: true }],
  subscribers: [{ id: "s1", customer_id: "customer-1", status: "active" }],
  batches: [{ id: "b1", fulfillment_date: "2026-07-20",
              status: "PENDING", subscriber_count: 4 }],
}
```

Assert the page renders `$25.00 / month`, `Subscribers (1)`, and `4 boxes`.
Also assert a non-OK response leaves an accessible error message instead of a
fake empty success screen.

- [ ] **Step 2: Run the page test and verify RED**

```bash
cd src/ui/next
pnpm exec vitest run src/app/subscriptions/page.test.tsx
```

Expected: price/interval and error-state assertions fail.

- [ ] **Step 3: Add local response types and correct rendering**

Define typed `SubscriptionOverview`, check `response.ok`, and render plan price
from `amount` and cadence from `interval`:

```tsx
<p className="text-sm text-gray-500">
  ${(plan.amount / 100).toFixed(2)} / {plan.interval}
</p>
```

Expose fetch failures with `role="alert"` while retaining the existing loading
and list layout.

- [ ] **Step 4: Run focused UI verification**

```bash
cd src/ui/next
pnpm exec vitest run src/app/api/subscriptions/route.test.ts \
  src/app/subscriptions/page.test.tsx
pnpm exec tsc --noEmit
```

Expected: tests and TypeScript pass. Restore only `tsconfig.tsbuildinfo` if the
compiler modifies it.

### Task 3: Verify and commit

- [ ] **Step 1: Run diff and focused cross-layer checks**

```bash
git diff --check
git status --short
```

Expected: no whitespace errors and only scoped files.

- [ ] **Step 2: Commit**

```bash
git add src/server/api/subscription.rs \
  src/ui/next/src/app/subscriptions/page.tsx \
  src/ui/next/src/app/subscriptions/page.test.tsx
git commit -m "fix(subscriptions): add authenticated overview contract"
```

