# Multi-Currency & Instant Localized Invoicing Architecture Plan

1. **Create Migrations:**
    * Create a new SQL migration file (`src/server/db/migrations/169_multi_currency_invoicing.sql` and `src/server/migrations/169_multi_currency_invoicing.sql`).
    * Add `base_currency` (TEXT), `transaction_currency` (TEXT), and `exchange_rate` (DOUBLE PRECISION) to `orders`, `invoices`, and `payment_events`.
    * Add `global_sales_enabled` (BOOLEAN DEFAULT FALSE) to `tenants`.
    * No need to touch RLS explicitly if just adding columns.

2. **Verify Migrations:**
    * Use `ls` and `read_file` to verify the creation and correct contents of the new SQL migration files.

3. **Update Database Models:**
    * Modify `src/server/domain/repository/models.rs` to include `base_currency: Option<String>`, `transaction_currency: Option<String>`, and `exchange_rate: Option<f64>` on `Order`, `Invoice`, and `PaymentEvent`.
    * Modify `Tenant` to include `global_sales_enabled: Option<bool>`.

4. **Update Agent Logic (`src/server/orchestration/departments/finance_agent.rs`):**
    * In `FinanceAgent::handle_event` (where `event_type` is `payment.captured`), enhance the simulation logic to extract currency information (defaulting to "USD" base, and detecting cross-border if `transaction_currency` is different).
    * If `global_sales_enabled` is active, it will automatically compute an exchange rate and log the multi-currency transaction.

5. **Verify Backend Code:**
    * Run `bazel test //src/server/...` to ensure all tests pass and code compiles successfully with the new models and agent logic.
    * Use Playwright tools/browser if appropriate to ensure the backend is running.

6. **Frontend UI Implementation:**
    * Modify `src/ui/next/src/app/finance/page.tsx` to include a Premium Translucent Glass "Global Sales" toggle in a settings area or header.
    * Modify the `InvoiceMobileView` and frontend invoice card display to show `transaction_currency` if it differs from the default/base currency.
    * Verify frontend rendering by loading the page via Playwright browser tool.

7. **Pre-commit Steps:**
    * Complete pre commit steps to ensure proper testing, verification, review, and reflection are done (using `pre_commit_instructions`).

8. **Submit:**
    * Push the branch and submit the PR.
