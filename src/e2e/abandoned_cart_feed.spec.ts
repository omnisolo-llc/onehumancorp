import { test, expect } from "./fixtures";
import { Pool } from "pg";

test.describe("Abandoned Cart Recovery Feed Test", () => {
  let pool: Pool;

  test.beforeAll(() => {
    pool = new Pool({
      connectionString:
        process.env.DATABASE_URL || "postgres://ohc:ohc@localhost:5432/ohc",
    });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test("should display abandoned cart recovery proposal in feed and approve it", async ({
    page,
    currentTenant,
  }) => {
    // Navigate to dashboard
    await page.goto("/dashboard");
    await page.waitForLoadState("networkidle");

    // Wait for the Unified Agent Feed component to appear
    await expect(
      page.getByRole("region", { name: "Unified Agent Feed" }),
    ).toBeVisible();

    // Directly insert an approval request into the DB to simulate the backend cart recovery scanner
    const approvalId = `e2e-cart-recovery-${Date.now()}`;
    const payload = JSON.stringify({
      feature_type: "cart_recovery",
      action_type: "cart_recovery.dispatch",
      checkout_session_id: "sess_123",
      customer_id: "cust_123",
      amount_cents: 8500,
      channel: "Email",
      to: "customer@example.com",
      subject: "Finish your checkout",
      body: "Hi there! We noticed you left a $85.00 checkout unfinished. Here's a special 10% discount to help you decide.",
      checkout_url: "https://app.onehumancorp.com/checkout/recover/sess_123",
      context: {
        cart_recovery: true,
        abandoned_carts_count: 1,
        potential_revenue: 85.0,
        discount_amount: 8.5,
        discount_percent: 10,
      },
    });

    await pool.query(
      `
      INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
      VALUES ($1, $2, 'marketing', 'Drafted cart recovery message for a $85.00 abandoned cart.', 'DRAFT', 'HIGH', $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
    `,
      [approvalId, currentTenant, payload],
    );

    // Reload the page to fetch the new approval
    await page.reload();
    await page.waitForLoadState("networkidle");

    // Verify the cart recovery card is displayed
    await expect(
      page.getByText(
        "Drafted cart recovery message for a $85.00 abandoned cart.",
      ),
    ).toBeVisible();
    await expect(page.getByText("Cart Value:")).toBeVisible();
    await expect(page.getByText("$85.00")).toBeVisible();
    await expect(page.getByText("Suggested Discount:")).toBeVisible();
    await expect(page.getByText("10% ($8.50)")).toBeVisible();
    await expect(
      page.getByText(
        '"Hi there! We noticed you left a $85.00 checkout unfinished.',
      ),
    ).toBeVisible();

    // Click "Approve & Send"
    await page.getByTestId("approve-cart-recovery").first().click();

    // Wait for it to disappear from the feed (it's optimistic UI or re-fetches)
    await expect(
      page.getByText(
        "Drafted cart recovery message for a $85.00 abandoned cart.",
      ),
    ).toBeHidden();

    // Verify the approval was updated in the DB and a job was queued
    const approvalRes = await pool.query(
      "SELECT status FROM agent_approvals WHERE id = $1",
      [approvalId],
    );
    expect(approvalRes.rows[0].status).toBe("APPROVED");

    const jobRes = await pool.query(
      "SELECT status, job_type, payload FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'cart_recovery' ORDER BY created_at DESC LIMIT 1",
      [currentTenant],
    );
    expect(jobRes.rows.length).toBeGreaterThan(0);
    expect(jobRes.rows[0].status).toBe("PENDING");
    expect(jobRes.rows[0].payload.checkout_session_id).toBe("sess_123");
  });
});
