import { test, expect } from "./fixtures";

test.describe("AI Budget UI", () => {
  test("shows Budget Alert toast when AI budget is 10 or below", async ({ page }) => {
    // Intercept the budget API request and mock the response
    await page.route("**/api/agents/approvals/budget", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ budget: 10 }),
      });
    });

    // Navigate to the team page
    await page.goto("/team");

    // Verify that the toast is displayed
    const alertBox = page.locator("text=Your agents have been busy!");
    await expect(alertBox).toBeVisible({ timeout: 10000 });
    await expect(page.locator("text=You are at 90% of your AI budget.")).toBeVisible();
    await expect(page.locator("button", { hasText: "Upgrade" })).toBeVisible();
  });
});
