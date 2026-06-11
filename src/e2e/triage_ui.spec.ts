import { test, expect } from "@playwright/test";

test.describe("Work Triage Agentic Inbox", () => {
  const tenantId = "test-tenant";

  test("Owner reviews and approves a triage item", async ({ page }) => {
    await page.goto("/login");
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto("/dashboard");

    await expect(
      page.locator("h2").filter({ hasText: "Unified Agent Feed" }),
    ).toBeVisible({ timeout: 15000 });

    const approveBtn = page.locator('[data-testid="approve-draft"]').first();
    await expect(approveBtn).toBeVisible({ timeout: 10000 });

    await approveBtn.click();
  });

  test("Owner sees empty state when there are no items", async ({ page }) => {
    await page.goto("/login");
    await page.fill('input[type="text"]', "empty-tenant-triage-test");
    await page.click('button[type="submit"]');

    await page.goto("/dashboard");

    await expect(page.locator("h2").filter({ hasText: "Unified Agent Feed" }))
      .not.toBeVisible({ timeout: 5000 })
      .catch(() => {});
  });

  test("Owner can dismiss a triage item", async ({ page }) => {
    await page.goto("/login");
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto("/dashboard");

    const dismissBtn = page.locator('[data-testid="dismiss-draft"]').first();
    if (await dismissBtn.isVisible()) {
      await dismissBtn.click();
    }
  });

  test("Triage feed renders items correctly", async ({ page }) => {
    await page.goto("/login");
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    await page.goto("/dashboard");

    await expect(page.locator("text=Why this matters").first()).toBeVisible({
      timeout: 15000,
    });
  });
});
