import { test, expect } from "./fixtures";

test.describe("In-App Help Center", () => {
  test.beforeEach(async ({ page, loginAs, unlimitedAdminUser }) => {
    // Tests are using real network. We don't mock backend responses.
    await loginAs(page, unlimitedAdminUser);
  });

  test("should allow user to navigate to help center from dashboard", async ({
    page,
  }) => {
    await page.goto("/api/ui/dashboard.html");

    // Should see help button in the main navigation or shell
    const helpButton = page.locator("nav").locator("a", { hasText: "Help" });
    await expect(helpButton).toBeVisible();
    await helpButton.click();
    await expect(page).toHaveURL(/\/api\/ui\/help\.html/);
  });

  test("should provide help resources and allow searching", async ({
    page,
  }) => {
    await page.goto("/api/ui/help.html");

    // Help center title should be visible
    await expect(
      page.locator("h1", { hasText: "In-App Help Center" }),
    ).toBeVisible();

    // Search bar should be functional
    const searchInput = page.locator('input[placeholder*="Search"]');
    await expect(searchInput).toBeAttached();

    await searchInput.fill("payments", { force: true });

    // There should be search results
    await page.waitForTimeout(1000); // Wait for debounce
    // We expect actual results based on seeded data, not an empty state
    await expect(page.getByText("Accepting Payments").first()).toBeVisible();
  });

  test("should display contact support option", async ({ page }) => {
    await page.goto("/api/ui/help.html");

    // Should see contact options
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.fill("NotAFunnySearchWord12398", { force: true });

    await page.waitForTimeout(1000); // Wait for debounce

    await expect(
      page
        .locator("text=Contact Support")
        .or(page.locator("text=Ask AI Support Agent"))
        .or(page.locator("text=Ask anything")),
    ).toBeVisible();
  });
});
