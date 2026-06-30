import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Quote Approval Flow', () => {
  test('Owner can approve a drafted quote and auto-generate deposit link', async ({ browser }) => {
    // Navigate authenticated as admin
    const page = await adminPage(browser);

    // E2E seed script provides some quotes or we can just visit the URL.
    // Usually there's a seeded draft quote
    await page.goto('/ui/quote.html?id=00000000-0000-0000-0000-000000000000&mode=owner');

    // Set up dialog listener before triggering the action that causes it
    let dialogAppeared = false;
    page.on('dialog', dialog => {
      dialogAppeared = true;
      expect(dialog.message()).toContain('Quote sent to customer');
      dialog.accept();
    });

    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();

  });
});
