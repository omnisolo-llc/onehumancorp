import { expect, test } from "./fixtures";

test.describe("Proactive Agent Autonomous Workflow", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test("Mobile UX and backend approval flow for proactive agent", async ({
    page,
    request,
  }) => {
    const tenantId = `tenant-${Math.random().toString(36).substring(7)}`;

    // 1. Mock DB by creating tenant directly
    await request.post('http://127.0.0.1:8081/api/onboarding/start', {
      data: {
        organization_id: tenantId,
        business_type: 'Boutique',
        company_name: 'Proactive Boutique'
      }
    });

    const productId = `prod-${Math.random().toString(36).substring(7)}`;
    // 2. Insert low inventory product to trigger worker
    await request.post('http://127.0.0.1:8081/api/v1/product', {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'admin'
      },
      data: {
        id: productId,
        name: 'Flour',
        description: 'Baking flour',
        price_cents: 500, // $5.00
        inventory_count: 5, // < 10, will trigger proactive agent worker
        item_type: 'Product'
      }
    });

    // 3. Instead of waiting 30 seconds for the worker loop to run, manually trigger insertion
    await request.post("http://127.0.0.1:8081/api/e2e/setup", {
      data: {
        query: `INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload) VALUES ('proactive-123', '${tenantId}', 'proactive', 'Flour is running low. Drafted email to supplier to restock. Send?', 'DRAFT', 'LOW', '{"title": "Low Inventory Alert", "item_id": "${productId}", "item_name": "Flour", "trigger": "low_inventory"}'::jsonb);`
      }
    });

    // Login via UI
    await page.goto('http://127.0.0.1:3000/login');
    await page.fill('input[type="email"]', `${tenantId}@example.com`);
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Make sure we are on dashboard
    await expect(page).toHaveURL(/.*\/dashboard.*/);

    // Ensure the unified feed tab is visible
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });

    // Verify the proactive agent action card is rendered
    const actionCard = page.locator("text=Low Inventory Alert").first();
    await expect(actionCard).toBeVisible();

    // Verify specific info inside the card
    await expect(page.locator("text=Flour is running low.").first()).toBeVisible();

    // Click the [Approve] button
    const approveButton = page.getByTestId("proactive-action-approve").first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify the action disappears or shows approved state
    await expect(page.locator("text=approved").first()).toBeVisible({ timeout: 10000 });

    // Verify backend mutation
    const actionResponse = await request.post("http://127.0.0.1:8081/api/e2e/setup", {
      data: {
        query: `SELECT status FROM agent_approvals WHERE id = 'proactive-123';`
      }
    });

    if (actionResponse.ok()) {
      const data = await actionResponse.json();
      expect(data.rows[0].status).toBe("APPROVED");
    }
  });
});
