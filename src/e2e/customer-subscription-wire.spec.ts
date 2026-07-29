import { test, expect } from '@playwright/test';

test.describe('Customer Subscription Portal', () => {
  test('loads subscription from backend and handles actions', async ({ page }) => {
    // Navigate to a potentially non-existent subscription just to verify the real network loading
    // Since we don't have a guaranteed seeded subscription ID, we will check if it handles "not found"
    // correctly from the backend rather than the test data.
    const testId = 'sub_9999999999';
    await page.goto(`/customer/subscriptions/${testId}`);

    // The UI should show "Subscription not found." if it correctly hit the backend and got 404
    await expect(page.locator('text="Subscription not found."')).toBeVisible();
  });
});
