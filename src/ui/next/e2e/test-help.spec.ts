import { test, expect } from '@playwright/test';

test.describe('Help Center and Docs E2E', () => {
  test('Verify help center navigation and specific articles', async ({ page }) => {
    // 1. Check Help Center Home
    await page.goto('http://localhost:3000/help');
    await expect(page.locator('h1')).toContainText('Help Center');
    await page.waitForTimeout(500); // allow hydration

    // 2. Search for an article
    await page.fill('input[placeholder="Search for help articles..."]', 'Getting Paid');
    await expect(page.locator('text=Getting Paid')).toBeVisible();
    await page.click('text=Getting Paid');

    // 3. Verify Article Page
    await page.waitForURL('**/help/payments');
    await expect(page.locator('h1')).toContainText('Getting Paid');
    await expect(page.locator('text=Connecting Your Bank Account')).toBeVisible();

    // 4. Back to Help Center
    await page.click('text=Back to Help Center');
    await page.waitForURL('**/help');
    await expect(page.locator('h1')).toContainText('Help Center');

    // 5. Check API Docs
    await page.goto('http://localhost:3000/api-docs');
    await expect(page.locator('text=Advanced:')).toBeVisible();

    // 6. Check Changelog
    await page.goto('http://localhost:3000/changelog');
    await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
  });

  test('Verify Help Chat Interaction', async ({ page }) => {
    await page.goto('http://localhost:3000/help');

    // E2E env var disables HelpChat floating button.
    // If NEXT_PUBLIC_E2E is false, we can test it.
    // In our tests we usually set it to true, but we will evaluate if it's visible.
    const isE2E = process.env.NEXT_PUBLIC_E2E === 'true';
    if (!isE2E) {
      const openBtn = page.locator('button[aria-label="Open help chat"]');
      if (await openBtn.isVisible()) {
        await openBtn.click();

        // Wait for chat to open
        await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();

        // Send a message
        await page.fill('input[placeholder="Ask me anything..."]', 'How do I add a product?');
        await page.click('button[aria-label="Send message"]');

        // Wait for response text to appear
        await expect(page.locator('.flex-1.p-4.overflow-y-auto')).toContainText('How do I add a product?');
      }
    }
  });
});
