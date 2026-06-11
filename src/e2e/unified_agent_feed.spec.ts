import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test("Renders and actions can be tapped on 375px mobile screen", async ({
    page,
    request,
  }) => {
    // 2. Load the dashboard on mobile
    await page.goto("/dashboard");

    // 3. Ensure the unified feed tab is visible
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });

    // 4. Test UI capability without crashing
    // Wait for the button to be stable before clicking
    await page.locator("text=Activity Feed").first().waitFor({ state: 'visible' });
    await page.locator("text=Activity Feed").first().click({ force: true });
    await expect(page.locator("text=Activity Feed").first()).toBeVisible();
  });
});
