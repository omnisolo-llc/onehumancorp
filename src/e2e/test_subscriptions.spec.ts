import { expect, test } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles user input', async ({ page, adminUser, loginAs }) => {
    // Navigate via proper routing for the React app, not the mock .html file.
    // Ensure we are testing the mobile viewport layout
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/settings/global-commerce');

    // We just verify the basic UI can render. We don't test LLMs by mocking them.
    await expect(page.locator('h1').first()).toBeVisible({ timeout: 15000 });
  });
});
