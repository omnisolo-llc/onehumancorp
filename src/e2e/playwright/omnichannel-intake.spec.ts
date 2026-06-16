import { test, expect } from "@playwright/test";

test.describe("Omnichannel Intake Agent feed card", () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test("should process a webhook, triage, and allow approval on mobile feed", async ({
    page,
    request,
  }) => {
    const tenantId = "omni_test_tenant_" + Date.now();
    const customerPhone = "+15555551234";

    // Seed the database
    await request.post("/api/v1/builder/seeder/exec", {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('omni_user_id', 'omni_user@example.com', 'Omni User', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Omni Store', 'omni_user@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO customers (id, tenant_id, name, email, phone)
          VALUES ('test_cust_1', '${tenantId}', 'Test Omnichannel Customer', 'omni@example.com', '${customerPhone}')
          ON CONFLICT DO NOTHING;
        `,
      },
    });

    // 2. Post the webhook payload directly to the API
    const response = await page.request.post("/api/v1/webhooks/omnichannel", {
      data: {
        tenant_id: tenantId,
        channel: "instagram",
        sender_id: customerPhone,
        message: "Hello, what is the status of my order?",
      },
    });

    expect(response.status()).toBe(200);

    // Wait a brief moment for the background worker to process triage
    await page.waitForTimeout(3000);

    // Navigate to dashboard where feed is shown
    await page.goto(`/login?test_email=omni_user@example.com`);
    await page.evaluate((t) => localStorage.setItem("tenant", t), tenantId);
    await page.goto("/dashboard");

    // Wait for feed section to be visible
    const feedSection = page.locator(
      'section[aria-label="Unified Agent Feed"]',
    );
    await expect(feedSection).toBeVisible({ timeout: 15000 });

    // Verify mobile constraints
    const bodyBox = await page.locator("body").boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);

    // Wait for the action card to appear
    const omniCard = page.locator('[data-testid="instagram-dm-card"]').first();
    await expect(omniCard).toBeVisible({ timeout: 15000 });

    // Find and click the approve button
    const approveBtn = page
      .locator('[data-testid="approve-instagram-dm"]')
      .first();
    await approveBtn.click();

    // Verify the card is removed after action
    await expect(omniCard).not.toBeVisible({ timeout: 10000 });
  });
});
