import { test, expect } from "@playwright/test";

test.describe("POS Inventory Sync - E2E Race Condition", () => {
  test("POS terminal applies lock and prevents double booking online", async ({
    page,
  }) => {
    const tenantId = "e2e-tenant";
    const productId = "e2e-product-cake";

    // Log in as an admin or tenant
    await page.goto("/login");
    await page.getByPlaceholder("Email address").fill("admin@ohc.local");
    await page.getByPlaceholder("Password").fill("admin");
    await page.getByRole("button", { name: "Sign In" }).click();

    await expect(page.locator("h1", { hasText: "Dashboard" })).toBeVisible({
      timeout: 15000,
    });

    const response = await page.request.post("/api/v1/auth/login", {
      data: {
        email: "admin@ohc.local",
        password: "admin",
      },
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    // Simulate POS (User B) acquiring lock
    const reserveRes = await page.request.post(
      "/api/v1/payments/terminal/reserve",
      {
        data: {
          tenant_id: tenantId,
          product_id: productId,
          quantity: 1,
          ttl_seconds: 15,
        },
        headers: {
          "x-tenant-id": tenantId,
          Authorization: `Bearer ${token}`,
        },
      },
    );

    expect(reserveRes.ok()).toBe(true);
    const lockData = await reserveRes.json();
    expect(lockData.success).toBe(true);

    // Simulate Online User (User A) attempting checkout for the same item
    const reserveRes2 = await page.request.post(
      "/api/v1/payments/terminal/reserve",
      {
        data: {
          tenant_id: tenantId,
          product_id: productId,
          quantity: 1,
          ttl_seconds: 15,
        },
        headers: {
          "x-tenant-id": tenantId,
          Authorization: `Bearer ${token}`,
        },
      },
    );

    // It should fail gracefully
    const lockData2 = await reserveRes2.json();
    expect(lockData2.success).toBe(false);
    expect(lockData2.error_message).toContain("another customer");

    // POS (User B) completes checkout
    const commitRes = await page.request.post(
      "/api/v1/payments/terminal/commit",
      {
        data: {
          tenant_id: tenantId,
          product_id: productId,
          quantity: 1,
          lock_id: lockData.lock_id,
        },
        headers: {
          "x-tenant-id": tenantId,
          Authorization: `Bearer ${token}`,
        },
      },
    );

    expect(commitRes.ok()).toBe(true);
  });
});
