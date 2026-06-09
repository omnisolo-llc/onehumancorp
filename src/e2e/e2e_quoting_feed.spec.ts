import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Quote Feed e2e', () => {
  test('approves quote from mobile feed', async ({ browser }) => {
    // 1. Visit unified agent feed
    const context = await browser.newContext({
        viewport: { width: 375, height: 812 },
    });
    const page = await adminPage({ page: await context.newPage() });

    await page.goto('/dashboard');

    // 2. See draft quote ready
    await expect(page.getByText('Draft Quote Ready: Fix leaking sink for John Doe')).toBeVisible();

    // 3. Tap approve
    await page.locator('[data-testid="approve-proposal"]').first().click();

    // 4. Assert quote is accepted
    // Wait for the decision to be processed and moving to history/activity or updated
    await expect(page.getByText('Draft Quote Ready: Fix leaking sink for John Doe')).not.toBeVisible();
  });
});
