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
    await page.locator('[data-testid="edit-quote-draft"]').click();

    await expect(page).toHaveURL(/\/quoting\?id=.*/);

    await expect(page.getByText('Quote Details')).toBeVisible();

    // Tap approve on the quoting page
    await page.getByRole('button', { name: 'Pay Deposit with Pay' }).click();

    // Assert quote is accepted
    await expect(page.getByText('Deposit Paid')).toBeVisible();
  });
});
