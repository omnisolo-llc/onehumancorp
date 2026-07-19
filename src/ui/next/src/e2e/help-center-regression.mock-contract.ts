import { test, expect } from "../../../../e2e/fixtures";

test.describe("Help Center E2E", () => {
  test("User navigates the Help Center correctly", async ({ page }) => {
    await page.goto("/help");

    await expect(page.locator("h1", { hasText: "In-App Help Center" })).toBeVisible();

    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await expect(searchInput).toBeVisible();
    await searchInput.fill("payments");

    // Help Chat
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();
    await expect(page.locator("h3", { hasText: "Ask anything" })).toBeVisible();
  });
});
