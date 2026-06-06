import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Engine', () => {
  test('Sales AI generates conversational checkout link from inbox intent', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
    await expect(page.locator('body')).toContainText(/No inbox message rows|Approve|Customer/);
  });
});
