import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Quote Feed e2e', () => {
  test('approves quote from mobile feed', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // 2. See draft quote ready
    await expect(page.getByText('Fix leaking sink for John Doe')).toBeVisible({ timeout: 15000 });

    // 3. Tap approve
    // Deep link works
    await page.locator('[data-testid="review-quote-draft"]').click();

    await expect(page.locator('role=dialog')).toBeVisible();

    await expect(page.getByText('Review Quote')).toBeVisible();

    // Tap approve on the quoting page
    await page.locator('role=dialog').getByRole('button', { name: 'Approve & Send' }).click();

    // Assert quote is accepted
    await expect(page.getByText('Proposal Accepted')).toBeVisible();
  });
});
