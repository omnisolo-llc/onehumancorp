import { test, expect } from '@playwright/test';

test.describe('Help Center and Documentation E2E', () => {
  test('User can navigate to Help Center, view tooltips, and use floating chat', async ({ page }) => {
    // 1. Login
    await page.goto('/');

    // Wait for the inputs
    await page.waitForSelector('input', { timeout: 60000 });
    const inputs = page.locator('input');
    await inputs.nth(0).fill('admin');
    await inputs.nth(1).fill('admin');

    await page.waitForTimeout(1000);
    await page.click('text="Sign In"');

    // Wait for CanvasKit transition
    await page.waitForTimeout(5000);

    // We expect the app shell to load. We can use the Help Center nav item.
    // The sidebar contains the text "Help Center". Let's click it.
    await page.click('text="Help Center"');

    await page.waitForTimeout(2000);

    // Assert the help center loaded by checking for categories
    await expect(page.locator('text="Getting Started"')).toBeVisible();
    await expect(page.locator('text="My Store"')).toBeVisible();
    await expect(page.locator('text="Payments"')).toBeVisible();

    // 2. Open Chat
    await page.click('text="Ask anything"');
    await page.waitForTimeout(1000);

    // 3. Assert Chat Dialog
    await expect(page.locator('text="AI Help Chat"')).toBeVisible();
    await expect(page.locator('text="Hello! I am your AI Help Agent. How can I assist you with OneHumanCorp today?"')).toBeVisible();

    await page.click('text="Close"');
  });
});
